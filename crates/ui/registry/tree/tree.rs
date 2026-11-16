//! Tree: a virtualized hierarchy of rows with disclosure, selection, keyboard
//! navigation, typeahead and caller-loaded children.
//!
//! A desktop composition, not a shadcn port. It flattens the expanded part of
//! the hierarchy into a gpuicn `VirtualList`, so it shares that list's
//! identity, selection, scrolling and focus rules, and adds indentation, a
//! disclosure chevron, Left/Right expansion, typeahead and lazy children.
//! Colors, sizes and radii come from the theme.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt,
    rc::Rc,
    time::Duration,
};

use gpui_icons::{LucideIcon, lucide};
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement as _, Pixels, Refineable as _, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use super::theme::UiTheme;
use super::virtual_list::{
    ListItem, ListRow, ListSelectionGesture, ListSelectionMode, VirtualList, VirtualListEvent,
    VirtualListState,
};

/// How long typed characters keep accumulating into one typeahead search.
const TYPEAHEAD_RESET: Duration = Duration::from_millis(900);

/// A node's children, as far as the caller has loaded them.
#[derive(Clone, Debug, PartialEq)]
pub enum TreeChildren {
    /// A leaf: no disclosure chevron.
    Leaf,
    /// A folder whose children are known (possibly none).
    Loaded(Vec<TreeNode>),
    /// A folder whose children have not been requested yet. Expanding it emits
    /// [`TreeEvent::LoadChildren`].
    Unloaded,
    /// A folder whose children are being loaded; shows a loading row.
    Loading,
    /// Loading failed; shows the message in a row that retries when activated.
    Failed(SharedString),
}

/// One node of caller data. IDs identify the data, not its position, and must
/// be unique across the whole tree.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    /// Stable identity, unique in the whole tree.
    pub id: SharedString,
    /// The complete accessible name, even when the visible text is truncated.
    pub label: SharedString,
    /// Visible but not selectable.
    pub disabled: bool,
    /// The node's children, or [`TreeChildren::Leaf`].
    pub children: TreeChildren,
}

impl TreeNode {
    /// A leaf node.
    pub fn leaf(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            children: TreeChildren::Leaf,
        }
    }

    /// A folder with known children.
    pub fn folder(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        children: impl IntoIterator<Item = TreeNode>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            children: TreeChildren::Loaded(children.into_iter().collect()),
        }
    }

    /// A folder whose children the caller loads on first expansion.
    pub fn lazy(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            children: TreeChildren::Unloaded,
        }
    }

    /// Disabled nodes stay visible and expandable but cannot be selected.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn is_folder(&self) -> bool {
        !matches!(self.children, TreeChildren::Leaf)
    }
}

/// A rejected replacement leaves the tree unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuplicateTreeNodeId(
    /// The repeated ID.
    pub SharedString,
);

impl fmt::Display for DuplicateTreeNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "duplicate tree node ID: {}", self.0)
    }
}

impl std::error::Error for DuplicateTreeNodeId {}

/// What kind of row is being rendered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeRowKind {
    /// A node without children.
    Leaf,
    /// A node that can expand.
    Folder {
        /// Whether its children are showing.
        expanded: bool,
    },
    /// The placeholder row under a folder whose children are loading.
    Loading,
    /// The placeholder row under a folder whose children failed to load.
    Failed(SharedString),
}

/// Facts about the row being rendered. Use `id`, never `index`, as identity.
#[derive(Clone, Debug)]
pub struct TreeRow {
    /// The node's ID, or for a placeholder row, its parent folder's ID.
    pub id: SharedString,
    /// The node's label, or a placeholder's message.
    pub label: SharedString,
    /// 0 for top-level nodes.
    pub depth: usize,
    /// Leaf, folder or placeholder.
    pub kind: TreeRowKind,
    /// Visible but not selectable.
    pub disabled: bool,
    /// Part of the selection.
    pub selected: bool,
    /// The tree's active row.
    pub focused: bool,
    /// Position among visible rows, for rendering only.
    pub index: usize,
}

/// Events carry stable node IDs. Read selection from [`TreeState`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeEvent {
    /// A pointer or keyboard gesture changed the selection.
    SelectionChanged,
    /// Keyboard navigation moved focus without changing selection.
    FocusChanged,
    /// Enter or double-click on a node. Folders also toggle.
    Activate(SharedString),
    /// A folder was expanded or collapsed.
    ExpansionChanged {
        /// The folder.
        id: SharedString,
        /// Its new state.
        expanded: bool,
    },
    /// A lazy folder needs its children: load them, then call
    /// [`TreeState::set_children`]. Also sent when a failed row is retried.
    LoadChildren(SharedString),
}

#[derive(Clone)]
struct NodeInfo {
    parent: Option<SharedString>,
    depth: usize,
}

#[derive(Clone)]
struct Visible {
    id: SharedString,
    label: SharedString,
    depth: usize,
    kind: TreeRowKind,
    disabled: bool,
}

struct Data {
    nodes: Vec<TreeNode>,
    info: HashMap<SharedString, NodeInfo>,
    expanded: HashSet<SharedString>,
    visible: Vec<Visible>,
    /// Visible row position by list ID.
    positions: HashMap<ElementId, usize>,
    typeahead: String,
    typed_at: Option<web_time::Instant>,
}

/// Caller-owned retained tree state. Clones refer to the same state.
#[derive(Clone)]
pub struct TreeState {
    data: Rc<RefCell<Data>>,
    list: VirtualListState,
}

const PLACEHOLDER_SUFFIX: &str = "\u{0}placeholder";

fn list_id(id: &SharedString) -> ElementId {
    ElementId::Name(id.clone())
}

fn placeholder_id(parent: &SharedString) -> ElementId {
    ElementId::Name(format!("{parent}{PLACEHOLDER_SUFFIX}").into())
}

/// The node ID a list row stands for; a placeholder maps to its folder.
fn node_id(id: &ElementId) -> Option<SharedString> {
    let ElementId::Name(name) = id else {
        return None;
    };
    Some(
        name.strip_suffix(PLACEHOLDER_SUFFIX)
            .map_or_else(|| name.clone(), |parent| parent.to_owned().into()),
    )
}

impl TreeState {
    /// Builds state for `nodes`, with every folder collapsed unless listed in
    /// `expanded`.
    pub fn new(
        nodes: Vec<TreeNode>,
        expanded: impl IntoIterator<Item = SharedString>,
    ) -> Result<Self, DuplicateTreeNodeId> {
        let info = index(&nodes)?;
        let expanded = expanded
            .into_iter()
            .filter(|id| info.contains_key(id))
            .collect();
        let mut data = Data {
            nodes,
            info,
            expanded,
            visible: Vec::new(),
            positions: HashMap::new(),
            typeahead: String::new(),
            typed_at: None,
        };
        let items = flatten(&mut data);
        let list = VirtualListState::new(items).expect("tree IDs are unique");
        Ok(Self {
            data: Rc::new(RefCell::new(data)),
            list,
        })
    }

    /// Replaces the nodes. Expansion, selection, focus and scroll position
    /// survive for IDs that still exist.
    pub fn replace_nodes(&self, nodes: Vec<TreeNode>) -> Result<(), DuplicateTreeNodeId> {
        let info = index(&nodes)?;
        {
            let mut data = self.data.borrow_mut();
            data.expanded.retain(|id| info.contains_key(id));
            data.nodes = nodes;
            data.info = info;
        }
        self.sync();
        Ok(())
    }

    /// Sets a folder's children, for example after [`TreeEvent::LoadChildren`].
    /// Returns `false` when `id` is unknown or a new child ID already exists.
    pub fn set_children(&self, id: &SharedString, children: TreeChildren) -> bool {
        let mut nodes = self.data.borrow().nodes.clone();
        let Some(node) = find_mut(&mut nodes, id) else {
            return false;
        };
        node.children = children;
        if self.replace_nodes(nodes).is_err() {
            return false;
        }
        true
    }

    /// Selection can hold one row (default) or several.
    pub fn set_selection_mode(&self, mode: ListSelectionMode) {
        self.list.set_selection_mode(mode);
    }

    /// Selected node IDs in visible order. Collapsed descendants drop out of
    /// selection when their folder collapses.
    pub fn selected(&self) -> Vec<SharedString> {
        self.list
            .selected()
            .iter()
            .filter(|id| !is_placeholder(id))
            .filter_map(node_id)
            .collect()
    }

    /// Replaces selection; unknown, hidden or disabled IDs are ignored.
    pub fn set_selected(&self, ids: impl IntoIterator<Item = SharedString>) {
        self.list
            .set_selected(ids.into_iter().map(|id| list_id(&id)));
    }

    /// The active row's node ID.
    pub fn focused(&self) -> Option<SharedString> {
        self.list.focused().as_ref().and_then(node_id)
    }

    /// Whether a folder is expanded.
    pub fn is_expanded(&self, id: &SharedString) -> bool {
        self.data.borrow().expanded.contains(id)
    }

    /// Expands or collapses a folder. Returns the events the change causes:
    /// the expansion change and, for an unloaded folder, a load request.
    pub fn set_expanded(&self, id: &SharedString, expanded: bool) -> Vec<TreeEvent> {
        let mut events = Vec::new();
        {
            let mut data = self.data.borrow_mut();
            let Some(node) = find(&data.nodes, id) else {
                return events;
            };
            if !node.is_folder() || data.expanded.contains(id) == expanded {
                return events;
            }
            let unloaded = node.children == TreeChildren::Unloaded;
            if expanded {
                data.expanded.insert(id.clone());
            } else {
                data.expanded.remove(id);
            }
            events.push(TreeEvent::ExpansionChanged {
                id: id.clone(),
                expanded,
            });
            if expanded && unloaded {
                events.push(TreeEvent::LoadChildren(id.clone()));
            }
        }
        if expanded && events.len() == 2 {
            // Show the loading row until the caller supplies children.
            let mut nodes = self.data.borrow().nodes.clone();
            if let Some(node) = find_mut(&mut nodes, id) {
                node.children = TreeChildren::Loading;
            }
            self.data.borrow_mut().nodes = nodes;
        }
        self.sync();
        events
    }

    /// Expands every ancestor of `id` and scrolls it into view. Returns
    /// `false` when `id` is unknown.
    pub fn reveal(&self, id: &SharedString) -> bool {
        let ancestors = {
            let data = self.data.borrow();
            if !data.info.contains_key(id) {
                return false;
            }
            let mut ancestors = Vec::new();
            let mut current = data.info[id].parent.clone();
            while let Some(parent) = current {
                current = data.info[&parent].parent.clone();
                ancestors.push(parent);
            }
            ancestors
        };
        {
            let mut data = self.data.borrow_mut();
            data.expanded.extend(ancestors);
        }
        self.sync();
        self.list.reveal(&list_id(id))
    }

    /// The number of rows currently visible (expanded part of the tree).
    pub fn visible_len(&self) -> usize {
        self.list.len()
    }

    /// Rows painted in the last frame; for performance checks.
    pub fn rows_rendered(&self) -> usize {
        self.list.rows_rendered()
    }

    /// The node's depth, or `None` when unknown.
    pub fn depth(&self, id: &SharedString) -> Option<usize> {
        self.data.borrow().info.get(id).map(|info| info.depth)
    }

    fn sync(&self) {
        let items = flatten(&mut self.data.borrow_mut());
        self.list.replace_items(items).expect("tree IDs are unique");
    }

    fn row(&self, row: &ListRow) -> Option<TreeRow> {
        let data = self.data.borrow();
        let position = *data.positions.get(&row.item.id)?;
        let visible = data.visible.get(position)?;
        Some(TreeRow {
            id: visible.id.clone(),
            label: visible.label.clone(),
            depth: visible.depth,
            kind: visible.kind.clone(),
            disabled: visible.disabled,
            selected: row.selected,
            focused: row.focused,
            index: row.index,
        })
    }

    fn visible_at(&self, position: usize) -> Option<Visible> {
        self.data.borrow().visible.get(position).cloned()
    }

    fn focused_position(&self) -> Option<usize> {
        let focused = self.list.focused()?;
        self.data.borrow().positions.get(&focused).copied()
    }

    fn select_row(&self, id: &SharedString) -> bool {
        self.list
            .select(&list_id(id), ListSelectionGesture::Replace)
    }

    /// Handles the keys the list itself does not: Left, Right and typeahead.
    fn key(
        &self,
        key: &str,
        modifiers: gpui_kit::Modifiers,
        typed: Option<&str>,
    ) -> Vec<TreeEvent> {
        if modifiers.platform || modifiers.control || modifiers.alt {
            return Vec::new();
        }
        let Some(position) = self.focused_position() else {
            return Vec::new();
        };
        let Some(row) = self.visible_at(position) else {
            return Vec::new();
        };
        match key {
            "right" => match row.kind {
                TreeRowKind::Folder { expanded: false } => self.set_expanded(&row.id, true),
                TreeRowKind::Folder { expanded: true } => match self.visible_at(position + 1) {
                    Some(child)
                        if child.depth > row.depth
                            && !child.disabled
                            && matches!(
                                child.kind,
                                TreeRowKind::Leaf | TreeRowKind::Folder { .. }
                            ) =>
                    {
                        self.select_row(&child.id);
                        vec![TreeEvent::SelectionChanged]
                    }
                    _ => Vec::new(),
                },
                _ => Vec::new(),
            },
            "left" => {
                if let TreeRowKind::Folder { expanded: true } = row.kind {
                    return self.set_expanded(&row.id, false);
                }
                let parent = self
                    .data
                    .borrow()
                    .info
                    .get(&row.id)
                    .and_then(|i| i.parent.clone());
                match parent {
                    Some(parent) if self.select_row(&parent) => vec![TreeEvent::SelectionChanged],
                    _ => Vec::new(),
                }
            }
            _ => typed.map_or_else(Vec::new, |text| self.typeahead(text, position)),
        }
    }

    fn typeahead(&self, typed: &str, position: usize) -> Vec<TreeEvent> {
        if typed.chars().any(|c| c.is_control()) || typed.is_empty() {
            return Vec::new();
        }
        let now = web_time::Instant::now();
        let (query, start) = {
            let mut data = self.data.borrow_mut();
            let fresh = data
                .typed_at
                .is_none_or(|at| now.duration_since(at) > TYPEAHEAD_RESET);
            if fresh {
                data.typeahead.clear();
            }
            data.typeahead.push_str(&typed.to_lowercase());
            data.typed_at = Some(now);
            // A new search starts after the focused row; extending one keeps
            // the current match if it still fits.
            (
                data.typeahead.clone(),
                if fresh { position + 1 } else { position },
            )
        };
        let data = self.data.borrow();
        let count = data.visible.len();
        let found = (0..count)
            .map(|offset| (start + offset) % count)
            .find(|&index| {
                let row = &data.visible[index];
                !row.disabled
                    && matches!(row.kind, TreeRowKind::Leaf | TreeRowKind::Folder { .. })
                    && row.label.to_lowercase().starts_with(&query)
            })
            .map(|index| data.visible[index].id.clone());
        drop(data);
        match found {
            Some(id) if self.select_row(&id) => vec![TreeEvent::SelectionChanged],
            _ => Vec::new(),
        }
    }
}

fn is_placeholder(id: &ElementId) -> bool {
    matches!(id, ElementId::Name(name) if name.ends_with(PLACEHOLDER_SUFFIX))
}

fn index(nodes: &[TreeNode]) -> Result<HashMap<SharedString, NodeInfo>, DuplicateTreeNodeId> {
    fn walk(
        nodes: &[TreeNode],
        parent: Option<&SharedString>,
        depth: usize,
        info: &mut HashMap<SharedString, NodeInfo>,
    ) -> Result<(), DuplicateTreeNodeId> {
        for node in nodes {
            if info
                .insert(
                    node.id.clone(),
                    NodeInfo {
                        parent: parent.cloned(),
                        depth,
                    },
                )
                .is_some()
                || node.id.ends_with(PLACEHOLDER_SUFFIX)
            {
                return Err(DuplicateTreeNodeId(node.id.clone()));
            }
            if let TreeChildren::Loaded(children) = &node.children {
                walk(children, Some(&node.id), depth + 1, info)?;
            }
        }
        Ok(())
    }
    let mut info = HashMap::new();
    walk(nodes, None, 0, &mut info)?;
    Ok(info)
}

fn find<'a>(nodes: &'a [TreeNode], id: &SharedString) -> Option<&'a TreeNode> {
    nodes.iter().find_map(|node| {
        if &node.id == id {
            return Some(node);
        }
        match &node.children {
            TreeChildren::Loaded(children) => find(children, id),
            _ => None,
        }
    })
}

fn find_mut<'a>(nodes: &'a mut [TreeNode], id: &SharedString) -> Option<&'a mut TreeNode> {
    for node in nodes {
        if &node.id == id {
            return Some(node);
        }
        if let TreeChildren::Loaded(children) = &mut node.children
            && let Some(found) = find_mut(children, id)
        {
            return Some(found);
        }
    }
    None
}

/// Rebuilds the visible rows and returns them as list items.
fn flatten(data: &mut Data) -> Vec<ListItem> {
    fn walk(
        nodes: &[TreeNode],
        depth: usize,
        expanded: &HashSet<SharedString>,
        out: &mut Vec<Visible>,
    ) {
        for node in nodes {
            let open = expanded.contains(&node.id);
            out.push(Visible {
                id: node.id.clone(),
                label: node.label.clone(),
                depth,
                kind: if node.is_folder() {
                    TreeRowKind::Folder { expanded: open }
                } else {
                    TreeRowKind::Leaf
                },
                disabled: node.disabled,
            });
            if !open {
                continue;
            }
            match &node.children {
                TreeChildren::Loaded(children) => walk(children, depth + 1, expanded, out),
                TreeChildren::Loading | TreeChildren::Unloaded => out.push(Visible {
                    id: node.id.clone(),
                    label: "Loading…".into(),
                    depth: depth + 1,
                    kind: TreeRowKind::Loading,
                    disabled: true,
                }),
                TreeChildren::Failed(message) => out.push(Visible {
                    id: node.id.clone(),
                    label: message.clone(),
                    depth: depth + 1,
                    kind: TreeRowKind::Failed(message.clone()),
                    disabled: false,
                }),
                TreeChildren::Leaf => {}
            }
        }
    }
    let mut visible = Vec::new();
    walk(&data.nodes, 0, &data.expanded, &mut visible);
    data.positions = visible
        .iter()
        .enumerate()
        .map(|(position, row)| (row_list_id(row), position))
        .collect();
    let items = visible
        .iter()
        .map(|row| ListItem::new(row_list_id(row), row.label.clone()).disabled(row.disabled))
        .collect();
    data.visible = visible;
    items
}

fn row_list_id(row: &Visible) -> ElementId {
    match row.kind {
        TreeRowKind::Loading | TreeRowKind::Failed(_) => placeholder_id(&row.id),
        _ => list_id(&row.id),
    }
}

type Renderer = Rc<dyn Fn(&TreeRow, &mut Window, &mut App) -> AnyElement>;
type EventHandler = Rc<dyn Fn(&TreeEvent, &mut Window, &mut App)>;

/// Renders a [`TreeState`]. The caller draws each row's content (icon, label,
/// trailing status); the tree adds indentation and the disclosure chevron.
/// Arrow keys move, Left and Right collapse, expand and move between parent
/// and child, typing jumps to a matching label, and Enter or double-click
/// activates (folders also toggle).
#[derive(IntoElement)]
pub struct Tree {
    id: ElementId,
    label: SharedString,
    state: TreeState,
    render_row: Renderer,
    on_event: Option<EventHandler>,
    row_height: Option<Pixels>,
    indent: Option<Pixels>,
    empty: Option<AnyElement>,
    style: gpui_kit::StyleRefinement,
}

impl Tree {
    /// `render_row` draws one row's content; it is called only for visible rows.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        state: TreeState,
        render_row: impl Fn(&TreeRow, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state,
            render_row: Rc::new(render_row),
            on_event: None,
            row_height: None,
            indent: None,
            empty: None,
            style: Default::default(),
        }
    }

    /// Fixed row height; defaults to the Virtual List's.
    pub fn row_height(mut self, height: Pixels) -> Self {
        self.row_height = Some(height);
        self
    }

    /// Indentation per level; defaults to four spacing units.
    pub fn indent(mut self, indent: Pixels) -> Self {
        self.indent = Some(indent);
        self
    }

    /// Selection, focus, activation, expansion and load requests. State is
    /// updated before the callback runs.
    pub fn on_event(
        mut self,
        handler: impl Fn(&TreeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_event = Some(Rc::new(handler));
        self
    }

    /// Content shown when the tree has no nodes.
    pub fn empty(mut self, content: impl IntoElement) -> Self {
        self.empty = Some(content.into_any_element());
        self
    }
}

impl Styled for Tree {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

fn emit(handler: Option<&EventHandler>, events: &[TreeEvent], window: &mut Window, cx: &mut App) {
    if let Some(handler) = handler {
        for event in events {
            handler(event, window, cx);
        }
    }
}

impl RenderOnce for Tree {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let indent = self.indent.unwrap_or(theme.spacing.unit * 4_f32);
        let chevron = theme.spacing.unit * 4_f32;
        let view = window.current_view();

        let state = self.state.clone();
        let render_row = self.render_row.clone();
        let on_event = self.on_event.clone();
        let row_theme = theme.clone();
        let row_state = state.clone();
        let row_events = on_event.clone();
        let mut list = VirtualList::new(
            self.id.clone(),
            self.label,
            state.list.clone(),
            move |row, window, cx| {
                let Some(tree_row) = row_state.row(row) else {
                    return div().into_any_element();
                };
                let depth = tree_row.depth;
                let folder = match tree_row.kind {
                    TreeRowKind::Folder { expanded } => Some(expanded),
                    _ => None,
                };
                let toggle_state = row_state.clone();
                let toggle_events = row_events.clone();
                let toggle_id = tree_row.id.clone();
                let content = render_row(&tree_row, window, cx);
                div()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .pl(indent * depth as f32)
                    .child(
                        div()
                            .id(ElementId::NamedChild(
                                std::sync::Arc::new(list_id(&tree_row.id)),
                                "disclosure".into(),
                            ))
                            .flex_shrink_0()
                            .size(chevron)
                            .flex()
                            .items_center()
                            .justify_center()
                            .when_some(folder, |el, expanded| {
                                el.child(
                                    lucide(if expanded {
                                        LucideIcon::ChevronDown
                                    } else {
                                        LucideIcon::ChevronRight
                                    })
                                    .size(chevron * 0.75)
                                    .text_color(row_theme.colors.muted_foreground),
                                )
                                .on_mouse_down(
                                    MouseButton::Left,
                                    move |_, window, cx| {
                                        // The chevron toggles without changing selection.
                                        let events =
                                            toggle_state.set_expanded(&toggle_id, !expanded);
                                        cx.stop_propagation();
                                        window.refresh();
                                        emit(toggle_events.as_ref(), &events, window, cx);
                                    },
                                )
                            }),
                    )
                    .child(div().flex_1().min_w_0().child(content))
                    .into_any_element()
            },
        );
        if let Some(height) = self.row_height {
            list = list.row_height(height);
        }
        if let Some(empty) = self.empty {
            list = list.empty(empty);
        }
        let list_state = state.clone();
        let list_events = on_event.clone();
        list = list.on_event(move |event, window, cx| {
            let events = match event {
                VirtualListEvent::SelectionChanged => vec![TreeEvent::SelectionChanged],
                VirtualListEvent::FocusChanged => vec![TreeEvent::FocusChanged],
                VirtualListEvent::Activate(id) => {
                    let Some(node) = node_id(id) else { return };
                    if is_placeholder(id) {
                        // A failed row retries; a loading row is disabled.
                        vec![TreeEvent::LoadChildren(node)]
                    } else {
                        let mut events = vec![TreeEvent::Activate(node.clone())];
                        let expanded = list_state.is_expanded(&node);
                        events.extend(list_state.set_expanded(&node, !expanded));
                        window.refresh();
                        events
                    }
                }
            };
            emit(list_events.as_ref(), &events, window, cx);
        });

        let key_state = state;
        let key_events = on_event;
        let mut root = div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .capture_key_down(move |event, window, cx| {
                let keystroke = &event.keystroke;
                let typed = keystroke
                    .key_char
                    .as_deref()
                    .filter(|_| keystroke.key.chars().count() == 1);
                let events = key_state.key(&keystroke.key, keystroke.modifiers, typed);
                if !events.is_empty() || matches!(keystroke.key.as_str(), "left" | "right") {
                    cx.notify(view);
                    window.prevent_default();
                    cx.stop_propagation();
                }
                emit(key_events.as_ref(), &events, window, cx);
            })
            .child(list);
        root.style().refine(&self.style);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{Context, Render, TestAppContext, VisualTestContext};

    fn sample() -> Vec<TreeNode> {
        vec![
            TreeNode::folder(
                "src",
                "src",
                [
                    TreeNode::leaf("src/main.rs", "main.rs"),
                    TreeNode::folder(
                        "src/ui",
                        "ui",
                        [TreeNode::leaf("src/ui/tree.rs", "tree.rs")],
                    ),
                    TreeNode::leaf("src/lib.rs", "lib.rs").disabled(true),
                ],
            ),
            TreeNode::lazy("vendor", "vendor"),
            TreeNode::leaf("README.md", "README.md"),
        ]
    }

    fn labels(state: &TreeState) -> Vec<String> {
        state
            .data
            .borrow()
            .visible
            .iter()
            .map(|row| format!("{}{}", "  ".repeat(row.depth), row.label))
            .collect()
    }

    #[test]
    fn rejects_duplicate_ids() {
        let nodes = vec![
            TreeNode::leaf("a", "a"),
            TreeNode::folder("b", "b", [TreeNode::leaf("a", "again")]),
        ];
        assert_eq!(
            TreeState::new(nodes, []).err(),
            Some(DuplicateTreeNodeId("a".into()))
        );
    }

    #[test]
    fn expanding_and_collapsing_changes_visible_rows() {
        let state = TreeState::new(sample(), []).unwrap();
        assert_eq!(labels(&state), ["src", "vendor", "README.md"]);
        let events = state.set_expanded(&"src".into(), true);
        assert_eq!(
            events,
            [TreeEvent::ExpansionChanged {
                id: "src".into(),
                expanded: true
            }]
        );
        assert_eq!(
            labels(&state),
            [
                "src",
                "  main.rs",
                "  ui",
                "  lib.rs",
                "vendor",
                "README.md"
            ]
        );
        assert!(
            state.set_expanded(&"src".into(), true).is_empty(),
            "no change, no event"
        );
        assert!(
            state.set_expanded(&"README.md".into(), true).is_empty(),
            "leaves do not expand"
        );
        state.set_expanded(&"src".into(), false);
        assert_eq!(state.visible_len(), 3);
    }

    #[test]
    fn lazy_folders_request_children_and_show_loading_then_failure() {
        let state = TreeState::new(sample(), []).unwrap();
        let events = state.set_expanded(&"vendor".into(), true);
        assert_eq!(events[1], TreeEvent::LoadChildren("vendor".into()));
        assert_eq!(labels(&state)[2], "  Loading…");

        assert!(state.set_children(
            &"vendor".into(),
            TreeChildren::Failed("Network is offline".into())
        ));
        assert_eq!(labels(&state)[2], "  Network is offline");

        assert!(state.set_children(
            &"vendor".into(),
            TreeChildren::Loaded(vec![TreeNode::leaf("vendor/a", "a")]),
        ));
        assert_eq!(labels(&state), ["src", "vendor", "  a", "README.md"]);
        assert!(!state.set_children(
            &"vendor".into(),
            TreeChildren::Loaded(vec![TreeNode::leaf("src", "clash")]),
        ));
    }

    #[test]
    fn reveal_expands_ancestors() {
        let state = TreeState::new(sample(), []).unwrap();
        assert!(state.reveal(&"src/ui/tree.rs".into()));
        assert!(state.is_expanded(&"src".into()) && state.is_expanded(&"src/ui".into()));
        assert!(labels(&state).contains(&"    tree.rs".to_owned()));
        assert!(!state.reveal(&"missing".into()));
        assert_eq!(state.depth(&"src/ui/tree.rs".into()), Some(2));
    }

    #[test]
    fn refresh_keeps_expansion_and_selection_for_surviving_ids() {
        let state = TreeState::new(sample(), ["src".into()]).unwrap();
        state.set_selected(["src/main.rs".into()]);
        let mut nodes = sample();
        nodes.insert(0, TreeNode::leaf(".gitignore", ".gitignore"));
        if let TreeChildren::Loaded(children) = &mut nodes[1].children {
            children.retain(|n| n.id != "src/ui");
        }
        state.replace_nodes(nodes).unwrap();
        assert!(state.is_expanded(&"src".into()));
        assert_eq!(state.selected(), [SharedString::from("src/main.rs")]);
        assert_eq!(
            labels(&state),
            [
                ".gitignore",
                "src",
                "  main.rs",
                "  lib.rs",
                "vendor",
                "README.md"
            ]
        );
    }

    #[test]
    fn left_right_and_typeahead() {
        let no = gpui_kit::Modifiers::default();
        let state = TreeState::new(sample(), []).unwrap();
        assert!(state.select_row(&"src".into()));
        assert_eq!(state.focused().as_deref(), Some("src"));

        // Right expands, then moves to the first child.
        state.key("right", no, None);
        assert!(state.is_expanded(&"src".into()));
        state.key("right", no, None);
        assert_eq!(state.focused().as_deref(), Some("src/main.rs"));
        // Left from a leaf moves to its parent, then collapses it.
        state.key("left", no, None);
        assert_eq!(state.focused().as_deref(), Some("src"));
        state.key("left", no, None);
        assert!(!state.is_expanded(&"src".into()));

        // Typing jumps to the next matching label, skipping disabled rows.
        state.key("r", no, Some("r"));
        assert_eq!(state.focused().as_deref(), Some("README.md"));
        state.key("v", gpui_kit::Modifiers::default(), Some("v"));
        // Within the reset window "rv" matches nothing, so focus stays.
        assert_eq!(state.focused().as_deref(), Some("README.md"));
    }

    struct Probe(TreeState);

    impl Render for Probe {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size(gpui_kit::px(400.)).child(Tree::new(
                "probe",
                "Files",
                self.0.clone(),
                |row, _, _| div().child(row.label.clone()).into_any_element(),
            ))
        }
    }

    #[gpui_kit::test]
    fn draws_only_visible_rows_in_both_themes(cx: &mut TestAppContext) {
        cx.update(super::super::theme::init);
        let nodes = (0..5_000)
            .map(|i| {
                TreeNode::folder(
                    format!("d{i}"),
                    format!("dir {i}"),
                    [TreeNode::leaf(format!("f{i}"), "file")],
                )
            })
            .collect();
        let state = TreeState::new(
            nodes,
            (0..5_000).map(|i| SharedString::from(format!("d{i}"))),
        )
        .unwrap();
        assert_eq!(state.visible_len(), 10_000);
        let probe = state.clone();
        let window = cx.add_window(move |_, _| Probe(probe));
        let mut cx = VisualTestContext::from_window(window.into(), cx);
        for mode in [
            super::super::theme::ThemeMode::Light,
            super::super::theme::ThemeMode::Dark,
        ] {
            cx.update(|_, cx| UiTheme::switch(cx, mode));
            cx.update(|window, cx| window.draw(cx).clear(cx));
            assert!(state.rows_rendered() > 0 && state.rows_rendered() < 100);
        }
    }
}

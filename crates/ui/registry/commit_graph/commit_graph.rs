//! Commit Graph: a Git history list whose rows carry a lane graph column and
//! ref labels, drawn from a layout the caller computes.
//!
//! A desktop composition, not a shadcn port. Rows are a gpuicn `VirtualList`,
//! so selection, keyboard navigation, scrolling, focus and context menus follow
//! that list. Each visible row paints its own slice of the graph: the edges
//! entering from the row above, the node, and the edges leaving for the row
//! below. Only visible rows are painted, so the graph stays aligned through
//! scrolling, resizing and refreshes. Lane colors come from the theme's chart
//! palette, and node kinds differ by shape, never by color alone.

use std::{cell::RefCell, rc::Rc};

use gpui_icons::LucideIcon;
use gpui_kit::{
    AnyElement, App, BorderStyle, Bounds, ElementId, Hsla, IntoElement, ParentElement as _,
    PathBuilder, Pixels, Point, RenderOnce, Rgba, SharedString, Styled, Window, canvas, div, point,
    px, quad, size,
};

use super::badge::{Badge, BadgeVariant};
use super::theme::UiTheme;
use super::virtual_list::{
    DuplicateListItemId, ListItem, ListRow, ListSelectionMode, VirtualList, VirtualListEvent,
    VirtualListState,
};

/// Lanes drawn before the column clips; more lanes show a clipping marker.
const DEFAULT_MAX_LANES: usize = 12;

/// The shape of a row's node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GraphNodeKind {
    /// A filled circle: an ordinary commit.
    #[default]
    Commit,
    /// A ring around a dot: a commit with more than one parent.
    Merge,
    /// A filled square: a stash or another non-branch entry.
    Stash,
    /// A hollow circle: uncommitted changes above the head commit.
    WorkingTree,
}

/// One edge segment inside a row, between two lanes.
///
/// An incoming edge runs from `from` at the row's top edge to `to` at the
/// node's height; an outgoing edge runs from `from` at the node's height to
/// `to` at the row's bottom edge. A lane passing through a row is an incoming
/// and an outgoing edge on the same lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GraphEdge {
    /// The lane the segment starts in.
    pub from: usize,
    /// The lane the segment ends in.
    pub to: usize,
    /// Index into the lane palette; it wraps around.
    pub color: usize,
}

impl GraphEdge {
    /// An edge from lane `from` to lane `to`, colored like `color`.
    pub fn new(from: usize, to: usize, color: usize) -> Self {
        Self { from, to, color }
    }

    /// A straight edge that stays in `lane`.
    pub fn straight(lane: usize, color: usize) -> Self {
        Self::new(lane, lane, color)
    }
}

/// What a ref label names, which picks its icon and treatment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphLabelKind {
    /// The checked-out branch, or a detached HEAD.
    Head,
    /// A local branch.
    Branch,
    /// A remote-tracking branch.
    Remote,
    /// A tag.
    Tag,
    /// A stash entry.
    Stash,
}

/// A ref name shown before the row's content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphLabel {
    /// The text, such as `main` or `origin/main`.
    pub text: SharedString,
    /// What the label names.
    pub kind: GraphLabelKind,
}

impl GraphLabel {
    /// A label of `kind` showing `text`.
    pub fn new(kind: GraphLabelKind, text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            kind,
        }
    }
}

/// One history row: its identity, node and the edges through it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphRow {
    /// Stable identity, such as the commit ID.
    pub id: ElementId,
    /// The complete accessible name, such as the commit summary and author.
    pub label: SharedString,
    /// The lane the node sits in.
    pub lane: usize,
    /// The node's shape.
    pub kind: GraphNodeKind,
    /// Index into the lane palette for the node.
    pub color: usize,
    /// Segments from the row's top edge to the node's height.
    pub incoming: Vec<GraphEdge>,
    /// Segments from the node's height to the row's bottom edge.
    pub outgoing: Vec<GraphEdge>,
    /// Ref labels shown before the row content.
    pub labels: Vec<GraphLabel>,
    /// Whether selection and activation are unavailable.
    pub disabled: bool,
}

impl GraphRow {
    /// A commit row with its node in `lane` and no edges or labels yet.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, lane: usize) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            lane,
            kind: GraphNodeKind::Commit,
            color: lane,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            labels: Vec::new(),
            disabled: false,
        }
    }

    /// The node's shape.
    #[must_use]
    pub fn kind(mut self, kind: GraphNodeKind) -> Self {
        self.kind = kind;
        self
    }

    /// The node's palette index; defaults to its lane.
    #[must_use]
    pub fn color(mut self, color: usize) -> Self {
        self.color = color;
        self
    }

    /// Adds a segment from the top edge to the node's height.
    #[must_use]
    pub fn incoming(mut self, edge: GraphEdge) -> Self {
        self.incoming.push(edge);
        self
    }

    /// Adds a segment from the node's height to the bottom edge.
    #[must_use]
    pub fn outgoing(mut self, edge: GraphEdge) -> Self {
        self.outgoing.push(edge);
        self
    }

    /// Adds a lane that passes straight through the row.
    #[must_use]
    pub fn through(self, lane: usize, color: usize) -> Self {
        self.incoming(GraphEdge::straight(lane, color))
            .outgoing(GraphEdge::straight(lane, color))
    }

    /// Adds a ref label.
    #[must_use]
    pub fn label(mut self, label: GraphLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Disabled rows stay visible but cannot be selected or activated.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The widest lane this row draws, plus one.
    fn lane_count(&self) -> usize {
        self.incoming
            .iter()
            .chain(&self.outgoing)
            .flat_map(|edge| [edge.from, edge.to])
            .chain([self.lane])
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }
}

/// One segment of a row's geometry in lane units: x is a lane center, y runs
/// from 0 (top edge) through 0.5 (node) to 1 (bottom edge).
#[derive(Clone, Copy, Debug, PartialEq)]
#[doc(hidden)]
pub struct Segment {
    pub from: (f32, f32),
    pub to: (f32, f32),
    pub color: usize,
}

/// A row's drawable geometry after clamping to the visible lanes.
#[derive(Clone, Debug, PartialEq)]
#[doc(hidden)]
pub struct RowGeometry {
    pub node_lane: f32,
    pub segments: Vec<Segment>,
    /// Whether anything was drawn at the clipped edge instead of its lane.
    pub clipped: bool,
}

/// Clamps every lane to `max_lanes` columns. Lanes past the limit are drawn
/// at the last column and mark the row as clipped; nothing panics.
#[doc(hidden)]
pub fn geometry(row: &GraphRow, max_lanes: usize) -> RowGeometry {
    let last = max_lanes.max(1) - 1;
    let mut clipped = false;
    let mut lane = |lane: usize| {
        if lane > last {
            clipped = true;
        }
        lane.min(last) as f32
    };
    let node_lane = lane(row.lane);
    let mut segments = Vec::with_capacity(row.incoming.len() + row.outgoing.len());
    for edge in &row.incoming {
        segments.push(Segment {
            from: (lane(edge.from), 0.),
            to: (lane(edge.to), 0.5),
            color: edge.color,
        });
    }
    for edge in &row.outgoing {
        segments.push(Segment {
            from: (lane(edge.from), 0.5),
            to: (lane(edge.to), 1.),
            color: edge.color,
        });
    }
    RowGeometry {
        node_lane,
        segments,
        clipped,
    }
}

struct Data {
    rows: Vec<GraphRow>,
    lanes: usize,
}

/// Caller-owned retained graph state. Clones refer to the same state.
#[derive(Clone)]
pub struct CommitGraphState {
    list: VirtualListState,
    data: Rc<RefCell<Data>>,
}

impl CommitGraphState {
    /// Builds the state from laid-out rows, newest first.
    pub fn new(rows: Vec<GraphRow>) -> Result<Self, DuplicateListItemId> {
        let state = Self {
            list: VirtualListState::new(Vec::new())?,
            data: Rc::new(RefCell::new(Data {
                rows: Vec::new(),
                lanes: 1,
            })),
        };
        state.replace_rows(rows)?;
        Ok(state)
    }

    /// Replaces every row, for a refresh or a page of older history. Rows
    /// that keep their ID keep selection and focus, and the scroll position
    /// stays anchored like `VirtualListState::replace_items`. A duplicate ID
    /// leaves the state unchanged.
    pub fn replace_rows(&self, rows: Vec<GraphRow>) -> Result<(), DuplicateListItemId> {
        self.list.replace_items(
            rows.iter()
                .map(|row| ListItem::new(row.id.clone(), row.label.clone()).disabled(row.disabled))
                .collect(),
        )?;
        let lanes = rows.iter().map(GraphRow::lane_count).max().unwrap_or(1);
        *self.data.borrow_mut() = Data { rows, lanes };
        Ok(())
    }

    /// Adds older rows after the last one, for the next page of history.
    /// This is O(appended), so paging through a long history stays fast;
    /// selection, focus and the scroll position are untouched. A duplicate
    /// ID leaves the state unchanged.
    pub fn append_rows(&self, rows: Vec<GraphRow>) -> Result<(), DuplicateListItemId> {
        self.list.append_items(
            rows.iter()
                .map(|row| ListItem::new(row.id.clone(), row.label.clone()).disabled(row.disabled))
                .collect(),
        )?;
        let mut data = self.data.borrow_mut();
        let lanes = rows.iter().map(GraphRow::lane_count).max().unwrap_or(1);
        data.lanes = data.lanes.max(lanes);
        data.rows.extend(rows);
        Ok(())
    }

    /// The number of rows.
    pub fn len(&self) -> usize {
        self.data.borrow().rows.len()
    }

    /// Whether there are no rows.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The row at `index`, in display order.
    pub fn row(&self, index: usize) -> Option<GraphRow> {
        self.data.borrow().rows.get(index).cloned()
    }

    /// The widest lane count of any row; the column is sized from it.
    pub fn lanes(&self) -> usize {
        self.data.borrow().lanes
    }

    /// Single selection (the default) or multiple, for cherry-picking ranges.
    pub fn set_selection_mode(&self, mode: ListSelectionMode) {
        self.list.set_selection_mode(mode);
    }

    /// Selected row IDs, in display order.
    pub fn selected(&self) -> Vec<ElementId> {
        self.list.selected()
    }

    /// Replaces the selection; unknown IDs are ignored.
    pub fn set_selected(&self, ids: impl IntoIterator<Item = ElementId>) {
        self.list.set_selected(ids);
    }

    /// The active row, independent of keyboard focus.
    pub fn focused(&self) -> Option<ElementId> {
        self.list.focused()
    }

    /// Scrolls `id` into view. Returns false for an unknown ID.
    pub fn reveal(&self, id: &ElementId) -> bool {
        self.list.reveal(id)
    }

    /// Rows painted in the last frame; for performance checks.
    pub fn rows_rendered(&self) -> usize {
        self.list.rows_rendered()
    }
}

type RowRenderer = Rc<dyn Fn(&GraphRow, &ListRow, &mut Window, &mut App) -> AnyElement>;
type EventHandler = Rc<dyn Fn(&VirtualListEvent, &mut Window, &mut App)>;

/// A history list with a graph column. The caller lays the graph out (lanes,
/// edges and colors per row) and renders each row's text; this component draws
/// the visible part of the graph and the ref labels, and owns selection and
/// keyboard behavior through `VirtualList`.
#[derive(IntoElement)]
pub struct CommitGraph {
    id: ElementId,
    label: SharedString,
    state: CommitGraphState,
    render_row: RowRenderer,
    lane_width: Option<Pixels>,
    max_lanes: usize,
    row_height: Option<Pixels>,
    on_event: Option<EventHandler>,
    empty: Option<AnyElement>,
}

impl CommitGraph {
    /// A graph over `state`; `render_row` draws each visible row's content
    /// after the graph column and labels, such as the summary, author and date.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        state: &CommitGraphState,
        render_row: impl Fn(&GraphRow, &ListRow, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state: state.clone(),
            render_row: Rc::new(render_row),
            lane_width: None,
            max_lanes: DEFAULT_MAX_LANES,
            row_height: None,
            on_event: None,
            empty: None,
        }
    }

    /// The width of one lane; defaults to four spacing units.
    #[must_use]
    pub fn lane_width(mut self, width: Pixels) -> Self {
        if f32::from(width).is_finite() && width >= px(4.) {
            self.lane_width = Some(width);
        }
        self
    }

    /// Lanes drawn before the column clips (at least 1; default 12). Edges past
    /// the limit are drawn at the edge, and a fade marks the clipped rows.
    #[must_use]
    pub fn max_lanes(mut self, lanes: usize) -> Self {
        self.max_lanes = lanes.max(1);
        self
    }

    /// The row height; defaults to the `VirtualList` default.
    #[must_use]
    pub fn row_height(mut self, height: Pixels) -> Self {
        self.row_height = Some(height);
        self
    }

    /// Selection, focus and activation events, after the state has changed.
    #[must_use]
    pub fn on_event(
        mut self,
        handler: impl Fn(&VirtualListEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_event = Some(Rc::new(handler));
        self
    }

    /// Content shown when there are no rows, such as a loading or empty state.
    #[must_use]
    pub fn empty(mut self, content: impl IntoElement) -> Self {
        self.empty = Some(content.into_any_element());
        self
    }
}

/// Lane colors in the palette.
const PALETTE_SIZE: usize = 8;

/// The lane palette: eight evenly spaced hues at the saturation and lightness
/// of the theme's `info` color, so lanes follow the theme in light and dark.
fn palette(theme: &UiTheme) -> [Rgba; PALETTE_SIZE] {
    let base = Hsla::from(theme.colors.info);
    std::array::from_fn(|step| {
        let mut color = base;
        color.h = (base.h + step as f32 / PALETTE_SIZE as f32).fract();
        // Keep pale or gray themes readable: lanes need some saturation.
        color.s = color.s.max(0.55);
        Rgba::from(color)
    })
}

fn label_badge(label: &GraphLabel) -> Badge {
    let (icon, variant) = match label.kind {
        GraphLabelKind::Head => (LucideIcon::GitBranch, BadgeVariant::Default),
        GraphLabelKind::Branch => (LucideIcon::GitBranch, BadgeVariant::Secondary),
        GraphLabelKind::Remote => (LucideIcon::Cloud, BadgeVariant::Outline),
        GraphLabelKind::Tag => (LucideIcon::Tag, BadgeVariant::Outline),
        GraphLabelKind::Stash => (LucideIcon::Archive, BadgeVariant::Outline),
    };
    Badge::new(label.text.clone()).icon(icon).variant(variant)
}

impl RenderOnce for CommitGraph {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let lane_width = self.lane_width.unwrap_or(theme.spacing.unit * 4_f32);
        let max_lanes = self.max_lanes;
        let columns = self.state.lanes().clamp(1, max_lanes);
        let column_width = lane_width * columns as f32;
        let colors = palette(&theme);
        let fade = theme.colors.muted_foreground;
        let gap = theme.spacing.unit * 2_f32;
        let (accent, background) = (theme.colors.accent, theme.colors.background);

        let row_state = self.state.clone();
        let render_row = self.render_row.clone();
        let mut list = VirtualList::new(
            self.id,
            self.label,
            self.state.list.clone(),
            move |list_row, window, cx| {
                let Some(row) = row_state.row(list_row.index) else {
                    return div().into_any_element();
                };
                let shape = geometry(&row, max_lanes);
                let kind = row.kind;
                let node_color = row.color;
                let hole = if list_row.selected {
                    accent
                } else {
                    background
                };
                let content = render_row(&row, list_row, window, cx);
                div()
                    .size_full()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .gap(gap)
                    .child(
                        canvas(
                            |_, _, _| {},
                            move |bounds, (), window, _| {
                                paint_row(
                                    bounds, &shape, kind, node_color, lane_width, &colors, hole,
                                    fade, window,
                                );
                            },
                        )
                        .flex_shrink_0()
                        .h_full()
                        .w(column_width),
                    )
                    .children(
                        row.labels
                            .iter()
                            .map(|label| label_badge(label).flex_shrink_0()),
                    )
                    .child(div().flex_1().min_w_0().overflow_hidden().child(content))
                    .into_any_element()
            },
        );
        if let Some(height) = self.row_height {
            list = list.row_height(height);
        }
        if let Some(empty) = self.empty {
            list = list.empty(empty);
        }
        if let Some(handler) = self.on_event {
            list = list.on_event(move |event, window, cx| handler(event, window, cx));
        }
        list
    }
}

/// Paints one row's slice of the graph inside `bounds`.
#[allow(clippy::too_many_arguments)]
fn paint_row(
    bounds: Bounds<Pixels>,
    shape: &RowGeometry,
    kind: GraphNodeKind,
    node_color: usize,
    lane_width: Pixels,
    colors: &[Rgba; PALETTE_SIZE],
    hole: Rgba,
    fade: Rgba,
    window: &mut Window,
) {
    // The list row's 1px border insets this cell; reach across it so lanes
    // join the rows above and below without a gap.
    let top = bounds.top() - px(1.);
    let height = bounds.size.height + px(2.);
    let at = |(lane, y): (f32, f32)| -> Point<Pixels> {
        point(bounds.left() + lane_width * (lane + 0.5), top + height * y)
    };
    let color = |index: usize| colors[index % colors.len()];
    let stroke = (lane_width * 0.14).max(px(1.5));

    for segment in &shape.segments {
        let (from, to) = (at(segment.from), at(segment.to));
        let mut path = PathBuilder::stroke(stroke);
        path.move_to(from);
        if segment.from.0 == segment.to.0 {
            path.line_to(to);
        } else {
            // Bend at the vertical midpoint, so lanes leave and arrive upright.
            let middle = (from.y + to.y) / 2.;
            path.cubic_bezier_to(to, point(from.x, middle), point(to.x, middle));
        }
        if let Ok(path) = path.build() {
            window.paint_path(path, color(segment.color));
        }
    }

    let center = at((shape.node_lane, 0.5));
    let radius = (lane_width * 0.3).min(height * 0.3);
    let fill = color(node_color);
    let square = |r: Pixels| Bounds::new(point(center.x - r, center.y - r), size(r * 2., r * 2.));
    match kind {
        GraphNodeKind::Commit => {
            window.paint_quad(quad(
                square(radius),
                radius,
                fill,
                px(0.),
                fill,
                BorderStyle::Solid,
            ));
        }
        GraphNodeKind::Merge => {
            let ring = radius * 1.15;
            window.paint_quad(quad(
                square(ring),
                ring,
                hole,
                stroke,
                fill,
                BorderStyle::Solid,
            ));
            let dot = radius * 0.45;
            window.paint_quad(quad(
                square(dot),
                dot,
                fill,
                px(0.),
                fill,
                BorderStyle::Solid,
            ));
        }
        GraphNodeKind::Stash => {
            let side = radius * 0.9;
            window.paint_quad(quad(
                square(side),
                px(2.),
                fill,
                px(0.),
                fill,
                BorderStyle::Solid,
            ));
        }
        GraphNodeKind::WorkingTree => {
            window.paint_quad(quad(
                square(radius),
                radius,
                hole,
                stroke,
                fill,
                BorderStyle::Dashed,
            ));
        }
    }

    if shape.clipped {
        // A short dotted marker at the column's trailing edge.
        let x = bounds.right() - px(2.);
        for step in 0..3 {
            let y = top + height * (0.3 + 0.2 * step as f32);
            let dot = px(1.25);
            window.paint_quad(quad(
                Bounds::new(point(x - dot, y - dot), size(dot * 2., dot * 2.)),
                dot,
                fade,
                px(0.),
                fade,
                BorderStyle::Solid,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{Context, Render, TestAppContext};

    fn linear(count: usize) -> Vec<GraphRow> {
        (0..count)
            .map(|i| {
                let mut row = GraphRow::new(
                    SharedString::from(format!("c{i}")),
                    format!("commit {i}"),
                    0,
                );
                if i > 0 {
                    row = row.incoming(GraphEdge::straight(0, 0));
                }
                if i + 1 < count {
                    row = row.outgoing(GraphEdge::straight(0, 0));
                }
                row
            })
            .collect()
    }

    #[test]
    fn geometry_aligns_edges_to_row_edges_and_node() {
        let row = GraphRow::new("m", "merge", 0)
            .kind(GraphNodeKind::Merge)
            .incoming(GraphEdge::straight(0, 0))
            .incoming(GraphEdge::new(1, 0, 1))
            .outgoing(GraphEdge::straight(0, 0));
        let shape = geometry(&row, 12);
        assert!(!shape.clipped);
        assert_eq!(shape.node_lane, 0.);
        assert_eq!(shape.segments.len(), 3);
        // Incoming segments end at the node's height; outgoing ones start there
        // and reach the bottom edge, where the next row's incoming ones begin.
        assert_eq!(shape.segments[1].from, (1., 0.));
        assert_eq!(shape.segments[1].to, (0., 0.5));
        assert_eq!(shape.segments[2].from, (0., 0.5));
        assert_eq!(shape.segments[2].to, (0., 1.));
    }

    #[test]
    fn malformed_lanes_are_clamped_not_panicking() {
        let row = GraphRow::new("x", "wide", usize::MAX)
            .color(usize::MAX)
            .incoming(GraphEdge::new(usize::MAX, 40, usize::MAX))
            .outgoing(GraphEdge::new(3, usize::MAX, 7));
        let shape = geometry(&row, 4);
        assert!(shape.clipped);
        assert_eq!(shape.node_lane, 3.);
        assert!(
            shape
                .segments
                .iter()
                .all(|s| s.from.0 <= 3. && s.to.0 <= 3.)
        );
        assert_eq!(geometry(&row, 0).node_lane, 0.);
        // Lane counts saturate instead of overflowing.
        assert_eq!(row.lane_count(), usize::MAX);
    }

    #[test]
    fn appended_pages_extend_rows_and_lanes() {
        let state = CommitGraphState::new(linear(2)).unwrap();
        state.set_selected([ElementId::Name("c1".into())]);
        state
            .append_rows(vec![
                GraphRow::new("c2", "commit 2", 0).through(3, 1),
                GraphRow::new("c3", "commit 3", 0),
            ])
            .unwrap();
        assert_eq!(state.len(), 4);
        assert_eq!(state.lanes(), 4);
        assert_eq!(state.selected(), [ElementId::Name("c1".into())]);
        assert!(
            state
                .append_rows(vec![GraphRow::new("c0", "again", 0)])
                .is_err()
        );
        assert_eq!(state.len(), 4);
        assert_eq!(state.row(3).map(|row| row.label), Some("commit 3".into()));
    }

    #[test]
    fn state_tracks_lanes_and_rejects_duplicates() {
        let state = CommitGraphState::new(linear(3)).unwrap();
        assert_eq!(state.len(), 3);
        assert_eq!(state.lanes(), 1);
        state.set_selected([ElementId::from(SharedString::from("c1"))]);

        let mut wide = linear(3);
        wide[1] = wide[1].clone().through(2, 2);
        state.replace_rows(wide).unwrap();
        assert_eq!(state.lanes(), 3);
        assert_eq!(
            state.selected(),
            vec![ElementId::from(SharedString::from("c1"))]
        );

        let mut duplicate = linear(2);
        duplicate.push(duplicate[0].clone());
        assert!(state.replace_rows(duplicate).is_err());
        assert_eq!(state.len(), 3);
    }

    #[gpui_kit::test]
    fn paints_only_visible_rows(cx: &mut TestAppContext) {
        struct Probe(CommitGraphState);
        impl Render for Probe {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().w(px(480.)).h(px(320.)).child(CommitGraph::new(
                    "graph",
                    "History",
                    &self.0,
                    |row, _, _, _| div().child(row.label.clone()).into_any_element(),
                ))
            }
        }
        let state = CommitGraphState::new(linear(10_000)).unwrap();
        let probe_state = state.clone();
        cx.update(super::super::theme::init);
        let (_, cx) = cx.add_window_view(|_, _| Probe(probe_state));
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let rendered = state.rows_rendered();
        assert!(rendered > 0 && rendered < 40, "rendered {rendered} rows");
    }
}

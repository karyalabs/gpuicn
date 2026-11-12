//! Nova tabs with controlled selection and automatic keyboard activation.
use super::theme::{
    RovingFocus, UiTheme, apply_style,
    locale::DirectionalStyled as _,
    primitives::{disabled, focus, text},
};
use gpui_kit::{
    AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    MouseDownEvent, ParentElement as _, RenderOnce, Rgba, Role, SharedString, Stateful,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _,
};
use std::rc::Rc;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
/// The visual presentation of the tab list.
pub enum TabsVariant {
    #[default]
    /// The default Nova presentation.
    Default,
    /// Underline the selected tab.
    Line,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
/// How large the tabs are.
pub enum TabsSize {
    #[default]
    /// 14 px labels.
    Default,
    /// 13 px labels in a slightly shorter row, for sidebars and toolbars.
    Sm,
}

type SecondaryHandler = Rc<dyn Fn(&MouseDownEvent, &mut Window, &mut App)>;

/// One tab with a stable ID, selection value and visible label.
pub struct Tab {
    id: ElementId,
    value: SharedString,
    label: SharedString,
    disabled: bool,
    aria: Option<SharedString>,
    trailing: Option<AnyElement>,
    on_secondary: Option<SecondaryHandler>,
}
impl Tab {
    /// Creates a `Tab` with a stable caller-owned ID.
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            label: label.into(),
            disabled: false,
            aria: None,
            trailing: None,
            on_secondary: None,
        }
    }
    /// Disables interaction and applies the disabled appearance.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Draws `element` after the label, such as a count badge or an unread
    /// dot. Describe what it says in the tab's accessible label too.
    pub fn trailing(mut self, element: impl IntoElement) -> Self {
        self.trailing = Some(element.into_any_element());
        self
    }
    /// Replaces the accessible label, for a tab whose trailing element says
    /// more than its visible label ("Design, 2 mentions").
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria = Some(label.into());
        self
    }
    /// Runs `handler` on a secondary (right) click, to open the tab's
    /// context menu.
    pub fn on_secondary_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_secondary = Some(Rc::new(handler));
        self
    }
}
type ChangeHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;
#[derive(IntoElement)]
/// A controlled tab list with automatic keyboard selection.
pub struct Tabs {
    id: ElementId,
    selected: Option<SharedString>,
    variant: TabsVariant,
    label: Option<SharedString>,
    size: TabsSize,
    colors: Option<(Rgba, Rgba)>,
    items: Vec<Tab>,
    end: Option<AnyElement>,
    on_change: Option<ChangeHandler>,
    style: StyleRefinement,
}
impl Tabs {
    /// Creates a `Tabs` with a stable caller-owned ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: None,
            variant: Default::default(),
            label: None,
            size: Default::default(),
            colors: None,
            items: Vec::new(),
            end: None,
            on_change: None,
            style: Default::default(),
        }
    }
    /// Sets the caller-owned selected tab value.
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }
    /// Selects the Nova surface style.
    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }
    /// Sets the accessible name of the control.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets the label size.
    pub fn size(mut self, size: TabsSize) -> Self {
        self.size = size;
        self
    }
    /// Draws the selected tab's label (and the line under it) in
    /// `selected` and the other labels in `other`, for tabs on a surface
    /// with its own colors, such as a sidebar.
    pub fn text_colors(mut self, selected: impl Into<Rgba>, other: impl Into<Rgba>) -> Self {
        self.colors = Some((selected.into(), other.into()));
        self
    }
    /// Draws `element` at the end of the row, after the tabs: a "More"
    /// menu for tabs that do not fit, or an add button.
    pub fn end(mut self, element: impl IntoElement) -> Self {
        self.end = Some(element.into_any_element());
        self
    }
    /// Appends an option to the group.
    pub fn item(mut self, item: Tab) -> Self {
        self.items.push(item);
        self
    }
    /// Reports a requested value change; retain the next value in the owning view.
    pub fn on_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}
impl Styled for Tabs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let colors = theme.colors;
        let selected = self
            .items
            .iter()
            .position(|i| Some(&i.value) == self.selected.as_ref());
        let focus = RovingFocus::new(
            self.id.clone(),
            &self
                .items
                .iter()
                .map(|i| (i.id.clone(), i.disabled))
                .collect::<Vec<_>>(),
            selected,
            window,
            cx,
        );
        let values: Vec<_> = self.items.iter().map(|i| i.value.clone()).collect();
        let count = self.items.len();
        let (selected_color, other_color) = self
            .colors
            .unwrap_or((colors.foreground, colors.muted_foreground));
        let (height, size) = match self.size {
            TabsSize::Default => (6.5, 14.),
            TabsSize::Sm => (6., 13.),
        };
        let children = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let active = Some(index) == selected;
                let change = self.on_change.clone();
                let handle = focus.handles[index].as_ref();
                gpui_kit::base::Tab::new(item.id)
                    .accessibility_label(item.aria.unwrap_or_else(|| item.label.clone()))
                    .flex()
                    .items_center()
                    .gap(theme.space(1.5))
                    .child(item.label)
                    .children(item.trailing)
                    .selected(active)
                    .disabled(item.disabled)
                    .set_position(index + 1, count)
                    .when_some(handle, |tab, h| tab.track_focus(h))
                    .h(theme.space(height))
                    .px(theme.space(1.5))
                    .py(theme.space(0.5))
                    .text_size(theme.text(size))
                    .text_color(if active { selected_color } else { other_color })
                    .when(self.variant == TabsVariant::Default, |tab| {
                        tab.rounded(theme.radius.sm)
                            .border_1()
                            .border_color(colors.background.opacity(0.))
                            .when(active, |tab| {
                                tab.bg(colors.background).shadow(theme.shadows.sm.to_vec())
                            })
                    })
                    .when(self.variant == TabsVariant::Line, |tab| {
                        tab.border_b_2().border_color(if active {
                            selected_color
                        } else {
                            colors.background.opacity(0.)
                        })
                    })
                    .map(|tab| disabled::interactive(tab, item.disabled))
                    .focus_visible({
                        let theme = theme.clone();
                        move |s| focus::ring(s, &theme)
                    })
                    .when_some(change, |tab, handler| {
                        tab.on_click(move |_, window, cx| handler(&item.value, window, cx))
                    })
                    .when_some(item.on_secondary, |tab, handler| {
                        tab.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                            handler(event, window, cx)
                        })
                    })
            })
            .collect::<Vec<_>>();
        let on_change = self.on_change;
        let root = gpui_kit::base::Tabs::new(self.id)
            .flex_row_in(theme.direction)
            .items_center()
            .when_some(self.label, |root, label| root.aria_label(label))
            .when(self.variant == TabsVariant::Default, |root| {
                root.h(theme.space(8.))
                    .rounded(theme.radius.lg)
                    .p(theme.space(0.75))
                    .bg(colors.muted)
            })
            .when(self.variant == TabsVariant::Line, |root| {
                root.gap(theme.space(1.))
                    .border_b_1()
                    .border_color(colors.border)
            })
            .on_key_down(move |event, window, cx| {
                if let Some(index) = focus.key(event, Some(gpui_kit::Axis::Horizontal), window, cx)
                    && let Some(change) = &on_change
                {
                    change(&values[index], window, cx);
                }
            })
            .children(children)
            .children(self.end);
        apply_style(root, &self.style)
    }
}
/// Mount the selected content explicitly in the caller's render function.
pub fn tabs_content(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    cx: &App,
) -> Stateful<Div> {
    let theme = UiTheme::read(cx);
    div()
        .id(id)
        .role(Role::TabPanel)
        .aria_label(label)
        .tab_index(0)
        .flex_1()
        .map(|panel| text::sm(panel, theme))
        .text_color(theme.colors.foreground)
}

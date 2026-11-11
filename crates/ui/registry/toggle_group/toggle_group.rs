//! Nova toggle sets with controlled selection and arrow-key focus.
use super::theme::{
    RovingFocus, UiTheme, apply_style,
    locale::DirectionalStyled as _,
    primitives::{control, focus},
};
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    prelude::FluentBuilder as _, px,
};
use std::rc::Rc;

type ChangeHandler = Rc<dyn Fn(&[SharedString], &mut Window, &mut App)>;
/// A labeled toggle option with custom child content.
pub struct ToggleGroupItem {
    id: ElementId,
    value: SharedString,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    style: StyleRefinement,
}
impl ToggleGroupItem {
    /// Creates a `ToggleGroupItem` with a stable caller-owned ID.
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            label: None,
            children: Vec::new(),
            disabled: false,
            style: Default::default(),
        }
    }
    /// Disables interaction and applies the disabled appearance.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Sets the accessible name of the control.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}
impl Styled for ToggleGroupItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl ParentElement for ToggleGroupItem {
    fn extend(&mut self, children: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(children);
    }
}
#[derive(IntoElement)]
/// A controlled collection of toggles with roving keyboard focus.
pub struct ToggleGroup {
    id: ElementId,
    value: Vec<SharedString>,
    label: Option<SharedString>,
    disabled: bool,
    multiple: bool,
    joined: bool,
    segmented: bool,
    items: Vec<ToggleGroupItem>,
    on_change: Option<ChangeHandler>,
    style: StyleRefinement,
}
impl ToggleGroup {
    /// Creates a `ToggleGroup` with a stable caller-owned ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: Vec::new(),
            label: None,
            disabled: false,
            multiple: false,
            joined: true,
            segmented: false,
            items: Vec::new(),
            on_change: None,
            style: Default::default(),
        }
    }
    /// Sets the caller-owned value displayed by this control.
    pub fn value(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.value = values.into_iter().map(Into::into).collect();
        self
    }
    /// Sets the accessible name of the control.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Disables interaction and applies the disabled appearance.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    /// Allows multiple selected values instead of one.
    pub fn multiple(mut self, value: bool) -> Self {
        self.multiple = value;
        self
    }
    /// Removes gaps between adjacent toggles.
    pub fn joined(mut self, value: bool) -> Self {
        self.joined = value;
        self
    }
    /// Draws the group as one segmented control: a filled track with the
    /// pressed option raised on it. For choosing one of a few options,
    /// where the choice must show at a glance.
    pub fn segmented(mut self, value: bool) -> Self {
        self.segmented = value;
        self
    }
    /// Appends an option to the group.
    pub fn item(mut self, item: ToggleGroupItem) -> Self {
        self.items.push(item);
        self
    }
    /// Reports a requested value change; retain the next value in the owning view.
    pub fn on_change(
        mut self,
        handler: impl Fn(&[SharedString], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}
impl Styled for ToggleGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl RenderOnce for ToggleGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let control = theme.control();
        let focus = RovingFocus::new(
            self.id.clone(),
            &self
                .items
                .iter()
                .map(|i| (i.id.clone(), self.disabled || i.disabled))
                .collect::<Vec<_>>(),
            None,
            window,
            cx,
        );
        let count = self.items.len();
        let segmented = self.segmented;
        let joined = self.joined && !segmented;
        let colors = theme.colors;
        let raised = control.segment;
        let children = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let pressed = self.value.contains(&item.value);
                let disabled = self.disabled || item.disabled;
                let handle = focus.handles[index].as_ref();
                let change = self.on_change.clone();
                let mut next = if self.multiple {
                    self.value.clone()
                } else {
                    Vec::new()
                };
                next.retain(|value| value != &item.value);
                if !pressed {
                    next.push(item.value);
                }
                let toggle = gpui_kit::base::Toggle::new(item.id)
                    .pressed(pressed)
                    .disabled(disabled)
                    .when_some(item.label, |toggle, label| {
                        toggle.accessibility_label(label)
                    })
                    .when_some(handle, |toggle, h| {
                        toggle.track_focus(h).tab_stop(h.tab_stop)
                    })
                    .flex_row_in(theme.direction)
                    .items_center()
                    .justify_center()
                    .gap(theme.space(1.))
                    .h(theme.space(if segmented { 7. } else { 8. }))
                    .min_w(theme.space(8.))
                    .px(theme.space(2.5))
                    .rounded(if joined { px(0.) } else { theme.radius.lg })
                    .when(joined && index == 0, |t| t.rounded_l(theme.radius.lg))
                    .when(joined && index + 1 == count, |t| {
                        t.rounded_r(theme.radius.lg)
                    })
                    .map(|t| control::toggle(t, &theme, pressed, disabled, false))
                    .when(segmented, |t| {
                        if pressed {
                            t.bg(raised)
                                .border_color(colors.border)
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                        } else {
                            t.bg(colors.background.opacity(0.))
                                .text_color(colors.muted_foreground)
                        }
                    })
                    .focus_visible({
                        let theme = theme.clone();
                        move |s| focus::ring(s, &theme)
                    })
                    .when(!disabled && !segmented, |t| {
                        t.hover(move |s| s.bg(control.hover))
                    })
                    .when(!disabled && segmented && !pressed, |t| {
                        t.hover(move |s| s.text_color(colors.foreground))
                    })
                    .children(item.children)
                    .when_some(change, |toggle, handler| {
                        toggle.on_change(move |_, _, window, cx| handler(&next, window, cx))
                    });
                apply_style(toggle, &item.style)
            })
            .collect::<Vec<_>>();
        let root = gpui_kit::base::ToggleGroup::new(self.id)
            .flex_row_in(theme.direction)
            .gap(if segmented {
                px(2.)
            } else {
                theme.space(if joined { 0. } else { 2. })
            })
            .when(segmented, |root| {
                root.p(px(2.))
                    .rounded(theme.radius.lg + px(2.))
                    .bg(colors.muted)
                    .border_1()
                    .border_color(colors.border)
            })
            .when_some(self.label, |root, label| root.aria_label(label))
            .on_key_down(move |event, window, cx| {
                focus.key(event, Some(gpui_kit::Axis::Horizontal), window, cx);
            })
            .children(children);
        apply_style(root, &self.style)
    }
}

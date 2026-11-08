//! Nova item rows: a title, an optional description, media and actions, laid
//! out in a row and grouped into a card with hairline separators. A port of
//! shadcn/ui's `Item` and `ItemGroup`, for settings pages and lists.

use gpui_kit::base::Button as BaseButton;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _, Styled, Window,
    div, prelude::FluentBuilder as _, px,
};
use std::rc::Rc;

use super::theme::{UiTheme, apply_style, locale::DirectionalStyled as _, primitives::disabled};

type Handler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// The surface an [`Item`] or [`ItemGroup`] draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemVariant {
    /// No border or background; rows sit on the page.
    #[default]
    Default,
    /// A bordered card with large corners.
    Outline,
    /// A muted background without a border.
    Muted,
}

/// The density of an [`Item`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemSize {
    /// At least 48px tall with 10px by 16px padding.
    #[default]
    Default,
    /// At least 36px tall with 6px by 12px padding.
    Sm,
}

/// One row: media, a title with an optional description, and actions at the
/// trailing edge (or under the copy when [`Item::stacked`]).
#[derive(IntoElement)]
pub struct Item {
    id: ElementId,
    style: gpui_kit::StyleRefinement,
    variant: ItemVariant,
    size: ItemSize,
    media: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    actions: Option<AnyElement>,
    children: Vec<AnyElement>,
    stacked: bool,
    disabled: bool,
    on_click: Option<Handler>,
}

impl Item {
    /// Creates an empty row with a caller-owned ID, unique among its siblings.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: Default::default(),
            variant: ItemVariant::Default,
            size: ItemSize::Default,
            media: None,
            title: None,
            description: None,
            actions: None,
            children: Vec::new(),
            stacked: false,
            disabled: false,
            on_click: None,
        }
    }

    /// Chooses the row's surface.
    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Chooses the row's density.
    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the leading icon, avatar or image.
    pub fn media(mut self, media: impl IntoElement) -> Self {
        self.media = Some(media.into_any_element());
        self
    }

    /// Sets the row's title, which is also its accessible name.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the muted line under the title, clamped to two lines.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the controls at the trailing edge, such as a switch or a button.
    pub fn actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    /// Puts the actions under the title and description at full width, for
    /// wide controls such as a tag field or a row of swatches.
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
        self
    }

    /// Disables a clickable row and applies the disabled appearance.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Makes the whole row a button that runs `handler`, for rows that open
    /// something. A clickable row should not also hold actions.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Item {
    /// Adds content under the description, inside the copy column.
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Item {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

fn surface<T: Styled>(element: T, variant: ItemVariant, theme: &UiTheme) -> T {
    match variant {
        ItemVariant::Default => element,
        ItemVariant::Outline => element
            .border_1()
            .border_color(theme.colors.border)
            .rounded(theme.radius.lg)
            .bg(theme.colors.card),
        ItemVariant::Muted => element.rounded(theme.radius.lg).bg(theme.colors.muted),
    }
}

impl Item {
    fn lay_out<T: Styled + ParentElement + gpui_kit::prelude::FluentBuilder>(
        self,
        element: T,
        theme: &UiTheme,
    ) -> T {
        let (min_h, px_, py) = match self.size {
            ItemSize::Default => (48., 16., 10.),
            ItemSize::Sm => (36., 12., 6.),
        };
        let copy = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(2.))
            .when_some(self.title, |copy, title| {
                copy.child(
                    div()
                        .text_size(theme.text(13.5))
                        .font_weight(FontWeight::NORMAL)
                        .text_color(theme.colors.foreground)
                        .child(title),
                )
            })
            .when_some(self.description, |copy, description| {
                copy.child(
                    div()
                        .text_size(theme.text(12.))
                        .text_color(theme.colors.muted_foreground)
                        .line_clamp(2)
                        .child(description),
                )
            })
            .children(self.children);
        let head = div()
            .flex_row_in(theme.direction)
            .items_center()
            .gap(theme.space(3.))
            .min_w_0()
            .w_full()
            .when_some(self.media, |head, media| {
                head.child(div().flex_shrink_0().flex().items_center().child(media))
            })
            .child(copy);
        let element = surface(element, self.variant, theme)
            .w_full()
            .min_h(px(min_h))
            .px(px(px_))
            .py(px(py))
            .font_family(theme.fonts.body.clone())
            .text_color(theme.colors.foreground);
        let actions = self.actions.map(|actions| {
            div()
                .flex_shrink_0()
                .flex()
                .items_center()
                .gap(theme.space(2.))
                .child(actions)
        });
        if self.stacked {
            element
                .flex()
                .flex_col()
                .gap(theme.space(2.5))
                .child(head)
                .when_some(actions, |el, actions| el.child(actions.w_full()))
        } else {
            element
                .flex_row_in(theme.direction)
                .items_center()
                .gap(theme.space(4.))
                .child(head.flex_1())
                .when_some(actions, |el, actions| el.child(actions))
        }
    }
}

impl RenderOnce for Item {
    fn render(mut self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let style = std::mem::take(&mut self.style);
        let label = self.title.clone().unwrap_or_default();
        match self.on_click.take() {
            Some(handler) => {
                let disabled = self.disabled;
                let accent = theme.colors.accent;
                let ring = theme.colors.ring;
                let button = BaseButton::new(self.id.clone())
                    .accessibility_label(label)
                    .disabled(disabled)
                    .when(!disabled, |el| {
                        el.cursor_pointer()
                            .hover(move |style| style.bg(accent))
                            .focus_visible(move |style| style.border_color(ring))
                    })
                    .map(|el| disabled::dim(el, disabled))
                    .border_1()
                    .border_color(theme.colors.border.opacity(0.))
                    .rounded(theme.radius.lg)
                    .on_click(move |event, window, cx| handler(event, window, cx));
                apply_style(self.lay_out(button, &theme), &style).into_any_element()
            }
            None => {
                let root = div()
                    .id(self.id.clone())
                    .role(Role::Group)
                    .when(!label.is_empty(), |el| el.aria_label(label));
                apply_style(self.lay_out(root, &theme), &style).into_any_element()
            }
        }
    }
}

/// A column of [`Item`]s with hairline separators between them. In the
/// [`ItemVariant::Outline`] variant it is one bordered card, the way settings
/// pages group rows.
#[derive(IntoElement)]
pub struct ItemGroup {
    id: ElementId,
    style: gpui_kit::StyleRefinement,
    variant: ItemVariant,
    children: Vec<AnyElement>,
}

impl ItemGroup {
    /// Creates an empty group with a caller-owned ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: Default::default(),
            variant: ItemVariant::Outline,
            children: Vec::new(),
        }
    }

    /// Chooses the group's surface; [`ItemVariant::Outline`] by default.
    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl ParentElement for ItemGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for ItemGroup {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ItemGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx);
        let mut root = surface(
            div()
                .id(self.id)
                .role(Role::List)
                .flex()
                .flex_col()
                .w_full(),
            self.variant,
            theme,
        )
        .overflow_hidden();
        for (index, child) in self.children.into_iter().enumerate() {
            if index > 0 {
                root = root.child(ItemSeparator::new());
            }
            root = root.child(child);
        }
        apply_style(root, &self.style)
    }
}

/// A one-pixel rule between items, inset to the group's edges.
#[derive(IntoElement, Default)]
pub struct ItemSeparator;

impl ItemSeparator {
    /// Creates a separator.
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ItemSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx);
        div()
            .flex_shrink_0()
            .w_full()
            .h(px(1.))
            .bg(theme.colors.border)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{Context, Render, size};

    struct Rows {
        stacked: bool,
    }
    impl Render for Rows {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().w(px(320.)).child(
                ItemGroup::new("group").child(
                    Item::new("row")
                        .title("Reduce motion")
                        .description("Turns off animations")
                        .stacked(self.stacked)
                        .actions(
                            div()
                                .debug_selector(|| "control".into())
                                .w(px(36.))
                                .h(px(20.)),
                        )
                        .child(div().debug_selector(|| "copy-end".into())),
                ),
            )
        }
    }

    fn layout(
        stacked: bool,
        cx: &mut gpui_kit::TestAppContext,
    ) -> (
        gpui_kit::Bounds<gpui_kit::Pixels>,
        gpui_kit::Bounds<gpui_kit::Pixels>,
    ) {
        cx.update(super::super::theme::init);
        let (_, cx) = cx.add_window_view(|_, _| Rows { stacked });
        cx.simulate_resize(size(px(800.), px(600.)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        (
            cx.debug_bounds("control").expect("the control is drawn"),
            cx.debug_bounds("copy-end").expect("the copy is drawn"),
        )
    }

    #[gpui_kit::test]
    fn the_control_stays_on_the_title_row_at_320px(cx: &mut gpui_kit::TestAppContext) {
        let (control, copy) = layout(false, cx);
        // The control is at the trailing edge, beside the copy, not under it.
        assert!(control.left() > copy.left(), "{control:?} {copy:?}");
        assert!(control.top() < copy.bottom(), "{control:?} {copy:?}");
        assert!(control.right() <= px(320.), "{control:?}");
    }

    #[gpui_kit::test]
    fn stacked_puts_the_control_under_the_copy(cx: &mut gpui_kit::TestAppContext) {
        let (control, copy) = layout(true, cx);
        assert!(control.top() >= copy.bottom(), "{control:?} {copy:?}");
        assert!(control.left() <= px(17.), "{control:?}");
    }
}

//! Button examples.

use crate::*;

/// Sizes
///
/// Four heights, from 24px to 36px. The default is 32px.
pub(crate) fn sizes(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            Button::new("sizes.xs")
                .size(ButtonSize::Xs)
                .label("Extra small"),
        )
        .child(Button::new("sizes.sm").size(ButtonSize::Sm).label("Small"))
        .child(Button::new("sizes.default").label("Default"))
        .child(Button::new("sizes.lg").size(ButtonSize::Lg).label("Large"))
}

/// With an icon
///
/// Children are laid out in order, so add the icon before or after the label.
pub(crate) fn with_icon(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let theme = UiTheme::read(cx);
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            Button::new("icon.leading")
                .child(
                    lucide(LucideIcon::Mail)
                        .size(px(16.))
                        .text_color(theme.colors.primary_foreground),
                )
                .label("Send email"),
        )
        .child(
            Button::new("icon.trailing")
                .variant(ButtonVariant::Outline)
                .label("Next")
                .child(
                    lucide(LucideIcon::ArrowRight)
                        .size(px(16.))
                        .text_color(theme.colors.foreground),
                ),
        )
}

/// Icon only
///
/// The square sizes hold one icon. Give the button an `aria_label`, because it has no text.
pub(crate) fn icon_only(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    let button = |id: &'static str, size: ButtonSize, icon: LucideIcon, label: &'static str| {
        Button::new(id)
            .size(size)
            .variant(ButtonVariant::Outline)
            .aria_label(label)
            .child(lucide(icon).size(px(16.)).text_color(color))
    };
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(button(
            "only.xs",
            ButtonSize::IconXs,
            LucideIcon::Plus,
            "Add",
        ))
        .child(button(
            "only.sm",
            ButtonSize::IconSm,
            LucideIcon::Search,
            "Search",
        ))
        .child(button(
            "only.default",
            ButtonSize::Icon,
            LucideIcon::Settings,
            "Settings",
        ))
        .child(button(
            "only.lg",
            ButtonSize::IconLg,
            LucideIcon::Trash2,
            "Delete",
        ))
}

/// Tinted
///
/// `tint` colors one button without a new theme color. The text color is picked for contrast.
pub(crate) fn tinted(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            Button::new("tint.green")
                .tint(gpui_kit::rgb(0x16a34a))
                .label("Approve"),
        )
        .child(
            Button::new("tint.amber")
                .tint(gpui_kit::rgb(0xf59e0b))
                .label("Hold"),
        )
        .child(
            Button::new("tint.violet")
                .tint(gpui_kit::rgb(0x7c3aed))
                .label("Upgrade"),
        )
}

/// Disabled
///
/// A disabled button keeps its place in the layout but takes no clicks or focus.
pub(crate) fn disabled(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(Button::new("disabled.default").disabled(true).label("Save"))
        .child(
            Button::new("disabled.outline")
                .variant(ButtonVariant::Outline)
                .disabled(true)
                .label("Cancel"),
        )
        .child(
            Button::new("disabled.ghost")
                .variant(ButtonVariant::Ghost)
                .disabled(true)
                .label("Skip"),
        )
}

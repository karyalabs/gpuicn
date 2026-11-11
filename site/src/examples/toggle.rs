//! Toggle examples.

use crate::*;
use gpuicn::toggle::ToggleSize;

/// Sizes
///
/// Small, default and large.
pub(crate) fn sizes(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    let toggle = |id: &'static str, size: ToggleSize| {
        Toggle::new(id)
            .size(size)
            .variant(ToggleVariant::Outline)
            .aria_label("Bold")
            .child(lucide(LucideIcon::Bold).size(px(16.)).text_color(color))
    };
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(toggle("sizes.sm", ToggleSize::Sm))
        .child(toggle("sizes.default", ToggleSize::Default))
        .child(toggle("sizes.lg", ToggleSize::Lg))
}

/// With text
///
/// A toggle can hold an icon and a label.
pub(crate) fn with_text(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    let this = cx.entity().downgrade();
    Toggle::new("text.star")
        .pressed(view.pressed)
        .variant(ToggleVariant::Outline)
        .aria_label("Star")
        .on_change(move |pressed, _, cx| {
            this.update(cx, |view, cx| {
                view.pressed = *pressed;
                cx.notify();
            })
            .ok();
        })
        .child(lucide(LucideIcon::Star).size(px(16.)).text_color(color))
        .child(if view.pressed { "Starred" } else { "Star" })
}

/// Outline
///
/// The default toggle has no border until it is pressed. The outline variant always has one.
pub(crate) fn outline(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    let icon = move |icon: LucideIcon| lucide(icon).size(px(16.)).text_color(color);
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            Toggle::new("outline.default")
                .aria_label("Italic")
                .child(icon(LucideIcon::Italic)),
        )
        .child(
            Toggle::new("outline.outline")
                .variant(ToggleVariant::Outline)
                .aria_label("Italic")
                .child(icon(LucideIcon::Italic)),
        )
}

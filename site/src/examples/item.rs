//! Item examples.

use crate::*;
use gpuicn::item::{Item, ItemVariant};

/// Variants
///
/// Plain, outlined and muted rows.
pub(crate) fn variants(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(380.))
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(
            Item::new("variants.default")
                .title("Default")
                .description("No border or fill."),
        )
        .child(
            Item::new("variants.outline")
                .variant(ItemVariant::Outline)
                .title("Outline")
                .description("A border around the row."),
        )
        .child(
            Item::new("variants.muted")
                .variant(ItemVariant::Muted)
                .title("Muted")
                .description("A quiet fill."),
        )
}

/// With an action
///
/// Media goes before the text and actions after it.
pub(crate) fn with_an_action(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    div().w_full().max_w(px(380.)).child(
        Item::new("action.row")
            .variant(ItemVariant::Outline)
            .media(lucide(LucideIcon::ShieldCheck).size(px(20.)).text_color(color))
            .title("Two-factor sign-in")
            .description("Ask for a code on new devices.")
            .actions(
                Button::new("action.enable")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Outline)
                    .label("Enable"),
            ),
    )
}

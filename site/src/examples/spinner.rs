//! Spinner examples.

use crate::*;

/// Sizes
///
/// A spinner is 16px unless you size it.
pub(crate) fn sizes(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(20.))
        .child(Spinner::new("sizes.default").label("Loading"))
        .child(Spinner::new("sizes.medium").size(px(24.)).label("Loading"))
        .child(Spinner::new("sizes.large").size(px(32.)).label("Loading"))
}

/// In a button
///
/// Disable the button while it works, and match the spinner to its text color.
pub(crate) fn in_a_button(
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
            Button::new("button.primary")
                .disabled(true)
                .child(
                    Spinner::new("button.primary.spinner")
                        .label("Saving")
                        .color(theme.colors.primary_foreground),
                )
                .label("Saving…"),
        )
        .child(
            Button::new("button.outline")
                .variant(ButtonVariant::Outline)
                .disabled(true)
                .child(
                    Spinner::new("button.outline.spinner")
                        .label("Loading")
                        .color(theme.colors.foreground),
                )
                .label("Loading…"),
        )
}

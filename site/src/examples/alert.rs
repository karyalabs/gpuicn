//! Alert examples.

use crate::*;
use gpuicn::alert::{Alert, AlertVariant};

/// With an action
///
/// Children follow the description, which is the place for a button.
pub(crate) fn with_action(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div().w_full().max_w(px(440.)).child(
        Alert::new("action")
            .variant(AlertVariant::Destructive)
            .title("The upload failed")
            .description("The connection dropped after 12 MB.")
            .child(
                Button::new("action.retry")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Outline)
                    .label("Try again"),
            ),
    )
}

/// Custom icon
///
/// Each variant has its own icon. Replace it with `icon`, or drop it with `no_icon`.
pub(crate) fn custom_icon(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(440.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            Alert::new("icon.custom")
                .icon(LucideIcon::Rocket)
                .title("Version 2 is out")
                .description("Restart the app to update."),
        )
        .child(
            Alert::new("icon.none")
                .variant(AlertVariant::Info)
                .no_icon()
                .title("No icon")
                .description("The text starts at the edge."),
        )
}

/// Title only
///
/// A one-line alert for a short confirmation.
pub(crate) fn title_only(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(440.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            Alert::new("title.success")
                .variant(AlertVariant::Success)
                .title("Settings saved"),
        )
        .child(
            Alert::new("title.warning")
                .variant(AlertVariant::Warning)
                .title("Your trial ends in three days"),
        )
}

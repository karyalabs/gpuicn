//! Toast examples.

use crate::*;
use std::time::Duration;

/// With an action
///
/// A notice can offer one action, such as undoing what it reports.
pub(crate) fn with_an_action(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let toasts = window.use_keyed_state("action.toasts", cx, |_, cx| ToastState::new(cx));
    let push = toasts.clone();
    div()
        .child(
            Button::new("action.show")
                .variant(ButtonVariant::Outline)
                .label("Archive message")
                .on_click(move |_, _, cx| {
                    push.update(cx, |toasts, cx| {
                        toasts.push(
                            "archived",
                            Notice::new("Message archived").action("Undo", |_, _| {}),
                            cx,
                        )
                    })
                }),
        )
        .child(toasts)
}

/// Timing
///
/// A notice leaves after its timeout. A persistent one stays until it is dismissed.
pub(crate) fn timing(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let toasts = window.use_keyed_state("timing.toasts", cx, |_, cx| ToastState::new(cx));
    let quick = toasts.clone();
    let sticky = toasts.clone();
    div()
        .flex()
        .gap(px(8.))
        .child(
            Button::new("timing.quick")
                .variant(ButtonVariant::Outline)
                .label("Two seconds")
                .on_click(move |_, _, cx| {
                    quick.update(cx, |toasts, cx| {
                        toasts.push(
                            "quick",
                            Notice::new("Copied to clipboard").timeout(Duration::from_secs(2)),
                            cx,
                        )
                    })
                }),
        )
        .child(
            Button::new("timing.sticky")
                .variant(ButtonVariant::Outline)
                .label("Until dismissed")
                .on_click(move |_, _, cx| {
                    sticky.update(cx, |toasts, cx| {
                        toasts.push(
                            "sticky",
                            Notice::new("You are offline")
                                .description("Changes will sync when you reconnect.")
                                .persistent()
                                .cancel("Dismiss"),
                            cx,
                        )
                    })
                }),
        )
        .child(toasts)
}

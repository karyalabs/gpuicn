//! Number Field examples.

use crate::*;

/// Step and limits
///
/// The limits and the step belong to the input's state. This one moves in fives, from 0 to 100.
pub(crate) fn step_and_limits(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("step.state", cx, |window, cx| {
        InputState::new(window, cx)
            .default_value("25")
            .min(0.)
            .max(100.)
            .step(5.)
    });
    div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child("Opacity, in steps of 5")
        .child(NumberField::new(&state).aria_label("Opacity").w(px(200.)))
}

/// Disabled and read only
///
/// A read-only field can be focused and copied from. A disabled one cannot.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let read_only = window.use_keyed_state("off.read-only", cx, |window, cx| {
        InputState::new(window, cx).default_value("8")
    });
    let disabled = window.use_keyed_state("off.disabled", cx, |window, cx| {
        InputState::new(window, cx).default_value("2")
    });
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            NumberField::new(&read_only)
                .aria_label("Read only")
                .read_only(true)
                .w(px(200.)),
        )
        .child(
            NumberField::new(&disabled)
                .aria_label("Disabled")
                .disabled(true)
                .w(px(200.)),
        )
}

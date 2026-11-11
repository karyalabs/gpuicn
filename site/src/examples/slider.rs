//! Slider examples.

use crate::*;
use gpuicn::slider::SliderState;

/// Steps
///
/// `step` snaps the value. This one moves in tens.
pub(crate) fn steps(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("steps.state", cx, |_, _| {
        SliderState::new()
            .min(0.)
            .max(100.)
            .step(10.)
            .default_value(40.)
    });
    div()
        .w_full()
        .max_w(px(280.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child("Brightness, in steps of 10")
        .child(Slider::new(&state).aria_label("Brightness"))
}

/// Gradient track
///
/// `gradient` paints the track with color stops, for a hue or opacity picker.
pub(crate) fn gradient(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("gradient.state", cx, |_, _| {
        SliderState::new()
            .min(0.)
            .max(360.)
            .step(1.)
            .default_value(200.)
    });
    div()
        .w_full()
        .max_w(px(280.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child("Hue")
        .child(
            Slider::new(&state).aria_label("Hue").gradient(
                [0xef4444, 0xeab308, 0x22c55e, 0x06b6d4, 0x3b82f6, 0xa855f7, 0xef4444]
                    .map(gpui_kit::rgb),
            ),
        )
}

/// Disabled
///
/// A disabled slider shows its value but does not move.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("disabled.state", cx, |_, _| {
        SliderState::new().min(0.).max(100.).default_value(65.)
    });
    div()
        .w_full()
        .max_w(px(280.))
        .child(Slider::new(&state).aria_label("Volume").disabled(true))
}

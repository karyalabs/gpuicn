//! Color Picker examples.

use crate::*;
use gpuicn::color_picker::{ColorPicker, ColorPickerState, Oklch};

/// Without swatches
///
/// The picker alone: lightness, chroma and hue.
pub(crate) fn without_swatches(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("plain.state", cx, |_, cx| {
        ColorPickerState::new(Oklch::new(0.6, 0.15, 150.), cx)
    });
    div()
        .w_full()
        .max_w(px(300.))
        .child(ColorPicker::new(&state))
}

/// Disabled
///
/// A disabled picker shows its color but does not change.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("off.state", cx, |_, cx| {
        ColorPickerState::new(Oklch::new(0.6, 0.2, 10.), cx)
    });
    div()
        .w_full()
        .max_w(px(300.))
        .child(ColorPicker::new(&state).disabled(true))
}

//! OTP Field examples.

use crate::*;

/// Four digits
///
/// The state sets the length. Pasting a code fills every box.
pub(crate) fn four_digits(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state =
        window.use_keyed_state("four.state", cx, |window, cx| OtpState::new(4, window, cx));
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child("Enter your PIN")
        .child(OtpField::new(&state).aria_label("PIN"))
}

/// Disabled
///
/// A disabled field takes no input, for example while the code is being checked.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state =
        window.use_keyed_state("off.state", cx, |window, cx| OtpState::new(6, window, cx));
    OtpField::new(&state)
        .aria_label("Verification code")
        .disabled(true)
}

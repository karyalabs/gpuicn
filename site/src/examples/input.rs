//! Input examples.

use crate::*;

/// Placeholder
///
/// The placeholder belongs to the input's state and shows while it is empty.
pub(crate) fn placeholder(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let search = window.use_keyed_state("placeholder.search", cx, |window, cx| {
        InputState::new(window, cx).placeholder("Search components")
    });
    div()
        .w_full()
        .max_w(px(320.))
        .child(Input::new(&search).aria_label("Search"))
}

/// Invalid
///
/// `invalid` draws the destructive border. Say what is wrong in text below.
pub(crate) fn invalid(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let email = window.use_keyed_state("invalid.email", cx, |window, cx| {
        InputState::new(window, cx).default_value("ada@example")
    });
    let destructive = UiTheme::read(cx).colors.destructive;
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(6.))
        .child(Input::new(&email).aria_label("Email").invalid(true))
        .child(
            div()
                .text_size(px(13.))
                .text_color(destructive)
                .child("Enter a full email address."),
        )
}

/// Disabled and read only
///
/// A read-only input can still be focused and copied from. A disabled one cannot.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let read_only = window.use_keyed_state("disabled.read-only", cx, |window, cx| {
        InputState::new(window, cx).default_value("Read only")
    });
    let disabled = window.use_keyed_state("disabled.disabled", cx, |window, cx| {
        InputState::new(window, cx).default_value("Disabled")
    });
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            Input::new(&read_only)
                .aria_label("Read only")
                .read_only(true),
        )
        .child(Input::new(&disabled).aria_label("Disabled").disabled(true))
}

//! Select examples.

use crate::*;

fn roles() -> [SelectItem; 3] {
    [
        SelectItem::new("viewer", "Viewer"),
        SelectItem::new("editor", "Editor"),
        SelectItem::new("owner", "Owner"),
    ]
}

/// With a value
///
/// Set the starting value on the state. Read it back with `value`.
pub(crate) fn with_a_value(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("value.state", cx, |window, cx| {
        let mut state = SelectState::new(roles(), window, cx);
        state.set_value(Some("editor".into()), window, cx);
        state
    });
    let role = state.read(cx).value().cloned().unwrap_or_default();
    div()
        .w_full()
        .max_w(px(240.))
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(Select::new(&state).aria_label("Role"))
        .child(format!("Selected: {role}"))
}

/// Invalid
///
/// `invalid` draws the destructive border when a choice is required.
pub(crate) fn invalid(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("invalid.state", cx, |window, cx| {
        SelectState::new(roles(), window, cx)
    });
    let missing = state.read(cx).value().is_none();
    let destructive = UiTheme::read(cx).colors.destructive;
    div()
        .w_full()
        .max_w(px(240.))
        .flex()
        .flex_col()
        .gap(px(6.))
        .child(
            Select::new(&state)
                .aria_label("Role")
                .placeholder("Pick a role…")
                .invalid(missing),
        )
        .when(missing, |column| {
            column.child(
                div()
                    .text_size(px(13.))
                    .text_color(destructive)
                    .child("A role is required."),
            )
        })
}

/// Disabled
///
/// A disabled select shows its placeholder or value but does not open.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("disabled.state", cx, |window, cx| {
        SelectState::new(roles(), window, cx)
    });
    Select::new(&state)
        .aria_label("Role")
        .placeholder("Managed by your admin")
        .disabled(true)
        .w_full()
        .max_w(px(240.))
}

//! Combobox examples.

use crate::*;
use gpuicn::combobox::combobox;

fn countries() -> [SelectItem; 5] {
    [
        SelectItem::new("br", "Brazil"),
        SelectItem::new("in", "India"),
        SelectItem::new("jp", "Japan"),
        SelectItem::new("ke", "Kenya"),
        SelectItem::new("no", "Norway"),
    ]
}

/// Empty
///
/// With no value, the placeholder shows. Type to filter the list.
pub(crate) fn empty(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("empty.state", cx, |window, cx| {
        SelectState::new(countries(), window, cx)
    });
    combobox(&state)
        .aria_label("Country")
        .placeholder("Search countries…")
        .w(px(240.))
}

/// Disabled
///
/// A disabled combobox shows its value but takes no input.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("off.state", cx, |window, cx| {
        let mut state = SelectState::new(countries(), window, cx);
        state.set_value(Some("jp".into()), window, cx);
        state
    });
    combobox(&state)
        .aria_label("Country")
        .disabled(true)
        .w(px(240.))
}

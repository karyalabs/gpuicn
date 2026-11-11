//! Autocomplete examples.

use crate::*;
use gpuicn::autocomplete::autocomplete;

/// Free text
///
/// Unlike a combobox, an autocomplete keeps text that is not in the list.
pub(crate) fn free_text(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("free.state", cx, |window, cx| {
        SelectState::new(
            [
                SelectItem::new("bug", "bug"),
                SelectItem::new("docs", "docs"),
                SelectItem::new("feature", "feature"),
            ],
            window,
            cx,
        )
    });
    let label = state.read(cx).value().cloned().unwrap_or_default();
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(
            autocomplete(&state)
                .aria_label("Label")
                .placeholder("Pick or type a label…")
                .w(px(240.)),
        )
        .child(format!("Label: {label}"))
}

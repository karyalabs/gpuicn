//! Textarea examples.

use crate::*;

/// Rows
///
/// The state sets how many rows show before the text scrolls.
pub(crate) fn rows(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let note = window.use_keyed_state("rows.note", cx, |window, cx| {
        TextareaState::new(window, cx)
            .rows(3)
            .placeholder("Three rows")
    });
    div().w_full().max_w(px(360.)).child(textarea(&note))
}

/// In a field
///
/// A field gives the textarea a label and a description.
pub(crate) fn in_a_field(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let feedback = window.use_keyed_state("field.feedback", cx, |window, cx| {
        TextareaState::new(window, cx)
            .rows(4)
            .placeholder("What should we change?")
    });
    div().w_full().max_w(px(360.)).child(
        Field::from_control("field.field", textarea(&feedback))
            .label("Feedback")
            .description("We read every message."),
    )
}

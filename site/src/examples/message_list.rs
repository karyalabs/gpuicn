//! Message list examples.

use crate::*;

fn row(index: usize) -> impl IntoElement {
    const LINES: [&str; 4] = [
        "Morning! Is the build green?",
        "Yes, all checks passed.",
        "Then I'll cut the release after lunch.",
        "Sounds good.",
    ];
    let (author, initials) = if index.is_multiple_of(2) {
        ("Ada", "AL")
    } else {
        ("Sam", "SK")
    };
    Message::new(("row", index))
        .avatar(Avatar::new(("row.avatar", index)).fallback(initials))
        .author(author)
        .timestamp(format!("9:{:02} AM", 10 + index))
        .child(LINES[index % LINES.len()])
}

/// Short history
///
/// The list draws row `index` on demand and starts at the newest message.
pub(crate) fn short_history(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("short.state", cx, |_, cx| MessageListState::new(24, cx));
    div()
        .w_full()
        .max_w(px(480.))
        .h(px(300.))
        .child(MessageList::new("short.list", &state, |index, _, _| row(index)))
}

/// Unread pill
///
/// `unseen` counts messages that arrived while scrolled up; the pill jumps to the end.
pub(crate) fn unread_pill(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("pill.state", cx, |_, cx| MessageListState::new(24, cx));
    let jump = state.clone();
    div().w_full().max_w(px(480.)).h(px(300.)).child(
        MessageList::new("pill.list", &state, |index, _, _| row(index))
            .unseen(3)
            .show_jump(true)
            .on_jump(move |_, cx| jump.update(cx, |state, cx| state.scroll_to_end(cx))),
    )
}

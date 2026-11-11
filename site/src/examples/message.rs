//! Message examples.

use crate::*;

/// Reactions
///
/// Each reaction shows a count, and is filled when you have reacted too.
pub(crate) fn reactions(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let reacted = view.checked;
    div().w_full().max_w(px(480.)).child(
        Message::new("reactions.message")
            .avatar(Avatar::new("reactions.avatar").fallback("AL"))
            .author("Ada")
            .timestamp("9:15 AM")
            .reactions([
                Reaction::new("👍", if reacted { 3 } else { 2 }, reacted),
                Reaction::new("🎉", 1, false),
            ])
            .on_reaction(cx.listener(|this, _: &SharedString, _, cx| {
                this.checked = !this.checked;
                cx.notify();
            }))
            .on_add_reaction(|_, _, _| {})
            .child("The release notes are ready for review."),
    )
}

/// Sending and failed
///
/// A pending message is dimmed. A failed one offers Retry and Delete.
pub(crate) fn sending_and_failed(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(480.))
        .flex()
        .flex_col()
        .child(
            Message::new("status.pending")
                .avatar(Avatar::new("status.pending.avatar").fallback("AL"))
                .author("Ada")
                .timestamp("9:16 AM")
                .status(MessageStatus::Pending)
                .child("Uploading the screenshots now."),
        )
        .child(
            Message::new("status.failed")
                .avatar(Avatar::new("status.failed.avatar").fallback("AL"))
                .author("Ada")
                .timestamp("9:17 AM")
                .status(MessageStatus::Failed)
                .on_retry(|_, _, _| {})
                .on_delete(|_, _, _| {})
                .child("Here they are."),
        )
}

/// Compact
///
/// Compact density drops the avatar and keeps the author on the message's line.
pub(crate) fn compact(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(480.))
        .flex()
        .flex_col()
        .child(
            Message::new("compact.first")
                .density(MessageDensity::Compact)
                .author("Ada")
                .timestamp("9:15")
                .child("Is the build green?"),
        )
        .child(
            Message::new("compact.second")
                .density(MessageDensity::Compact)
                .author("Sam")
                .timestamp("9:16")
                .child("Yes, all checks passed."),
        )
        .child(
            Message::new("compact.third")
                .density(MessageDensity::Compact)
                .continuation(true)
                .gutter_time("9:16")
                .child("Merging now."),
        )
}

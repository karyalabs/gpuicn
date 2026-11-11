//! Emoji picker examples.

use crate::*;

/// Small set
///
/// The application supplies the emoji, so a picker can offer just a few.
pub(crate) fn small_set(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("small.state", cx, |window, cx| {
        EmojiPickerState::new(
            [
                EmojiCategory::new(
                    "Reactions",
                    LucideIcon::FaceSlightlySmiling,
                    [
                        Emoji::new("thumbsup", "👍"),
                        Emoji::new("heart", "❤️"),
                        Emoji::new("tada", "🎉"),
                        Emoji::new("eyes", "👀"),
                        Emoji::new("rocket", "🚀"),
                        Emoji::new("thinking", "🤔"),
                    ],
                ),
                EmojiCategory::new(
                    "Status",
                    LucideIcon::Flag,
                    [
                        Emoji::new("white_check_mark", "✅").keywords(["done"]),
                        Emoji::new("warning", "⚠️"),
                        Emoji::new("x", "❌").keywords(["failed"]),
                        Emoji::new("hourglass", "⏳").keywords(["waiting"]),
                    ],
                ),
            ],
            window,
            cx,
        )
    });
    EmojiPicker::new(&state).bordered(true)
}

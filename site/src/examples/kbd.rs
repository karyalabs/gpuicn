//! Kbd examples.

use crate::*;

/// Shortcuts
///
/// `Kbd::shortcut` writes a shortcut the way the current platform does: ⌘K on macOS, Ctrl+K elsewhere.
pub(crate) fn shortcuts(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(16.))
        .child(kbd_group([Kbd::shortcut("secondary-k")], cx))
        .child(kbd_group([Kbd::shortcut("secondary-shift-p")], cx))
        .child(kbd_group([Kbd::new("Esc")], cx))
}

/// In a button
///
/// A key hint after a button's label.
pub(crate) fn in_a_button(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    Button::new("hint.search")
        .variant(ButtonVariant::Outline)
        .label("Search")
        .child(kbd_group([Kbd::shortcut("secondary-k")], cx))
}

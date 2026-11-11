//! Command examples.

use crate::*;
use gpuicn::command::{Command, CommandItem};

/// Inline
///
/// `Command` also draws in the page, with a border, when it is not in a dialog.
pub(crate) fn inline(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("inline.state", cx, |window, cx| {
        CommandState::new(
            vec![
                CommandItem::new("calendar", "Calendar")
                    .group("Suggestions")
                    .icon(LucideIcon::Calendar),
                CommandItem::new("emoji", "Search emoji")
                    .group("Suggestions")
                    .icon(LucideIcon::FaceSlightlySmiling),
                CommandItem::new("profile", "Profile")
                    .group("Settings")
                    .icon(LucideIcon::User)
                    .shortcut("secondary-p"),
                CommandItem::new("settings", "Settings")
                    .group("Settings")
                    .icon(LucideIcon::Settings)
                    .shortcut("secondary-,"),
            ],
            window,
            cx,
        )
    });
    div()
        .w_full()
        .max_w(px(380.))
        .child(Command::new(&state).bordered(true))
}

/// Keywords
///
/// Keywords match a search without showing in the row: try "theme" or "sign out".
pub(crate) fn keywords(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("keywords.state", cx, |window, cx| {
        let mut state = CommandState::new(
            vec![
                CommandItem::new("dark", "Toggle dark mode")
                    .icon(LucideIcon::Moon)
                    .keywords(["theme", "appearance"]),
                CommandItem::new("logout", "Log out")
                    .icon(LucideIcon::LogOut)
                    .keywords(["sign out", "quit"]),
                CommandItem::new("billing", "Billing")
                    .icon(LucideIcon::CreditCard)
                    .description("Plan, invoices and payment method")
                    .keywords(["upgrade", "card"]),
            ],
            window,
            cx,
        );
        state.set_placeholder("Type a command…", window, cx);
        state
    });
    div()
        .w_full()
        .max_w(px(380.))
        .child(Command::new(&state).bordered(true))
}

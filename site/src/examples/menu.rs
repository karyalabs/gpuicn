//! Menu examples.

use crate::*;

/// Checked items
///
/// `checked` draws a check mark and `radio` a dot, for settings kept in a menu.
pub(crate) fn checked_items(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let menu = window.use_keyed_state("checked.menu", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::label("checked.show", "Show"),
                MenuItem::new("checked.toolbar", "Toolbar").checked(true),
                MenuItem::new("checked.status", "Status bar").checked(false),
                MenuItem::separator(),
                MenuItem::label("checked.density", "Density"),
                MenuItem::new("checked.compact", "Compact").radio(false),
                MenuItem::new("checked.comfortable", "Comfortable").radio(true),
            ],
            cx,
        )
    });
    Menu::new(&menu, "View")
}

/// Icons and shortcuts
///
/// An item can carry an icon, a second line of text, and a shortcut hint.
pub(crate) fn icons_and_shortcuts(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let menu = window.use_keyed_state("icons.menu", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("icons.new", "New file")
                    .icon(LucideIcon::FilePlus)
                    .shortcut("secondary-n"),
                MenuItem::new("icons.open", "Open…")
                    .icon(LucideIcon::FolderOpen)
                    .shortcut("secondary-o"),
                MenuItem::new("icons.share", "Share")
                    .icon(LucideIcon::Share2)
                    .description("Anyone with the link can view"),
                MenuItem::separator(),
                MenuItem::new("icons.trash", "Move to trash")
                    .icon(LucideIcon::Trash2)
                    .disabled(true),
            ],
            cx,
        )
        .width(px(280.))
    });
    Menu::new(&menu, "File")
}

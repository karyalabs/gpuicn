//! Menubar examples.

use crate::*;

/// Icons and shortcuts
///
/// Each menu takes the same items as a dropdown menu, with icons and shortcut hints.
pub(crate) fn icons_and_shortcuts(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let file = window.use_keyed_state("icons.file", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("icons.new", "New file")
                    .icon(LucideIcon::FilePlus)
                    .shortcut("secondary-n"),
                MenuItem::new("icons.open", "Open…")
                    .icon(LucideIcon::FolderOpen)
                    .shortcut("secondary-o"),
                MenuItem::separator(),
                MenuItem::new("icons.save", "Save")
                    .icon(LucideIcon::Save)
                    .shortcut("secondary-s"),
            ],
            cx,
        )
        .width(px(220.))
    });
    let edit = window.use_keyed_state("icons.edit", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("icons.undo", "Undo")
                    .icon(LucideIcon::Undo2)
                    .shortcut("secondary-z"),
                MenuItem::new("icons.redo", "Redo")
                    .icon(LucideIcon::Redo2)
                    .shortcut("secondary-shift-z")
                    .disabled(true),
            ],
            cx,
        )
        .width(px(220.))
    });
    Menubar::new("icons.menubar", "Application menu")
        .menu(&file, "File")
        .menu(&edit, "Edit")
}

/// Settings in a menu
///
/// Checked and radio items keep view settings in the bar.
pub(crate) fn settings_in_a_menu(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let view = window.use_keyed_state("settings.view", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("settings.sidebar", "Sidebar").checked(true),
                MenuItem::new("settings.status", "Status bar").checked(false),
                MenuItem::separator(),
                MenuItem::label("settings.zoom", "Zoom"),
                MenuItem::new("settings.fit", "Fit to window").radio(true),
                MenuItem::new("settings.actual", "Actual size").radio(false),
            ],
            cx,
        )
    });
    let help = window.use_keyed_state("settings.help", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("settings.docs", "Documentation"),
                MenuItem::new("settings.shortcuts", "Keyboard shortcuts"),
            ],
            cx,
        )
    });
    Menubar::new("settings.menubar", "Application menu")
        .menu(&view, "View")
        .menu(&help, "Help")
}

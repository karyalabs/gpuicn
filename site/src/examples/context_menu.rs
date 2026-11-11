//! Context menu examples.

use crate::*;

/// On a list row
///
/// Each row wraps its content in its own menu; right click a file.
pub(crate) fn on_a_list_row(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let theme = UiTheme::read(cx).clone();
    div()
        .w(px(280.))
        .flex()
        .flex_col()
        .rounded(theme.radius.lg)
        .border_1()
        .border_color(theme.colors.border)
        .p(px(4.))
        .children(
            ["Roadmap.md", "Budget.xlsx", "Logo.svg"]
                .into_iter()
                .enumerate()
                .map(|(index, name)| {
            let menu = window.use_keyed_state(("row.menu", index), cx, |_, cx| {
                MenuState::new(
                    [
                        MenuItem::new("row.open", "Open").icon(LucideIcon::ExternalLink),
                        MenuItem::new("row.rename", "Rename").icon(LucideIcon::Pencil),
                        MenuItem::separator(),
                        MenuItem::new("row.delete", "Delete").icon(LucideIcon::Trash2),
                    ],
                    cx,
                )
            });
            gpuicn::context_menu::context_menu(
                &menu,
                format!("{name} actions"),
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .px(px(8.))
                    .py(px(6.))
                    .rounded(theme.radius.md)
                    .child(
                        lucide(LucideIcon::File)
                            .size(px(16.))
                            .text_color(theme.colors.muted_foreground),
                    )
                    .child(name),
            )
            .w_full()
        }),
        )
}

/// Checked items
///
/// A context menu takes the same items as a menu, so it can hold settings.
pub(crate) fn checked_items(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let menu = window.use_keyed_state("checked.menu", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::label("checked.columns", "Columns"),
                MenuItem::new("checked.name", "Name").checked(true),
                MenuItem::new("checked.size", "Size").checked(true),
                MenuItem::new("checked.modified", "Modified").checked(false),
            ],
            cx,
        )
    });
    gpuicn::context_menu::context_menu(
        &menu,
        "Table columns",
        div()
            .w(px(280.))
            .h(px(120.))
            .border_1()
            .border_color(UiTheme::read(cx).colors.border)
            .rounded(px(8.))
            .flex()
            .items_center()
            .justify_center()
            .child("Right click the table header"),
    )
}

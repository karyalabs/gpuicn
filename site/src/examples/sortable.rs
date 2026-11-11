//! Sortable examples.

use crate::*;

/// Flat list
///
/// One group with an empty header is a plain list; drag a row to reorder it.
pub(crate) fn flat_list(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let tasks = window.use_keyed_state("flat.tasks", cx, |_, _| {
        vec![vec!["Write the brief", "Review designs", "Ship the beta", "Plan the launch"]]
    });
    let moved = tasks.clone();
    let theme = UiTheme::read(cx).clone();
    div().w(px(280.)).child(
        Sortable::new("flat.sortable", "Tasks")
            .gap(px(6.))
            .group(
                SortableGroup::new("flat.group", "Tasks")
                    .header(|_, _| div())
                    .rows(tasks.read(cx)[0].iter().map(|&task| {
                        let theme = theme.clone();
                        SortableRow::new(task, task, move |_, _| {
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.))
                                .px(px(10.))
                                .py(px(8.))
                                .rounded(theme.radius.md)
                                .border_1()
                                .border_color(theme.colors.border)
                                .bg(theme.colors.background)
                                .child(
                                    lucide(LucideIcon::GripVertical)
                                        .size(px(14.))
                                        .text_color(theme.colors.muted_foreground),
                                )
                                .child(task)
                        })
                    })),
            )
            .on_reorder(move |movement, _, cx| {
                moved.update(cx, |tasks, cx| {
                    movement.apply(tasks);
                    cx.notify();
                })
            }),
    )
}

/// Between groups
///
/// A row can move to another group; `movement.apply` updates your lists.
pub(crate) fn between_groups(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let columns = window.use_keyed_state("groups.columns", cx, |_, _| {
        vec![
            vec!["Dark mode", "Search"],
            vec!["Onboarding"],
            vec!["Sign in"],
        ]
    });
    let moved = columns.clone();
    let theme = UiTheme::read(cx).clone();
    let names = ["To do", "Doing", "Done"];
    div().w(px(280.)).child(
        Sortable::new("groups.sortable", "Board")
            .groups(names.iter().zip(columns.read(cx)).map(|(&name, rows)| {
                SortableGroup::new(name, name).rows(rows.iter().map(|&card| {
                    let theme = theme.clone();
                    SortableRow::new(card, card, move |_, _| {
                        div()
                            .px(px(10.))
                            .py(px(6.))
                            .rounded(theme.radius.md)
                            .border_1()
                            .border_color(theme.colors.border)
                            .bg(theme.colors.background)
                            .child(card)
                    })
                }))
            }))
            .on_reorder(move |movement, _, cx| {
                moved.update(cx, |columns, cx| {
                    movement.apply(columns);
                    cx.notify();
                })
            }),
    )
}

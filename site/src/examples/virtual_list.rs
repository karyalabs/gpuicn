//! Virtual list examples.

use crate::*;
use gpuicn::virtual_list::{ListItem, VirtualList, VirtualListState};

/// Single selection
///
/// By default one row is selected at a time; arrows move it and Enter activates.
pub(crate) fn single_selection(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let rows = window
        .use_keyed_state("single.rows", cx, |_, _| {
            VirtualListState::new(
                (0..1_000usize)
                    .map(|index| ListItem::new(("customer", index), format!("Customer {}", index + 1)))
                    .collect(),
            )
            .expect("row IDs are unique")
        })
        .read(cx)
        .clone();
    let border = UiTheme::read(cx).colors.border;
    div()
        .w(px(300.))
        .h(px(260.))
        .rounded(px(8.))
        .border_1()
        .border_color(border)
        .p(px(4.))
        .child(
            VirtualList::new("single.list", "Customers", rows, |row, _, _| {
                div()
                    .w_full()
                    .truncate()
                    .child(row.item.label.clone())
                    .into_any_element()
            })
            .on_event(cx.listener(|_, _, _, cx| cx.notify())),
        )
}

/// Empty
///
/// `empty` is drawn in place of the rows when the list has none.
pub(crate) fn empty(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let rows = window
        .use_keyed_state("empty.rows", cx, |_, _| {
            VirtualListState::new(Vec::new()).expect("no rows")
        })
        .read(cx)
        .clone();
    let theme = UiTheme::read(cx).clone();
    div()
        .w(px(300.))
        .h(px(180.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme.colors.border)
        .child(
            VirtualList::new("empty.list", "Search results", rows, |row, _, _| {
                div().child(row.item.label.clone()).into_any_element()
            })
            .empty(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.colors.muted_foreground)
                    .child("No files match your search."),
            ),
        )
}

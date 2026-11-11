//! Separator examples.

use crate::*;

/// Vertical
///
/// A vertical separator takes the height of its row.
pub(crate) fn vertical(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .h(px(20.))
        .gap(px(12.))
        .child("Blog")
        .child(Separator::new("vertical.first").vertical())
        .child("Docs")
        .child(Separator::new("vertical.second").vertical())
        .child("Source")
}

/// In a list
///
/// A horizontal separator between groups of rows.
pub(crate) fn in_a_list(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(240.))
        .flex()
        .flex_col()
        .gap(px(10.))
        .child("Profile")
        .child("Billing")
        .child(Separator::new("list.rule"))
        .child("Sign out")
}

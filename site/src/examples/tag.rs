//! Tag examples.

use crate::*;

/// Status dots
///
/// A colored dot before the label, for a state.
pub(crate) fn status_dots(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(Tag::new("dots.online", "Online").dot(gpui_kit::rgb(0x22c55e)))
        .child(Tag::new("dots.away", "Away").dot(gpui_kit::rgb(0xf59e0b)))
        .child(Tag::new("dots.busy", "Busy").dot(gpui_kit::rgb(0xef4444)))
}

/// Removable
///
/// `on_remove` adds a close button. Remove the tag from your own state when it fires.
pub(crate) fn removable(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let filters = window.use_keyed_state("removable.filters", cx, |_, _| {
        vec!["is: open", "label: bug", "author: ada"]
    });
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .children(filters.read(cx).clone().into_iter().map(|filter| {
            let filters = filters.clone();
            Tag::new(filter, filter)
                .variant(TagVariant::Outline)
                .on_remove(move |_, _, cx| {
                    filters.update(cx, |filters, cx| {
                        filters.retain(|other| *other != filter);
                        cx.notify();
                    })
                })
        }))
        .when(filters.read(cx).is_empty(), |row| row.child("No filters"))
}

/// With an icon
///
/// An icon before the label, in any variant.
pub(crate) fn with_icon(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(Tag::new("icon.bot", "Bot").icon(LucideIcon::Bot))
        .child(
            Tag::new("icon.guest", "Guest")
                .variant(TagVariant::Outline)
                .icon(LucideIcon::User),
        )
        .child(
            Tag::new("icon.locked", "Locked")
                .variant(TagVariant::Destructive)
                .icon(LucideIcon::Lock),
        )
}

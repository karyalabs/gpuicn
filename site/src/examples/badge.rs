//! Badge examples.

use crate::*;

/// Variants
///
/// Five looks for a short label.
pub(crate) fn variants(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .justify_center()
        .gap(px(8.))
        .child(Badge::new("Default"))
        .child(Badge::new("Secondary").variant(BadgeVariant::Secondary))
        .child(Badge::new("Destructive").variant(BadgeVariant::Destructive))
        .child(Badge::new("Outline").variant(BadgeVariant::Outline))
        .child(Badge::new("Ghost").variant(BadgeVariant::Ghost))
}

/// Counts
///
/// `Badge::count` caps the number, so 250 with a cap of 99 reads "99+".
pub(crate) fn counts(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let row = |label: &'static str, badge: Badge| {
        div()
            .w(px(200.))
            .flex()
            .items_center()
            .justify_between()
            .child(label)
            .child(badge)
    };
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(row("Inbox", Badge::count(3, 99)))
        .child(row(
            "Drafts",
            Badge::count(42, 99).variant(BadgeVariant::Secondary),
        ))
        .child(row(
            "Spam",
            Badge::count(250, 99).variant(BadgeVariant::Destructive),
        ))
}

/// With an icon
///
/// The icon sits before the label and takes the badge's text color.
pub(crate) fn with_icon(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            Badge::new("Verified")
                .variant(BadgeVariant::Outline)
                .icon(LucideIcon::BadgeCheck),
        )
        .child(
            Badge::new("Beta")
                .variant(BadgeVariant::Secondary)
                .icon(LucideIcon::FlaskConical),
        )
        .child(
            Badge::new("Failed")
                .variant(BadgeVariant::Destructive)
                .icon(LucideIcon::CircleAlert),
        )
}

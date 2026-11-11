//! Avatar examples.

use crate::*;

/// Sizes
///
/// Small, default and large.
pub(crate) fn sizes(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(16.))
        .child(
            Avatar::new("sizes.sm")
                .size(AvatarSize::Sm)
                .aria_label("Ana Lopez")
                .fallback("AL"),
        )
        .child(
            Avatar::new("sizes.default")
                .aria_label("Ana Lopez")
                .fallback("AL"),
        )
        .child(
            Avatar::new("sizes.lg")
                .size(AvatarSize::Lg)
                .aria_label("Ana Lopez")
                .fallback("AL"),
        )
}

/// Status
///
/// A presence dot in the corner: online, away, do not disturb, or offline.
pub(crate) fn status(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div().flex().items_center().gap(px(16.)).children(
        [
            ("status.online", "Ana Lopez", "AL", AvatarStatus::Online),
            ("status.away", "Sam Kim", "SK", AvatarStatus::Away),
            ("status.dnd", "Priya Nair", "PN", AvatarStatus::Dnd),
            ("status.offline", "Lee Chen", "LC", AvatarStatus::Offline),
        ]
        .into_iter()
        .map(|(id, name, initials, status)| {
            Avatar::new(id)
                .size(AvatarSize::Lg)
                .aria_label(name)
                .fallback(initials)
                .status(status)
        }),
    )
}

/// Fallback
///
/// The fallback text shows while the image loads, and stays if it fails.
pub(crate) fn fallback(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(16.))
        .child(
            Avatar::new("fallback.broken")
                .size(AvatarSize::Lg)
                .image("https://example.invalid/missing.png")
                .aria_label("Grace Hopper")
                .fallback("GH"),
        )
        .child(
            Avatar::new("fallback.initials")
                .size(AvatarSize::Lg)
                .aria_label("Ada Lovelace")
                .fallback("AL"),
        )
}

//! Avatar Group examples.

use crate::*;
use gpuicn::avatar_group::AvatarGroup;

const PEOPLE: [&str; 6] = [
    "Ada Lovelace",
    "Grace Hopper",
    "Alan Turing",
    "Katherine Johnson",
    "Niklaus Wirth",
    "Margaret Hamilton",
];

fn group(id: &'static str, count: usize) -> AvatarGroup {
    PEOPLE
        .iter()
        .take(count)
        .fold(AvatarGroup::new(id), |group, name| {
            let initial: String = name.chars().take(1).collect();
            group.person(*name, Avatar::new(*name).fallback(initial))
        })
}

/// Overflow
///
/// `max` caps the faces shown. The rest become a count.
pub(crate) fn overflow(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(16.))
        .child(group("overflow.three", 6).max(3).aria_label("Reviewers"))
        .child(group("overflow.five", 6).max(5).aria_label("Reviewers"))
}

/// Sizes
///
/// The group sizes every avatar in it.
pub(crate) fn sizes(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(24.))
        .child(
            group("sizes.sm", 3)
                .size(AvatarSize::Sm)
                .aria_label("Members"),
        )
        .child(group("sizes.default", 3).aria_label("Members"))
        .child(
            group("sizes.lg", 3)
                .size(AvatarSize::Lg)
                .aria_label("Members"),
        )
}

//! Scroll Area examples.

use crate::*;

/// Horizontal
///
/// Set the axis to scroll a row. The scrollbar moves to the bottom edge.
pub(crate) fn horizontal(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let theme = UiTheme::read(cx);
    ScrollArea::new("horizontal.area")
        .aria_label("Albums")
        .axis(gpui_kit::Axis::Horizontal)
        .w(px(360.))
        .child(
            div()
                .flex()
                .gap(px(12.))
                .pb(px(12.))
                .children((1..=12).map(|index| {
                    div()
                        .flex_none()
                        .size(px(96.))
                        .rounded(theme.radius.lg)
                        .bg(theme.colors.muted)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(format!("Album {index}"))
                })),
        )
}

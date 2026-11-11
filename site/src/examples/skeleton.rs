//! Skeleton examples.

use crate::*;

/// Card
///
/// Size and round a skeleton like the content it stands in for.
pub(crate) fn card(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w(px(240.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            Skeleton::new("card.image")
                .w_full()
                .h(px(120.))
                .rounded(px(12.)),
        )
        .child(Skeleton::new("card.title").w(px(160.)).h(px(14.)))
        .child(Skeleton::new("card.text").w(px(110.)).h(px(12.)))
}

/// Text
///
/// Lines of different widths read as a paragraph.
pub(crate) fn text(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(10.))
        .children(
            [1., 0.92, 0.97, 0.6]
                .into_iter()
                .enumerate()
                .map(|(index, width)| {
                    Skeleton::new(("text.line", index))
                        .w(gpui_kit::relative(width))
                        .h(px(12.))
                }),
        )
}

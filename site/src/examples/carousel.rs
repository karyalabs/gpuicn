//! Carousel examples.

use crate::*;

fn slide(label: &'static str, detail: &'static str, cx: &App) -> impl IntoElement {
    let theme = UiTheme::read(cx);
    div()
        .size_full()
        .flex()
        .flex_col()
        .justify_end()
        .gap(px(4.))
        .p(px(16.))
        .rounded(theme.radius.xl)
        .border_1()
        .border_color(theme.colors.border)
        .bg(theme.colors.muted)
        .child(
            div()
                .font_weight(gpui_kit::FontWeight::MEDIUM)
                .child(label),
        )
        .child(
            div()
                .text_size(theme.text(13.))
                .text_color(theme.colors.muted_foreground)
                .child(detail),
        )
}

const SLIDES: [(&str, &str); 3] = [
    ("Dialogs", "Modal, alert and drawer."),
    ("Menus", "Dropdown, context and menubar."),
    ("Chat", "Messages, composer and reactions."),
];

/// Arrows only
///
/// Leave out the pagination when the slides are few; the arrows stop at each end.
pub(crate) fn arrows_only(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("arrows.state", cx, |_, _| CarouselState::new(3));
    Carousel::new("arrows.carousel", &state)
        .aria_label("Component groups")
        .w(px(280.))
        .child(
            CarouselContent::new(&state)
                .h(px(140.))
                .children(SLIDES.iter().enumerate().map(|(index, (label, detail))| {
                    CarouselItem::new(("arrows.slide", index), index, &state)
                        .child(slide(label, detail, cx))
                })),
        )
        .child(CarouselPrevious::new(&state))
        .child(CarouselNext::new(&state))
}

/// Dots only
///
/// A looping carousel can drop the arrows: swipe, or pick a dot.
pub(crate) fn dots_only(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("dots.state", cx, |_, _| {
        CarouselState::new(3).with_looping(true)
    });
    Carousel::new("dots.carousel", &state)
        .aria_label("Component groups")
        .w(px(280.))
        .child(
            CarouselContent::new(&state)
                .h(px(140.))
                .children(SLIDES.iter().enumerate().map(|(index, (label, detail))| {
                    CarouselItem::new(("dots.slide", index), index, &state)
                        .child(slide(label, detail, cx))
                })),
        )
        .child(CarouselPagination::new().children((0..3).map(|index| {
            CarouselPaginationItem::new(("dots.page", index), index, &state)
        })))
}

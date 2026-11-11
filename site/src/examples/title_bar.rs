//! Title bar examples.

use crate::*;

fn frame(cx: &App) -> gpui_kit::Div {
    let theme = UiTheme::read(cx);
    div()
        .w_full()
        .max_w(px(480.))
        .rounded(theme.radius.xl)
        .border_1()
        .border_color(theme.colors.border)
        .overflow_hidden()
        .bg(theme.colors.background)
}

/// Page colors
///
/// `page` draws the bar in the page's colors, for a window with no sidebar under it.
pub(crate) fn page_colors(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let theme = UiTheme::read(cx).clone();
    frame(cx)
        .child(
            TitleBar::new()
                .page(true)
                .rounded_t(theme.radius.xl - px(1.))
                .child(div().flex_1())
                .child(
                    div()
                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                        .child("Settings"),
                )
                .child(div().flex_1()),
        )
        .child(
            div()
                .h(px(100.))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(13.))
                .text_color(theme.colors.muted_foreground)
                .child("A settings window has only a title."),
        )
}

/// With actions
///
/// Buttons in the bar stay clickable; the space around them drags the window.
pub(crate) fn with_actions(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let theme = UiTheme::read(cx).clone();
    let icon = |id: &'static str, label: &'static str, icon: LucideIcon| {
        Button::new(id)
            .size(ButtonSize::IconSm)
            .variant(ButtonVariant::Ghost)
            .aria_label(label)
            .child(
                lucide(icon)
                    .size(px(16.))
                    .text_color(theme.colors.muted_foreground),
            )
    };
    frame(cx)
        .child(
            TitleBar::new()
                .rounded_t(theme.radius.xl - px(1.))
                .child(icon("actions.back", "Back", LucideIcon::ChevronLeft))
                .child(icon("actions.forward", "Forward", LucideIcon::ChevronRight))
                .child(
                    div()
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .child("Documents"),
                )
                .child(div().flex_1())
                .child(icon("actions.search", "Search", LucideIcon::Search))
                .child(icon("actions.share", "Share", LucideIcon::Share2))
                .px(px(8.)),
        )
        .child(
            div()
                .h(px(100.))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(13.))
                .text_color(theme.colors.muted_foreground)
                .child("Navigation on the left, actions on the right."),
        )
}

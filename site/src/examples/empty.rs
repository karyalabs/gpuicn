//! Empty examples.

use crate::*;

/// Without an action
///
/// The header alone, for a list that fills by itself.
pub(crate) fn without_an_action(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    // The framed media slot does not size or color an icon; do both here.
    let color = UiTheme::read(cx).colors.foreground;
    Empty::new().w_full().max_w(px(420.)).header(
        EmptyHeader::new()
            .media(
                EmptyMedia::new()
                    .with_variant(EmptyMediaVariant::Icon)
                    .child(lucide(LucideIcon::Inbox).size(px(16.)).text_color(color)),
            )
            .title(EmptyTitle::new().child("You are all caught up"))
            .description(EmptyDescription::new().child("New messages will show up here.")),
    )
}

/// Two actions
///
/// The content slot holds whatever comes next, such as a pair of buttons.
pub(crate) fn two_actions(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    // The framed media slot does not size or color an icon; do both here.
    let color = UiTheme::read(cx).colors.foreground;
    Empty::new()
        .w_full()
        .max_w(px(420.))
        .border_1()
        .header(
            EmptyHeader::new()
                .media(
                    EmptyMedia::new()
                        .with_variant(EmptyMediaVariant::Icon)
                        .child(lucide(LucideIcon::SearchX).size(px(16.)).text_color(color)),
                )
                .title(EmptyTitle::new().child("No results"))
                .description(
                    EmptyDescription::new().child("Nothing matches \"quarterly report\"."),
                ),
        )
        .content(
            EmptyContent::new().child(
                div()
                    .flex()
                    .gap(px(8.))
                    .child(Button::new("two.clear").label("Clear search"))
                    .child(
                        Button::new("two.new")
                            .variant(ButtonVariant::Outline)
                            .label("New document"),
                    ),
            ),
        )
}

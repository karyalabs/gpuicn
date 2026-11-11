//! Preview card examples.

use crate::*;

/// Profile
///
/// The popup takes any content: here an avatar, a name and a short bio.
pub(crate) fn profile(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    preview_card("profile")
        .trigger(
            Button::new("profile.trigger")
                .variant(ButtonVariant::Link)
                .label("@ada"),
        )
        .content(|_, _, cx| {
            let muted = UiTheme::read(cx).colors.muted_foreground;
            preview_card_popup("profile.popup", cx).child(
                div()
                    .flex()
                    .gap(px(12.))
                    .child(Avatar::new("profile.avatar").fallback("AL"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .child(
                                div()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .child("Ada Lovelace"),
                            )
                            .child("Writes the first programs. Likes engines.")
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .text_color(muted)
                                    .child("Joined December 1842"),
                            ),
                    ),
            )
        })
}

/// In a sentence
///
/// A link trigger sits in running text; rest the pointer on it to preview the page.
pub(crate) fn in_a_sentence(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(4.))
        .child("Read the")
        .child(
            preview_card("sentence")
                .trigger(
                    Button::new("sentence.trigger")
                        .variant(ButtonVariant::Link)
                        .px(px(0.))
                        .label("installation guide"),
                )
                .content(|_, _, cx| {
                    preview_card_popup("sentence.popup", cx).child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .child(
                                div()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .child("Installation"),
                            )
                            .child("Add the CLI, pick a theme, and copy in your first component."),
                    )
                }),
        )
        .child("before you start.")
}

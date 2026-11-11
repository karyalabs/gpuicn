//! Popover examples.

use crate::*;

/// Text only
///
/// A title and a description are enough for a short explanation.
pub(crate) fn text_only(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    popover("text")
        .trigger(popover_trigger("text.trigger", cx).child("What is a seat?"))
        .content(|_, _, cx| {
            popover_popup("text.popup", "Seats", cx)
                .child(popover_title(cx).child("Seats"))
                .child(
                    popover_description(cx)
                        .child("A seat is one person who can sign in. Guests do not use a seat."),
                )
        })
}

/// With actions
///
/// A popover can hold buttons, for a small confirmation beside its trigger.
pub(crate) fn with_actions(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    popover("actions")
        .trigger(popover_trigger("actions.trigger", cx).child("Share"))
        .content(|_, _, cx| {
            popover_popup("actions.popup", "Share this page", cx)
                .child(popover_title(cx).child("Share this page"))
                .child(popover_description(cx).child("Anyone with the link can view it."))
                .child(
                    div()
                        .flex()
                        .gap(px(8.))
                        .child(Button::new("actions.copy").label("Copy link"))
                        .child(
                            Button::new("actions.email")
                                .variant(ButtonVariant::Outline)
                                .label("Email"),
                        ),
                )
        })
}

/// Opens above
///
/// `placement` sets the preferred side; the popup flips when there is no room.
pub(crate) fn opens_above(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div().pt(px(120.)).child(
        hover_popover("above", "Ada Lovelace, away")
            .placement(gpui_kit::base::Placement::Top)
            .trigger(
                Avatar::new("above.avatar")
                    .fallback("AL")
                    .status(AvatarStatus::Away),
            )
            .content(|_, cx| {
                popover_popup("above.card", "Ada Lovelace", cx)
                    .child(popover_title(cx).child("Ada Lovelace"))
                    .child(popover_description(cx).child("Away · back at 2 PM"))
            }),
    )
}

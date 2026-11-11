//! Fieldset examples.

use crate::*;

/// Label legend
///
/// The `Label` variant is a compact heading for a small group of choices.
pub(crate) fn label_legend(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let mentions = cx.listener(|this, checked: &bool, _, cx| {
        this.checked = *checked;
        cx.notify();
    });
    let replies = cx.listener(|this, checked: &bool, _, cx| {
        this.pressed = *checked;
        cx.notify();
    });
    let row = |checkbox: Checkbox, label: &'static str| {
        div()
            .flex()
            .items_center()
            .gap(px(10.))
            .child(checkbox.aria_label(label))
            .child(label)
    };
    fieldset("notify", cx)
        .w(px(300.))
        .aria_label("Notify me about")
        .child(fieldset_legend(FieldsetLegendVariant::Label, cx).child("Notify me about"))
        .child(row(
            Checkbox::new("notify.mentions")
                .checked(view.checked)
                .on_change(mentions),
            "Mentions",
        ))
        .child(row(
            Checkbox::new("notify.replies")
                .checked(view.pressed)
                .on_change(replies),
            "Replies to my threads",
        ))
}

/// With a description
///
/// Text under the legend explains the whole group once.
pub(crate) fn with_a_description(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let muted = UiTheme::read(cx).colors.muted_foreground;
    let card = demo_input("billing.card", "", "1234 5678 9012 3456", window, cx);
    let name = demo_input("billing.name", "", "Ada Lovelace", window, cx);
    fieldset("billing", cx)
        .w(px(300.))
        .aria_label("Payment method")
        .child(fieldset_legend(FieldsetLegendVariant::Legend, cx).child("Payment method"))
        .child(
            div()
                .text_size(px(13.))
                .text_color(muted)
                .child("Your card is charged when the order ships."),
        )
        .child(Field::new("billing-card", &card).label("Card number"))
        .child(Field::new("billing-name", &name).label("Name on card"))
}

//! Field examples.

use crate::*;

/// Required
///
/// `required` marks the label. The field wires the label and the description to the control.
pub(crate) fn required(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let name = demo_input("required.name", "", "Ada Lovelace", window, cx);
    div().w_full().max_w(px(300.)).child(
        Field::new("required.field", &name)
            .label("Full name")
            .description("As it appears on your passport.")
            .required(true),
    )
}

/// With an error
///
/// An error replaces the description and marks the control invalid.
pub(crate) fn with_an_error(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = demo_input("error.handle", "ada lovelace", "", window, cx);
    div().w_full().max_w(px(300.)).child(
        Field::new("error.field", &handle)
            .label("Username")
            .error("Usernames cannot contain spaces."),
    )
}

/// Horizontal
///
/// The label sits beside the control, which suits a checkbox.
pub(crate) fn horizontal(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let this = cx.entity().downgrade();
    div().w_full().max_w(px(340.)).child(
        Field::from_control(
            "horizontal.field",
            Checkbox::new("horizontal.box")
                .checked(view.checked)
                .on_change(move |checked, _, cx| {
                    this.update(cx, |view, cx| {
                        view.checked = *checked;
                        cx.notify();
                    })
                    .ok();
                }),
        )
        .orientation(FieldOrientation::Horizontal)
        .label("Share usage data")
        .description("Anonymous, and only crash counts."),
    )
}

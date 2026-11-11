//! Checkbox examples.

use crate::*;

/// With a description
///
/// The checkbox is only the box. Lay out its label and help text beside it.
pub(crate) fn with_description(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let muted = UiTheme::read(cx).colors.muted_foreground;
    let this = cx.entity().downgrade();
    div()
        .flex()
        .items_start()
        .gap(px(10.))
        .child(
            Checkbox::new("description.box")
                .checked(view.checked)
                .aria_label("Email me about new releases")
                .on_change(move |checked, _, cx| {
                    this.update(cx, |view, cx| {
                        view.checked = *checked;
                        cx.notify();
                    })
                    .ok();
                }),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.))
                .child("Email me about new releases")
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(muted)
                        .child("About one message a month."),
                ),
        )
}

/// Indeterminate
///
/// A parent box shows a dash when only some of its children are checked.
pub(crate) fn indeterminate(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(checkbox_row(
            Checkbox::new("some.all")
                .indeterminate(true)
                .aria_label("All files"),
            "All files",
        ))
        .child(
            div()
                .pl(px(24.))
                .flex()
                .flex_col()
                .gap(px(10.))
                .child(checkbox_row(
                    Checkbox::new("some.readme")
                        .checked(true)
                        .aria_label("README.md"),
                    "README.md",
                ))
                .child(checkbox_row(
                    Checkbox::new("some.license").aria_label("LICENSE"),
                    "LICENSE",
                )),
        )
}

/// Disabled
///
/// Disabled boxes keep their value but cannot be changed or focused.
pub(crate) fn disabled(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(checkbox_row(
            Checkbox::new("off.unchecked")
                .disabled(true)
                .aria_label("Unchecked"),
            "Unchecked",
        ))
        .child(checkbox_row(
            Checkbox::new("off.checked")
                .checked(true)
                .disabled(true)
                .aria_label("Checked"),
            "Checked",
        ))
}

//! Dialog examples.

use crate::*;

/// With a cancel button
///
/// `dialog_close` closes without acting. `dialog_action` closes after your handler runs.
pub(crate) fn with_a_cancel_button(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = window
        .use_keyed_state("cancel.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let popup = dialog_popup("cancel.popup", "Rename project", cx)
        .child(dialog_title("cancel.title", cx).child("Rename project"))
        .child(
            dialog_description("cancel.description", cx)
                .child("Links to the old name keep working."),
        )
        .child(
            dialog_footer(cx)
                .child(dialog_close("cancel.close", cx).label("Cancel"))
                .child(dialog_action("cancel.save", &handle).label("Rename")),
        );
    div()
        .child(
            dialog_trigger("cancel.trigger", &handle)
                .variant(ButtonVariant::Outline)
                .label("Rename project"),
        )
        .child(dialog("cancel.dialog", &handle, popup, window, cx))
}

/// Long content
///
/// A header keeps the title in place above a longer body.
pub(crate) fn long_content(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = window
        .use_keyed_state("long.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let muted = UiTheme::read(cx).colors.muted_foreground;
    let popup = dialog_popup("long.popup", "What's new", cx)
        .child(
            dialog_header(cx)
                .child(dialog_title("long.title", cx).child("What's new"))
                .child(dialog_description("long.description", cx).child("Version 2.4")),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .text_size(px(14.))
                .text_color(muted)
                .child("• Search now covers archived projects.")
                .child("• Exports keep their column widths.")
                .child("• The sidebar remembers what you collapsed."),
        )
        .child(dialog_footer(cx).child(dialog_action("long.done", &handle).label("Got it")));
    div()
        .child(dialog_trigger("long.trigger", &handle).label("See what's new"))
        .child(dialog("long.dialog", &handle, popup, window, cx))
}

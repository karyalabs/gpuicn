//! Alert Dialog examples.

use crate::*;

/// Destructive
///
/// An alert dialog only closes through its buttons. Style the action to match what it does.
pub(crate) fn destructive(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = window
        .use_keyed_state("destructive.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let popup = dialog_popup("destructive.popup", "Delete 3 files?", cx)
        .child(dialog_title("destructive.title", cx).child("Delete 3 files?"))
        .child(
            dialog_description("destructive.description", cx)
                .child("They are removed for everyone and cannot be restored."),
        )
        .child(
            dialog_footer(cx)
                .child(alert_dialog_cancel("destructive.cancel").label("Keep files"))
                .child(
                    alert_dialog_action("destructive.confirm")
                        .variant(ButtonVariant::Destructive)
                        .label("Delete"),
                ),
        );
    div()
        .child(
            dialog_trigger("destructive.trigger", &handle)
                .variant(ButtonVariant::Destructive)
                .label("Delete files"),
        )
        .child(alert_dialog(
            "destructive.dialog",
            &handle,
            popup,
            window,
            cx,
        ))
}

/// One button
///
/// With only an action, the dialog is a notice the person has to acknowledge.
pub(crate) fn one_button(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = window
        .use_keyed_state("one.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let popup = dialog_popup("one.popup", "Session expired", cx)
        .child(dialog_title("one.title", cx).child("Session expired"))
        .child(dialog_description("one.description", cx).child("Sign in again to continue."))
        .child(dialog_footer(cx).child(alert_dialog_action("one.confirm").label("Sign in")));
    div()
        .child(
            dialog_trigger("one.trigger", &handle)
                .variant(ButtonVariant::Outline)
                .label("Show notice"),
        )
        .child(alert_dialog("one.dialog", &handle, popup, window, cx))
}

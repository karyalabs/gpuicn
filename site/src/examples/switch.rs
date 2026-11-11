//! Switch examples.

use crate::*;

/// Settings rows
///
/// A label on the left and the switch on the right, as in a settings list.
pub(crate) fn settings(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let this = cx.entity().downgrade();
    let row = |label: &'static str, switch: Switch| {
        div()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .child(label)
            .child(switch)
    };
    div()
        .w_full()
        .max_w(px(300.))
        .flex()
        .flex_col()
        .gap(px(14.))
        .child(row(
            "Desktop notifications",
            Switch::new("settings.notifications")
                .checked(view.checked)
                .aria_label("Desktop notifications")
                .on_change(move |checked, _, cx| {
                    this.update(cx, |view, cx| {
                        view.checked = *checked;
                        cx.notify();
                    })
                    .ok();
                }),
        ))
        .child(row(
            "Play a sound",
            Switch::new("settings.sound")
                .checked(true)
                .aria_label("Play a sound"),
        ))
}

/// Disabled
///
/// Disabled switches keep their value but cannot be changed or focused.
pub(crate) fn disabled(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(14.))
        .child(switch_row(
            Switch::new("off.unchecked")
                .disabled(true)
                .aria_label("Off"),
            "Off",
        ))
        .child(switch_row(
            Switch::new("off.checked")
                .checked(true)
                .disabled(true)
                .aria_label("On"),
            "On",
        ))
}

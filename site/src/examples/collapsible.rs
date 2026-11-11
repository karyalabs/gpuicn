//! Collapsible examples.

use crate::*;

/// Text trigger
///
/// The trigger is a button, so it can carry a label as well as an icon.
pub(crate) fn text_trigger(
    view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let open = view.checked;
    let muted = UiTheme::read(cx).colors.muted_foreground;
    let border = UiTheme::read(cx).colors.border;
    div().w(px(320.)).h(px(180.)).child(
        collapsible("text.collapsible", open, window, cx)
            .gap(px(8.))
            .child("Your order ships on Friday.")
            .child(
                collapsible_trigger("text.trigger", open, cx)
                    .px(px(10.))
                    .py(px(4.))
                    .rounded(px(8.))
                    .border_1()
                    .border_color(border)
                    .child(if open { "Hide details" } else { "Show details" })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.checked = !this.checked;
                        cx.notify();
                    })),
            )
            .content(
                collapsible_content(cx)
                    .text_color(muted)
                    .child("Two items, sent by courier. A signature is needed on delivery."),
            ),
    )
}

/// Show more
///
/// `ShowMore` clips long content to a height and fades it out above a button.
pub(crate) fn show_more(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let lines = [
        "Fixed the popover flipping at the window edge.",
        "Menus keep their highlight after a submenu closes.",
        "Inputs announce their error text to screen readers.",
        "Dialogs return focus to the control that opened them.",
        "The calendar starts the week from the locale.",
        "Tooltips wait for the pointer to rest before opening.",
        "Sliders snap to their step when dragged.",
    ];
    div().w(px(340.)).h(px(260.)).child(
        ShowMore::new(
            "more.notes",
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .children(lines.map(|line| div().child(line))),
        )
        .max_height(px(96.))
        .expanded(view.pressed)
        .on_toggle(cx.listener(|this, expanded: &bool, _, cx| {
            this.pressed = *expanded;
            cx.notify();
        })),
    )
}

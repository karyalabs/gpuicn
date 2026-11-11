//! Accordion examples.

use crate::*;

/// Several open
///
/// You hold the open state, so any number of sections can be open at once.
pub(crate) fn several_open(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let open = window.use_keyed_state("several.open", cx, |_, _| [true, true, false]);
    let sections = [
        ("Shipping", "Orders leave the warehouse within two days."),
        ("Returns", "Send anything back within 30 days."),
        ("Warranty", "Hardware is covered for two years."),
    ];
    accordion("several.accordion", cx).w_full().max_w(px(420.)).children(
        sections
            .into_iter()
            .enumerate()
            .map(|(index, (title, text))| {
                let is_open = open.read(cx)[index];
                let open = open.clone();
                accordion_item(cx)
                    .open(is_open)
                    .header(accordion_header(
                        accordion_trigger(("several.trigger", index), is_open, false, cx)
                            .child(title)
                            .on_click(move |_, _, cx| {
                                open.update(cx, |open, cx| {
                                    open[index] = !open[index];
                                    cx.notify();
                                })
                            }),
                    ))
                    .panel(accordion_content(
                        ("several.panel", index),
                        is_open,
                        text,
                        window,
                        cx,
                    ))
            }),
    )
}

/// Disabled section
///
/// A disabled trigger stays in the list but cannot be opened.
pub(crate) fn disabled_section(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let open = window.use_keyed_state("off.open", cx, |_, _| false);
    let is_open = *open.read(cx);
    accordion("off.accordion", cx)
        .w_full()
        .max_w(px(420.))
        .child(
            accordion_item(cx)
                .open(is_open)
                .header(accordion_header(
                    accordion_trigger("off.plan", is_open, false, cx)
                        .child("Your plan")
                        .on_click(move |_, _, cx| {
                            open.update(cx, |open, cx| {
                                *open = !*open;
                                cx.notify();
                            })
                        }),
                ))
                .panel(accordion_content(
                    "off.plan.panel",
                    is_open,
                    "Team, billed yearly.",
                    window,
                    cx,
                )),
        )
        .child(
            accordion_item(cx)
                .header(accordion_header(
                    accordion_trigger("off.invoices", false, true, cx)
                        .child("Invoices (owners only)"),
                ))
                .panel(accordion_content(
                    "off.invoices.panel",
                    false,
                    "",
                    window,
                    cx,
                )),
        )
}

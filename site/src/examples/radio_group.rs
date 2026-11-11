//! Radio Group examples.

use crate::*;

/// Plans
///
/// The group holds the value. Each item reports its own value when picked.
pub(crate) fn plans(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = window.use_keyed_state("plans.value", cx, |_, _| SharedString::from("team"));
    let selected = value.read(cx).clone();
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            RadioGroup::new("plans.group")
                .aria_label("Plan")
                .value(selected.clone())
                .item(RadioItem::new("plans.free", "free").label("Free"))
                .item(RadioItem::new("plans.team", "team").label("Team"))
                .item(RadioItem::new("plans.business", "business").label("Business"))
                .on_change(move |next, _, cx| {
                    value.update(cx, |value, cx| {
                        *value = next.clone();
                        cx.notify();
                    })
                }),
        )
        .child(format!("Plan: {selected}"))
}

/// Disabled
///
/// Disable the whole group, or one item.
pub(crate) fn disabled(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    RadioGroup::new("off.group")
        .aria_label("Region")
        .value("eu")
        .disabled(true)
        .item(RadioItem::new("off.us", "us").label("United States"))
        .item(RadioItem::new("off.eu", "eu").label("Europe"))
}

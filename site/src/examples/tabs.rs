//! Tabs examples.

use crate::*;

/// Line
///
/// The line variant underlines the selected tab instead of filling it.
pub(crate) fn line(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = window.use_keyed_state("line.value", cx, |_, _| SharedString::from("overview"));
    let selected = value.read(cx).clone();
    div()
        .w_full()
        .max_w(px(360.))
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(
            Tabs::new("line.tabs")
                .variant(TabsVariant::Line)
                .aria_label("Project")
                .selected(selected.clone())
                .item(Tab::new("line.overview", "overview", "Overview"))
                .item(Tab::new("line.activity", "activity", "Activity"))
                .item(Tab::new("line.settings", "settings", "Settings"))
                .on_change(move |next, _, cx| {
                    value.update(cx, |value, cx| {
                        *value = next.clone();
                        cx.notify();
                    })
                }),
        )
        .child(
            tabs_content("line.panel", selected.clone(), cx)
                .child(format!("The {selected} panel.")),
        )
}

/// Disabled tab
///
/// A disabled tab stays in the row but is skipped by the pointer and the arrow keys.
pub(crate) fn disabled_tab(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = window.use_keyed_state("off.value", cx, |_, _| SharedString::from("free"));
    let selected = value.read(cx).clone();
    Tabs::new("off.tabs")
        .aria_label("Plan")
        .selected(selected)
        .item(Tab::new("off.free", "free", "Free"))
        .item(Tab::new("off.team", "team", "Team"))
        .item(Tab::new("off.enterprise", "enterprise", "Enterprise").disabled(true))
        .on_change(move |next, _, cx| {
            value.update(cx, |value, cx| {
                *value = next.clone();
                cx.notify();
            })
        })
}

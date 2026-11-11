//! Meter examples.

use crate::*;

/// Levels
///
/// A meter shows a measurement, such as space used. For work in progress, use Progress.
pub(crate) fn levels(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let row = |id: &'static str, label: &'static str, value: f64| {
        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(format!("{label} · {value:.0}%"))
            .child(
                Meter::new(id)
                    .value(value)
                    .aria_label(format!("{label}: {value:.0} percent")),
            )
    };
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(row("levels.cpu", "CPU", 24.))
        .child(row("levels.memory", "Memory", 61.))
        .child(row("levels.disk", "Disk", 92.))
}

/// Custom range
///
/// `range` sets the ends, so the value can be in your own units.
pub(crate) fn custom_range(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(6.))
        .child("7 of 10 seats used")
        .child(
            Meter::new("range.seats")
                .range(0., 10.)
                .value(7.)
                .aria_label("7 of 10 seats used"),
        )
}

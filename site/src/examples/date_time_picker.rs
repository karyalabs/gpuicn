//! Date and time picker examples.

use crate::*;
use gpuicn::calendar::CalendarDate;
use gpuicn::date_time_picker::{CalendarTime, DateTimePicker, DateTimeState, HourCycle};

/// Placeholder
///
/// With no date chosen, the button shows the placeholder.
pub(crate) fn placeholder(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("placeholder.state", cx, |_, cx| DateTimeState::new(cx));
    div().w(px(280.)).child(
        DateTimePicker::new("placeholder.picker", &state)
            .placeholder("Schedule for later")
            .aria_label("Send at")
            .w_full(),
    )
}

/// 24-hour clock
///
/// `set_hour_cycle` picks a 12 or 24-hour clock, usually from the locale.
pub(crate) fn twenty_four_hour(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("clock.state", cx, |_, cx| {
        let mut state = DateTimeState::new(cx);
        state.set_hour_cycle(HourCycle::for_locale("en-GB"), cx);
        state.set_value(
            CalendarDate::new(2026, 3, 16),
            CalendarTime::new(17, 45).unwrap(),
            cx,
        );
        state
    });
    div().w(px(280.)).child(
        DateTimePicker::new("clock.picker", &state)
            .aria_label("Departure")
            .w_full(),
    )
}

/// Disabled
///
/// A disabled picker keeps its value and does not open.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("disabled.state", cx, |_, cx| {
        let mut state = DateTimeState::new(cx);
        state.set_hour_cycle(HourCycle::for_locale("en-US"), cx);
        state.set_value(
            CalendarDate::new(2026, 3, 16),
            CalendarTime::new(9, 30).unwrap(),
            cx,
        );
        state
    });
    div().w(px(280.)).child(
        DateTimePicker::new("disabled.picker", &state)
            .aria_label("Send at")
            .disabled(true)
            .w_full(),
    )
}

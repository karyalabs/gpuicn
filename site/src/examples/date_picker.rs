//! Date Picker examples.

use crate::*;
use gpuicn::calendar::{CalendarDate, CalendarState};
use gpuicn::date_picker::DatePicker;

/// Placeholder
///
/// The placeholder shows until a date is picked.
pub(crate) fn placeholder(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("placeholder.state", cx, |_, cx| CalendarState::new(cx));
    DatePicker::new("placeholder.picker", &state)
        .placeholder("Pick a due date")
        .aria_label("Due date")
        .w(px(260.))
}

/// Custom format
///
/// `format` writes the picked date your way. This one is ISO 8601.
pub(crate) fn custom_format(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("format.state", cx, |_, cx| {
        let mut state = CalendarState::new(cx);
        state.set_selected(CalendarDate::new(2026, 3, 14), cx);
        state
    });
    DatePicker::new("format.picker", &state)
        .aria_label("Start date")
        .format(|date| format!("{}-{:02}-{:02}", date.year(), date.month(), date.day()).into())
        .w(px(260.))
}

/// Disabled
///
/// A disabled picker shows its date but does not open.
pub(crate) fn disabled(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("off.state", cx, |_, cx| {
        let mut state = CalendarState::new(cx);
        state.set_selected(CalendarDate::new(2026, 3, 14), cx);
        state
    });
    DatePicker::new("off.picker", &state)
        .aria_label("Billing date")
        .disabled(true)
        .w(px(260.))
}

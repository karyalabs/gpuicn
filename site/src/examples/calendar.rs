//! Calendar examples.

use crate::*;
use gpuicn::calendar::{Calendar, CalendarDate, CalendarState, Weekday};

fn framed(calendar: Calendar, cx: &App) -> impl IntoElement {
    let theme = UiTheme::read(cx);
    div()
        .rounded(theme.radius.lg)
        .border_1()
        .border_color(theme.colors.border)
        .bg(theme.colors.card)
        .child(calendar)
}

/// Limits
///
/// `set_limits` disables days outside a range. Here only the next two weeks can be picked.
pub(crate) fn limits(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("limits.state", cx, |_, cx| {
        let mut state = CalendarState::new(cx);
        let today = CalendarDate::new(2026, 3, 14).unwrap();
        state.set_today(today, cx);
        state.set_limits(Some(today), Some(today.add_days(14)), cx);
        state
    });
    framed(Calendar::new(&state), cx)
}

/// Monday first
///
/// The week starts on Sunday unless you set the first weekday.
pub(crate) fn monday_first(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("monday.state", cx, |_, cx| {
        let mut state = CalendarState::new(cx);
        state.set_today(CalendarDate::new(2026, 3, 14).unwrap(), cx);
        state.set_first_weekday(Weekday::Monday, cx);
        state
    });
    framed(Calendar::new(&state), cx)
}

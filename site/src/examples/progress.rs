//! Progress examples.

use crate::*;

/// Determinate
///
/// A value from 0 to 100. The button adds ten.
pub(crate) fn determinate(
    view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = (30 + view.count * 10) % 110;
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(format!("Uploading · {value}%"))
        .child(
            Progress::new("determinate.bar")
                .value(value as f64)
                .label("Uploading"),
        )
        .child(
            div().flex().child(
                Button::new("determinate.more")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Outline)
                    .label("Add 10%")
                    .on_click(cx.listener(|view, _, _, cx| {
                        view.count += 1;
                        cx.notify();
                    })),
            ),
        )
}

/// Indeterminate
///
/// For work with no known length. The bar slides until you give it a value.
pub(crate) fn indeterminate(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(320.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child("Connecting…")
        .child(
            Progress::new("indeterminate.bar")
                .indeterminate()
                .label("Connecting"),
        )
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
        .gap(px(12.))
        .child("Step 3 of 5")
        .child(
            Progress::new("range.bar")
                .range(0., 5.)
                .value(3.)
                .label("Setup"),
        )
}

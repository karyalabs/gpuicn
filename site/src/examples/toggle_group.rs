//! Toggle Group examples.

use crate::*;

/// Single
///
/// One value at a time. Ignore an empty change to keep one item pressed.
pub(crate) fn single(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value =
        window.use_keyed_state("single.value", cx, |_, _| vec![SharedString::from("left")]);
    let color = UiTheme::read(cx).colors.foreground;
    let item = |name: &'static str, label: &'static str, icon: LucideIcon| {
        ToggleGroupItem::new(name, name)
            .aria_label(label)
            .child(lucide(icon).size(px(16.)).text_color(color))
    };
    ToggleGroup::new("single.group")
        .aria_label("Alignment")
        .value(value.read(cx).clone())
        .item(item("left", "Align left", LucideIcon::TextAlignStart))
        .item(item("center", "Align center", LucideIcon::TextAlignCenter))
        .item(item("right", "Align right", LucideIcon::TextAlignEnd))
        .on_change(move |next, _, cx| {
            if !next.is_empty() {
                value.update(cx, |value, cx| {
                    *value = next.to_vec();
                    cx.notify();
                })
            }
        })
}

/// Multiple
///
/// `multiple` lets any number of items be pressed.
pub(crate) fn multiple(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = window.use_keyed_state("multiple.value", cx, |_, _| {
        vec![SharedString::from("bold"), SharedString::from("italic")]
    });
    let item = |name: &'static str, label: &'static str| {
        ToggleGroupItem::new(name, name)
            .aria_label(label)
            .child(label)
    };
    ToggleGroup::new("multiple.group")
        .aria_label("Text styles")
        .multiple(true)
        .value(value.read(cx).clone())
        .item(item("bold", "Bold"))
        .item(item("italic", "Italic"))
        .item(item("underline", "Underline"))
        .on_change(move |next, _, cx| {
            value.update(cx, |value, cx| {
                *value = next.to_vec();
                cx.notify();
            })
        })
}

/// Segmented
///
/// `segmented` draws the group as one control with a sliding thumb.
pub(crate) fn segmented(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = window.use_keyed_state("segmented.value", cx, |_, _| {
        vec![SharedString::from("month")]
    });
    let item = |name: &'static str, label: &'static str| {
        ToggleGroupItem::new(name, name)
            .aria_label(label)
            .child(label)
    };
    ToggleGroup::new("segmented.group")
        .aria_label("Range")
        .segmented(true)
        .value(value.read(cx).clone())
        .item(item("day", "Day"))
        .item(item("week", "Week"))
        .item(item("month", "Month"))
        .on_change(move |next, _, cx| {
            if !next.is_empty() {
                value.update(cx, |value, cx| {
                    *value = next.to_vec();
                    cx.notify();
                })
            }
        })
}

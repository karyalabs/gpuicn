//! The toggle group catalog preview.

use crate::*;

impl Showcase {
    pub(crate) fn toggle_group_preview(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let value =
            window.use_keyed_state("format-values", cx, |_, _| vec![SharedString::from("bold")]);
        let density = window.use_keyed_state("density-value", cx, |_, _| {
            vec![SharedString::from("comfortable")]
        });
        let segmented = ToggleGroup::new("density")
            .aria_label("Density")
            .segmented(true)
            .value(density.read(cx).clone())
            .item(
                ToggleGroupItem::new("compact", "compact")
                    .aria_label("Compact")
                    .child("Compact"),
            )
            .item(
                ToggleGroupItem::new("comfortable", "comfortable")
                    .aria_label("Comfortable")
                    .child("Comfortable"),
            )
            .item(
                ToggleGroupItem::new("roomy", "roomy")
                    .aria_label("Roomy")
                    .child("Roomy"),
            )
            .on_change(move |next, _, cx| {
                if !next.is_empty() {
                    density.update(cx, |value, cx| {
                        *value = next.to_vec();
                        cx.notify();
                    })
                }
            });
        let format = ToggleGroup::new("format")
            .aria_label("Text styles")
            .multiple(true)
            .value(value.read(cx).clone())
            .item(
                ToggleGroupItem::new("bold", "bold")
                    .aria_label("Bold")
                    .child("Bold"),
            )
            .item(
                ToggleGroupItem::new("italic", "italic")
                    .aria_label("Italic")
                    .child("Italic"),
            )
            .item(
                ToggleGroupItem::new("underline", "underline")
                    .aria_label("Underline")
                    .child("Underline"),
            )
            .on_change(move |next, _, cx| {
                value.update(cx, |value, cx| {
                    *value = next.to_vec();
                    cx.notify();
                })
            });
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0))
            .child(format)
            .child(segmented)
    }
}

pub(crate) fn preview(view: &Showcase, window: &mut Window, cx: &mut Context<Showcase>) -> gpui_kit::AnyElement {
    view.toggle_group_preview(window, cx).into_any_element()
}

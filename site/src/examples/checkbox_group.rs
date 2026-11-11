//! Checkbox Group examples.

use crate::*;

/// Several checked
///
/// The group's value is the list of checked items.
pub(crate) fn several_checked(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let value = window.use_keyed_state("several.value", cx, |_, _| {
        vec![SharedString::from("mentions"), SharedString::from("replies")]
    });
    let checked = value.read(cx).clone();
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            CheckboxGroup::new("several.group")
                .aria_label("Notify me about")
                .value(checked.clone())
                .item(CheckboxGroupItem::new("several.mentions", "mentions").label("Mentions"))
                .item(CheckboxGroupItem::new("several.replies", "replies").label("Replies"))
                .item(CheckboxGroupItem::new("several.likes", "likes").label("Likes"))
                .on_change(move |next, _, cx| {
                    value.update(cx, |value, cx| {
                        *value = next.to_vec();
                        cx.notify();
                    })
                }),
        )
        .child(format!("{} of 3 on", checked.len()))
}

/// Disabled
///
/// Disable the whole group, or one item.
pub(crate) fn disabled(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    CheckboxGroup::new("off.group")
        .aria_label("Permissions")
        .value(["read"])
        .disabled(true)
        .item(CheckboxGroupItem::new("off.read", "read").label("Read"))
        .item(CheckboxGroupItem::new("off.write", "write").label("Write"))
}

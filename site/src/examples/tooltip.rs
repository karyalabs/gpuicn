//! Tooltip examples.

use crate::*;

/// Icon buttons
///
/// An icon-only button needs a name; the tooltip shows it on hover and focus.
pub(crate) fn icon_buttons(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let theme = UiTheme::read(cx).clone();
    let tools = [
        ("icons.bold", "Bold", LucideIcon::Bold),
        ("icons.italic", "Italic", LucideIcon::Italic),
        ("icons.link", "Insert link", LucideIcon::Link),
    ];
    div().flex().gap(px(4.)).children(tools.map(|(id, label, icon)| {
        tooltip(
            (id, 0usize),
            label,
            gpui_kit::base::Button::new(id)
                .accessibility_label(label)
                .size(px(32.))
                .flex()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(theme.colors.border)
                .rounded(theme.radius.md)
                .child(
                    lucide(icon)
                        .size(px(16.))
                        .text_color(theme.colors.foreground),
                ),
            window,
            cx,
        )
    }))
}

/// Longer text
///
/// A tooltip can explain a control in a short sentence.
pub(crate) fn longer_text(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    tooltip(
        "longer.tooltip",
        "Drafts are saved on this device only",
        gpui_kit::base::Button::new("longer.trigger")
            .accessibility_label("Save draft")
            .px(px(12.))
            .py(px(8.))
            .border_1()
            .border_color(UiTheme::read(cx).colors.border)
            .rounded(px(8.))
            .child("Save draft"),
        window,
        cx,
    )
}

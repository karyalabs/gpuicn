//! Toolbar examples.

use crate::*;

/// With icons
///
/// Buttons take any child. Give icon buttons a label for screen readers.
pub(crate) fn with_icons(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    let icon = move |icon: LucideIcon| lucide(icon).size(px(16.)).text_color(color);
    Toolbar::new("icons.toolbar", "Formatting", cx)
        .button(ToolbarButton::new("icons.bold", "Bold", cx).child(icon(LucideIcon::Bold)))
        .button(ToolbarButton::new("icons.italic", "Italic", cx).child(icon(LucideIcon::Italic)))
        .button(
            ToolbarButton::new("icons.underline", "Underline", cx)
                .child(icon(LucideIcon::Underline)),
        )
        .separator()
        .button(ToolbarButton::new("icons.link", "Add a link", cx).child(icon(LucideIcon::Link)))
        .button(
            ToolbarButton::new("icons.image", "Add an image", cx)
                .disabled(true)
                .child(icon(LucideIcon::Image)),
        )
}

/// Vertical
///
/// A vertical toolbar moves between its buttons with the up and down keys.
pub(crate) fn vertical(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let color = UiTheme::read(cx).colors.foreground;
    let icon = move |icon: LucideIcon| lucide(icon).size(px(16.)).text_color(color);
    Toolbar::new("vertical.toolbar", "Drawing tools", cx)
        .axis(gpui_kit::Axis::Vertical)
        .button(
            ToolbarButton::new("vertical.select", "Select", cx)
                .child(icon(LucideIcon::MousePointer2)),
        )
        .button(ToolbarButton::new("vertical.pen", "Pen", cx).child(icon(LucideIcon::Pen)))
        .button(
            ToolbarButton::new("vertical.shape", "Shape", cx).child(icon(LucideIcon::Square)),
        )
        .button(
            ToolbarButton::new("vertical.eraser", "Eraser", cx).child(icon(LucideIcon::Eraser)),
        )
}

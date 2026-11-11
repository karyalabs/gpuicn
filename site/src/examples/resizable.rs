//! Resizable examples.

use crate::*;
use gpuicn::resizable::{PaneLimits, Resizable};

/// Vertical
///
/// `vertical` stacks the panes; the size is then the first pane's height.
pub(crate) fn vertical(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let height = window.use_keyed_state("vertical.height", cx, |_, _| px(110.));
    let resized = height.clone();
    div().w(px(320.)).h(px(260.)).child(
        Resizable::new("vertical.split", "Resize editor and terminal", *height.read(cx))
            .vertical()
            .first(div().p(px(16.)).child("Editor"))
            .second(div().p(px(16.)).child("Terminal"))
            .on_resize(move |size: &gpui_kit::Pixels, _, cx| {
                resized.update(cx, |height, cx| {
                    *height = *size;
                    cx.notify();
                })
            })
            .first_limits(PaneLimits::new(px(60.), px(200.)))
            .second_limits(PaneLimits::new(px(60.), px(200.))),
    )
}

/// Sidebar limits
///
/// Limits keep a pane usable: this sidebar stays between 120 and 220 pixels.
pub(crate) fn sidebar_limits(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let width = window.use_keyed_state("limits.width", cx, |_, _| px(160.));
    let resized = width.clone();
    let muted = UiTheme::read(cx).colors.muted_foreground;
    div().w(px(440.)).h(px(200.)).child(
        Resizable::new("limits.split", "Resize sidebar", *width.read(cx))
            .first(
                div()
                    .p(px(16.))
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child("Inbox")
                    .child("Drafts")
                    .child("Sent"),
            )
            .second(
                div()
                    .p(px(16.))
                    .text_color(muted)
                    .child("Drag the divider. It stops at each limit."),
            )
            .on_resize(move |size: &gpui_kit::Pixels, _, cx| {
                resized.update(cx, |width, cx| {
                    *width = *size;
                    cx.notify();
                })
            })
            .first_limits(PaneLimits::new(px(120.), px(220.))),
    )
}

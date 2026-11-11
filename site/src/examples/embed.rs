//! Embed examples.

use crate::*;
use gpuicn::embed::{LinkPreview, MessageEmbed, QuotedMessage};

/// Link preview
///
/// A link's title, site and description, without an image.
pub(crate) fn link_preview(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div().w_full().max_w(px(460.)).child(
        LinkPreview::new("link.preview", "GPUI: a fast, productive UI framework for Rust")
            .site_name("gpui.rs")
            .description(
                "A hybrid immediate and retained mode, GPU accelerated UI framework, \
                 designed to support a wide variety of applications.",
            ),
    )
}

/// Status colors
///
/// `color_str` takes `good`, `warning`, `danger` or a hex color for the bar.
pub(crate) fn status_colors(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .w_full()
        .max_w(px(460.))
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            MessageEmbed::new("colors.good")
                .color_str("good")
                .title("Deploy finished")
                .body("Version 2.4 is live."),
        )
        .child(
            MessageEmbed::new("colors.warning")
                .color_str("warning")
                .title("Disk almost full")
                .body("The build cache is using 91% of the disk."),
        )
        .child(
            MessageEmbed::new("colors.danger")
                .color_str("danger")
                .title("Checks failed")
                .body("2 of 14 tests failed on main.")
                .field("Suite", "integration", true)
                .field("Duration", "3m 12s", true),
        )
}

/// Quoted message
///
/// A message shared from elsewhere, with its author, time and channel.
pub(crate) fn quoted_message(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div().w_full().max_w(px(460.)).child(
        QuotedMessage::new("quote.message", "Ada Lovelace")
            .initials("AL")
            .timestamp("Today at 9:15 AM")
            .channel("~town-square")
            .body("The release notes are ready for review."),
    )
}

//! Markdown examples.

use crate::*;
use gpuicn::markdown::markdown;

/// Text styles
///
/// Bold, strikethrough, inline code, links, quotes and lists.
pub(crate) fn text_styles(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let source = "**Bold**, ~~struck~~ and `inline code`, with a [link](https://example.com).\n\n> A quote sits behind a rule.\n\n1. First step\n2. Second step\n\n- A point\n- Another point";
    div()
        .w_full()
        .max_w(px(460.))
        .child(markdown("styles.markdown", source, cx))
}

/// Code block
///
/// A fenced block is highlighted by its language and has a copy button.
pub(crate) fn code_block(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let source = "```rust\nfn main() {\n    println!(\"Hello from GPUI\");\n}\n```";
    div()
        .w_full()
        .max_w(px(460.))
        .child(markdown("code.markdown", source, cx))
}

/// Table
///
/// A table takes the width of its content, with a header row.
pub(crate) fn table(
    _view: &Showcase,
    _window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let source = "| Plan | Seats | Price |\n| --- | --- | --- |\n| Free | 3 | $0 |\n| Team | 20 | $12 |\n| Business | Unlimited | $30 |";
    div()
        .w_full()
        .max_w(px(460.))
        .child(markdown("table.markdown", source, cx))
}

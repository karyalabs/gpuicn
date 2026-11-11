//! Composer examples.

use crate::*;

/// Send button only
///
/// The smallest composer: a placeholder, and a send button after a spacer.
pub(crate) fn send_button_only(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("send.state", cx, |window, cx| {
        let mut state = ComposerState::new(4, window, cx);
        state.set_placeholder("Write a reply…", window, cx);
        state
    });
    let send = state.clone();
    let color = UiTheme::read(cx).colors.primary_foreground;
    div().w_full().max_w(px(440.)).child(
        Composer::new(&state).child(div().flex_1()).child(
            Button::new("send.button")
                .size(ButtonSize::IconSm)
                .aria_label("Send")
                .on_click(move |_, window, cx| {
                    send.update(cx, |state, cx| {
                        state.set_value("", window, cx);
                        state.focus(window, cx);
                    })
                })
                .child(
                    lucide(LucideIcon::SendHorizontal)
                        .size(px(16.))
                        .text_color(color),
                ),
        ),
    )
}

/// Formatting buttons
///
/// `wrap_selection` puts Markdown marks around the selected text.
pub(crate) fn formatting_buttons(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let state = window.use_keyed_state("format.state", cx, |window, cx| {
        let mut state = ComposerState::new(4, window, cx);
        state.set_placeholder("Select some text, then press a button", window, cx);
        state
    });
    let color = UiTheme::read(cx).colors.muted_foreground;
    let tool = |id: &'static str, label: &'static str, icon: LucideIcon, mark: &'static str| {
        let state = state.clone();
        Button::new(id)
            .size(ButtonSize::IconSm)
            .variant(ButtonVariant::Ghost)
            .aria_label(label)
            .on_click(move |_, window, cx| {
                state.update(cx, |state, cx| {
                    state.wrap_selection(mark, mark, window, cx);
                    state.focus(window, cx);
                })
            })
            .child(lucide(icon).size(px(16.)).text_color(color))
    };
    div().w_full().max_w(px(440.)).child(
        Composer::new(&state)
            .child(tool("format.bold", "Bold", LucideIcon::Bold, "**"))
            .child(tool("format.italic", "Italic", LucideIcon::Italic, "_"))
            .child(tool(
                "format.strike",
                "Strikethrough",
                LucideIcon::Strikethrough,
                "~~",
            ))
            .child(tool("format.code", "Code", LucideIcon::Code, "`")),
    )
}

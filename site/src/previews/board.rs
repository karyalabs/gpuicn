//! The home page's hero: many components at once, laid out as a board of
//! cards. It is not a catalog entry; `?demo=board` asks for it.

use crate::*;

impl Showcase {
    pub(crate) fn board_preview(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let width = f32::from(window.viewport_size().width);
        let columns = match width {
            width if width >= 900. => 3,
            width if width >= 600. => 2,
            _ => 1,
        };
        let card = |child: gpui_kit::AnyElement| {
            div()
                .w_full()
                .flex()
                .flex_col()
                .items_center()
                .overflow_hidden()
                .rounded(theme.radius.xl)
                .border_1()
                .border_color(theme.colors.border)
                .bg(theme.colors.card)
                .p(px(20.))
                .child(child)
                .into_any_element()
        };
        let bare = |child: gpui_kit::AnyElement| {
            div()
                .w_full()
                .flex()
                .flex_col()
                .items_center()
                .child(child)
                .into_any_element()
        };
        // Reading order: the first cards are the ones a narrow window keeps.
        let cards = vec![
            bare(self.calendar_preview(window, cx).into_any_element()),
            card(self.form_preview(window, cx).into_any_element()),
            bare(self.item_preview(window, cx).into_any_element()),
            card(self.button_preview(window, cx).into_any_element()),
            card(self.tabs_preview(window, cx).into_any_element()),
            card(self.color_picker_preview(window, cx).into_any_element()),
            card(self.toggle_group_preview(window, cx).into_any_element()),
            card(self.switch_preview(window, cx).into_any_element()),
            card(self.slider_preview(window, cx).into_any_element()),
            card(self.badge_preview(window, cx).into_any_element()),
            card(self.progress_preview(window, cx).into_any_element()),
            card(self.tag_preview(window, cx).into_any_element()),
        ];
        let mut stacks: Vec<Vec<gpui_kit::AnyElement>> = (0..columns).map(|_| Vec::new()).collect();
        for (index, card) in cards.into_iter().enumerate() {
            stacks[index % columns].push(card);
        }
        div()
            .size_full()
            .flex()
            .items_start()
            .gap(px(16.))
            .p(px(16.))
            .children(stacks.into_iter().map(|stack| {
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .children(stack)
            }))
    }
}

pub(crate) fn preview(
    view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> gpui_kit::AnyElement {
    view.board_preview(window, cx).into_any_element()
}

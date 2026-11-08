//! The item catalog preview.

use crate::*;
use gpuicn::item::{Item, ItemGroup, ItemSize, ItemVariant};

impl Showcase {
    pub(crate) fn item_preview(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let switch_view = cx.entity().downgrade();
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .w(px(440.0))
            .child(
                ItemGroup::new("preview.item.group")
                    .child(
                        Item::new("preview.item.motion")
                            .title("Reduce motion")
                            .description("Turns off sliding panels and fades")
                            .actions(
                                Switch::new("preview.item.motion.switch")
                                    .checked(self.checked)
                                    .aria_label("Reduce motion")
                                    .on_change(move |checked, _, cx| {
                                        switch_view
                                            .update(cx, |this, cx| {
                                                this.checked = *checked;
                                                cx.notify();
                                            })
                                            .ok();
                                    }),
                            ),
                    )
                    .child(
                        Item::new("preview.item.folder")
                            .title("Download folder")
                            .description("~/Downloads")
                            .actions(
                                Button::new("preview.item.folder.change")
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Sm)
                                    .label("Change…"),
                            ),
                    )
                    .child(
                        Item::new("preview.item.keywords")
                            .title("Keywords")
                            .description("Words that notify you, besides your name")
                            .stacked(true)
                            .actions(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(6.0))
                                    .child(Badge::new("release").variant(BadgeVariant::Secondary))
                                    .child(Badge::new("gpui").variant(BadgeVariant::Secondary))
                                    .child(Badge::new("deploy").variant(BadgeVariant::Secondary)),
                            ),
                    ),
            )
            .child(
                Item::new("preview.item.muted")
                    .variant(ItemVariant::Muted)
                    .size(ItemSize::Sm)
                    .media(lucide(LucideIcon::Server).size(px(16.0)))
                    .title("karya.team")
                    .description("Signed in as @arya")
                    .on_click(|_, _, _| {}),
            )
    }
}

pub(crate) fn preview(
    view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> gpui_kit::AnyElement {
    view.item_preview(window, cx).into_any_element()
}

//! The empty catalog preview.

use crate::*;

impl Showcase {
    pub(crate) fn empty_preview(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The framed media slot does not size or color an icon; do both here.
        let color = UiTheme::read(cx).colors.foreground;
        Empty::new()
            .w(px(420.))
            .h(px(320.))
            .border_1()
            .header(
                EmptyHeader::new()
                    .media(
                        EmptyMedia::new()
                            .with_variant(EmptyMediaVariant::Icon)
                            .child(lucide(LucideIcon::FolderPlus).size(px(16.)).text_color(color)),
                    )
                    .title(EmptyTitle::new().child("No projects yet"))
                    .description(
                        EmptyDescription::new()
                            .child("Create a project to start organizing your work."),
                    ),
            )
            .content(EmptyContent::new().child(Button::new("empty.create").label("Create project")))
    }
}

pub(crate) fn preview(view: &Showcase, window: &mut Window, cx: &mut Context<Showcase>) -> gpui_kit::AnyElement {
    view.empty_preview(window, cx).into_any_element()
}

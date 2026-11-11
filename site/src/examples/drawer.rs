//! Drawer examples.

use crate::*;

/// From the side
///
/// `direction` attaches the drawer to any edge; a side drawer suits details and filters.
pub(crate) fn from_the_side(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = window
        .use_keyed_state("side.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let side = DrawerSide::Right;
    let muted = UiTheme::read(cx).colors.muted_foreground;
    let row = |label: &'static str, value: &'static str| {
        div()
            .flex()
            .justify_between()
            .child(div().text_color(muted).child(label))
            .child(value)
    };
    Drawer::new("side.drawer", &handle)
        .direction(side)
        .child(drawer_trigger("side.trigger", &handle).label("Order details"))
        .content(
            DrawerContent::new("side.content", "Order details")
                .child(
                    drawer_header(side, cx)
                        .child(drawer_title("side.title", cx).child("Order 1042"))
                        .child(
                            drawer_description("side.description", cx)
                                .child("Placed on 16 March."),
                        ),
                )
                .child(
                    drawer_body("side.body", cx)
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(row("Items", "2"))
                        .child(row("Shipping", "Courier"))
                        .child(row("Total", "$48.00")),
                )
                .child(drawer_footer(cx).child(drawer_close("side.close", &handle).label("Close"))),
        )
}

/// Must choose
///
/// With `dismissible(false)` a click outside does not close it; only its buttons do.
pub(crate) fn must_choose(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let handle = window
        .use_keyed_state("choose.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let side = DrawerSide::Bottom;
    Drawer::new("choose.drawer", &handle)
        .dismissible(false)
        .child(drawer_trigger("choose.trigger", &handle).label("Review cookies"))
        .content(
            DrawerContent::new("choose.content", "Cookies")
                .child(
                    drawer_header(side, cx)
                        .child(drawer_title("choose.title", cx).child("Cookies"))
                        .child(
                            drawer_description("choose.description", cx)
                                .child("We use cookies to remember your settings."),
                        ),
                )
                .child(
                    drawer_footer(cx)
                        .child(
                            drawer_close("choose.accept", &handle)
                                .variant(ButtonVariant::Default)
                                .label("Accept"),
                        )
                        .child(drawer_close("choose.decline", &handle).label("Decline")),
                ),
        )
}

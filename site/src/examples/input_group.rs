//! Input Group examples.

use crate::*;

/// With an icon
///
/// An addon holds any element. Here it is a search icon before the text.
pub(crate) fn with_an_icon(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let search = demo_input("icon.search", "", "Search the docs", window, cx);
    let color = UiTheme::read(cx).colors.muted_foreground;
    div().w_full().max_w(px(340.)).child(
        InputGroup::new("icon.group", InputGroupInput::new(&search))
            .aria_label("Search")
            .addon(
                InputGroupAddon::new("icon.addon")
                    .child(lucide(LucideIcon::Search).size(px(16.)).text_color(color)),
            ),
    )
}

/// With a button
///
/// An addon at the end can hold a button that acts on the text.
pub(crate) fn with_a_button(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let email = demo_input("button.email", "", "you@example.com", window, cx);
    div().w_full().max_w(px(340.)).child(
        InputGroup::new("button.group", InputGroupInput::new(&email))
            .aria_label("Email")
            .addon(
                InputGroupAddon::new("button.addon")
                    .align(InputGroupAddonAlignment::InlineEnd)
                    .button(InputGroupButton::new("button.subscribe").label("Subscribe")),
            ),
    )
}

/// Units
///
/// Short text on both sides, for a price or a measurement.
pub(crate) fn units(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let amount = demo_input("units.amount", "49.00", "", window, cx);
    div().w_full().max_w(px(220.)).child(
        InputGroup::new("units.group", InputGroupInput::new(&amount))
            .aria_label("Price")
            .addon(InputGroupAddon::new("units.before").child(InputGroupText::new().child("$")))
            .addon(
                InputGroupAddon::new("units.after")
                    .align(InputGroupAddonAlignment::InlineEnd)
                    .child(InputGroupText::new().child("USD")),
            ),
    )
}

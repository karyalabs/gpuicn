//! Named examples for a component's page, shown beside its overview preview.
//!
//! `?demo=button&example=sizes` draws `button::sizes`. An example is a free
//! function in `examples/<component>.rs` whose doc comment gives its title on
//! the first line and a sentence about it after a blank line; the website
//! reads both, and the function's source, from the file. List the function
//! in `examples!` below so the showcase can draw it.

use crate::Showcase;
use gpui_kit::{AnyElement, Context, IntoElement as _, Window};

macro_rules! examples {
    ($($module:ident: $($example:ident),+;)*) => {
        $(pub(crate) mod $module;)*

        /// Every example, as `(component module, function)`.
        #[cfg(test)]
        pub(crate) const ALL: &[(&str, &str)] = &[
            $($((stringify!($module), stringify!($example)),)+)*
        ];

        /// Draws `example` (`with-icon` or `with_icon`) of the component
        /// whose module is `component`, if there is one.
        pub(crate) fn render(
            component: &str,
            example: &str,
            view: &Showcase,
            window: &mut Window,
            cx: &mut Context<Showcase>,
        ) -> Option<AnyElement> {
            match (component, example.replace('-', "_").as_str()) {
                $($((stringify!($module), stringify!($example)) => {
                    Some($module::$example(view, window, cx).into_any_element())
                })+)*
                _ => None,
            }
        }
    };
}

examples! {
    alert: with_action, custom_icon, title_only;
    avatar: sizes, status, fallback;
    avatar_group: overflow, sizes;
    badge: variants, counts, with_icon;
    button: sizes, with_icon, icon_only, tinted, disabled;
    calendar: limits, monday_first;
    checkbox: with_description, indeterminate, disabled;
    checkbox_group: several_checked, disabled;
    date_picker: placeholder, custom_format, disabled;
    field: required, with_an_error, horizontal;
    input: placeholder, invalid, disabled;
    input_group: with_an_icon, with_a_button, units;
    item: variants, with_an_action;
    kbd: shortcuts, in_a_button;
    meter: levels, custom_range;
    number_field: step_and_limits, disabled;
    otp_field: four_digits, disabled;
    progress: determinate, indeterminate, custom_range;
    radio_group: plans, disabled;
    select: with_a_value, invalid, disabled;
    separator: vertical, in_a_list;
    skeleton: card, text;
    slider: steps, gradient, disabled;
    spinner: sizes, in_a_button;
    switch: settings, disabled;
    tabs: line, disabled_tab;
    tag: status_dots, removable, with_icon;
    textarea: rows, in_a_field;
    toggle: sizes, with_text, outline;
    toggle_group: single, multiple, segmented;
    toolbar: with_icons, vertical;
}

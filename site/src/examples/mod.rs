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
    accordion: several_open, disabled_section;
    alert: with_action, custom_icon, title_only;
    alert_dialog: destructive, one_button;
    attachment: files, upload_states;
    autocomplete: free_text;
    avatar: sizes, status, fallback;
    avatar_group: overflow, sizes;
    badge: variants, counts, with_icon;
    button: sizes, with_icon, icon_only, tinted, disabled;
    calendar: limits, monday_first;
    carousel: arrows_only, dots_only;
    checkbox: with_description, indeterminate, disabled;
    checkbox_group: several_checked, disabled;
    collapsible: text_trigger, show_more;
    color_picker: without_swatches, disabled;
    combobox: empty, disabled;
    command: inline, keywords;
    composer: send_button_only, formatting_buttons;
    context_menu: on_a_list_row, checked_items;
    date_picker: placeholder, custom_format, disabled;
    date_time_picker: placeholder, twenty_four_hour, disabled;
    dialog: with_a_cancel_button, long_content;
    drawer: from_the_side, must_choose;
    embed: link_preview, status_colors, quoted_message;
    emoji_picker: small_set;
    empty: without_an_action, two_actions;
    field: required, with_an_error, horizontal;
    fieldset: label_legend, with_a_description;
    form: sign_in, with_fieldsets;
    input: placeholder, invalid, disabled;
    input_group: with_an_icon, with_a_button, units;
    item: variants, with_an_action;
    kbd: shortcuts, in_a_button;
    lightbox: single_image;
    markdown: text_styles, code_block, table;
    menu: checked_items, icons_and_shortcuts;
    menubar: icons_and_shortcuts, settings_in_a_menu;
    message: reactions, sending_and_failed, compact;
    message_list: short_history, unread_pill;
    meter: levels, custom_range;
    navigation_menu: with_descriptions;
    number_field: step_and_limits, disabled;
    otp_field: four_digits, disabled;
    popover: text_only, with_actions, opens_above;
    preview_card: profile, in_a_sentence;
    progress: determinate, indeterminate, custom_range;
    radio_group: plans, disabled;
    resizable: vertical, sidebar_limits;
    scroll_area: horizontal;
    select: with_a_value, invalid, disabled;
    separator: vertical, in_a_list;
    skeleton: card, text;
    slider: steps, gradient, disabled;
    sortable: flat_list, between_groups;
    spinner: sizes, in_a_button;
    switch: settings, disabled;
    tabs: line, disabled_tab;
    tag: status_dots, removable, with_icon;
    textarea: rows, in_a_field;
    title_bar: page_colors, with_actions;
    toast: with_an_action, timing;
    toggle: sizes, with_text, outline;
    toggle_group: single, multiple, segmented;
    toolbar: with_icons, vertical;
    tooltip: icon_buttons, longer_text;
    virtual_list: single_selection, empty;
}

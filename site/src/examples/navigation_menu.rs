//! Navigation menu examples.

use crate::*;

/// With descriptions
///
/// A link can carry an icon and a line saying where it leads.
pub(crate) fn with_descriptions(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let product = window.use_keyed_state("described.product", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("described.components", "Components")
                    .link()
                    .icon(LucideIcon::LayoutGrid)
                    .description("Every component, with live previews"),
                MenuItem::new("described.themes", "Themes")
                    .link()
                    .icon(LucideIcon::Palette)
                    .description("Base colors, accents and radius"),
                MenuItem::new("described.cli", "CLI")
                    .link()
                    .icon(LucideIcon::Terminal)
                    .description("Copy components into your app"),
            ],
            cx,
        )
        .width(px(300.))
    });
    let resources = window.use_keyed_state("described.resources", cx, |_, cx| {
        MenuState::new(
            [
                MenuItem::new("described.changelog", "Changelog")
                    .link()
                    .icon(LucideIcon::Clock)
                    .description("What changed in each release"),
                MenuItem::new("described.source", "Source")
                    .link()
                    .icon(LucideIcon::Code)
                    .description("Browse the code on GitHub"),
            ],
            cx,
        )
        .width(px(300.))
    });
    navigation_menu("described.nav", "Site")
        .menu(&product, "Product")
        .menu(&resources, "Resources")
}

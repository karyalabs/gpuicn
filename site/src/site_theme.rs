//! The theme the website asks for. The site's theme menu picks a shadcn base
//! color, an accent and a corner radius; every preview frame receives them as
//! `?style=<base>-<accent>-<radius in rem>` and again in each `preview-update`
//! message, and builds its `UiTheme` here the way an app would: from theme CSS.

use gpui_kit::{Rgba, px, rgb};
use gpuicn::{ThemeMode, UiRadius, UiTheme};

/// The base colors `gpuicn theme set --base-color` accepts.
const BASE_COLORS: &[(&str, &str)] = &[
    (
        "neutral",
        include_str!("../../crates/cli/templates/base-colors/neutral.css"),
    ),
    (
        "stone",
        include_str!("../../crates/cli/templates/base-colors/stone.css"),
    ),
    (
        "zinc",
        include_str!("../../crates/cli/templates/base-colors/zinc.css"),
    ),
    (
        "mauve",
        include_str!("../../crates/cli/templates/base-colors/mauve.css"),
    ),
    (
        "olive",
        include_str!("../../crates/cli/templates/base-colors/olive.css"),
    ),
    (
        "mist",
        include_str!("../../crates/cli/templates/base-colors/mist.css"),
    ),
    (
        "taupe",
        include_str!("../../crates/cli/templates/base-colors/taupe.css"),
    ),
];

/// Accent name, then the primary color in light and in dark mode. Keep in step
/// with `accents` in `web/src/lib/theme.ts`.
const ACCENTS: &[(&str, u32, u32)] = &[
    ("blue", 0x2563eb, 0x3b82f6),
    ("violet", 0x7c3aed, 0x8b5cf6),
    ("rose", 0xe11d48, 0xf43f5e),
    ("orange", 0xea580c, 0xf97316),
    ("green", 0x16a34a, 0x22c55e),
];

/// Builds the theme for `style`, which is `base-accent-radius`, such as `zinc-blue-0.5`. Unknown or
/// missing parts keep the Neutral defaults.
pub(crate) fn theme(mode: ThemeMode, style: &str) -> UiTheme {
    let mut parts = style.split('-');
    let base = parts.next().unwrap_or_default();
    let accent = parts.next().unwrap_or_default();
    let radius = parts.next().and_then(|value| value.parse::<f32>().ok());

    let mut theme = BASE_COLORS
        .iter()
        .find(|(name, _)| *name == base)
        .and_then(|(_, css)| UiTheme::from_shadcn_css(css, mode).ok())
        .unwrap_or_else(|| match mode {
            ThemeMode::Dark => UiTheme::neutral_dark(),
            _ => UiTheme::neutral_light(),
        });
    if let Some((_, light, dark)) = ACCENTS.iter().find(|(name, _, _)| *name == accent) {
        let primary: Rgba = rgb(if mode == ThemeMode::Dark {
            *dark
        } else {
            *light
        });
        theme.colors.primary = primary;
        theme.colors.primary_foreground = rgb(0xffffff);
        theme.colors.ring = primary;
        theme.colors.sidebar_primary = primary;
        theme.colors.sidebar_primary_foreground = rgb(0xffffff);
    }
    if let Some(radius) = radius.filter(|value| (0.0..=1.5).contains(value)) {
        theme.radius = UiRadius::new(px(radius * 16.));
    }
    theme
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_changes_only_what_it_names() {
        assert_eq!(theme(ThemeMode::Light, ""), UiTheme::neutral_light());
        assert_eq!(
            theme(ThemeMode::Dark, "nope-nope-nope"),
            UiTheme::neutral_dark()
        );
        let styled = theme(ThemeMode::Light, "zinc-blue-0.25");
        assert_eq!(styled.colors.primary, rgb(0x2563eb));
        assert_eq!(styled.radius.base, px(4.));
        assert_ne!(
            styled.colors.foreground,
            UiTheme::neutral_light().colors.foreground
        );
        for (name, _) in BASE_COLORS {
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                assert_eq!(theme(mode, name).mode, mode, "{name}");
            }
        }
    }
}

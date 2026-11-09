//! Nova-styled loading placeholders.

use gpui_kit::{
    App, ElementId, InteractiveElement as _, IntoElement, RenderOnce, Styled, Window, div,
    ease_in_out,
};

use super::theme::UiTheme;

/// One pulse cycle, matching Tailwind's `animate-pulse`.
const PULSE: std::time::Duration = std::time::Duration::from_secs(2);

/// A muted block that stands in for content while it loads.
///
/// Size it with `w`, `h` and `rounded` like any element. Skeletons are hidden from
/// assistive technology; announce loading with a nearby `Spinner` or label instead.
#[derive(IntoElement)]
pub struct Skeleton {
    style: gpui_kit::StyleRefinement,
    id: ElementId,
}

impl Skeleton {
    /// Creates a `Skeleton` with a stable caller-owned ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            style: gpui_kit::StyleRefinement::default(),
            id: id.into(),
        }
    }
}

impl RenderOnce for Skeleton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx);
        let root = div()
            .id(self.id)
            .w_full()
            .h(theme.space(4.))
            .flex_shrink_0()
            .rounded(theme.radius.md)
            .bg(theme.colors.muted);
        let root = super::theme::apply_style(root, &self.style);
        if theme.motion.reduced || cx.reduce_motion() {
            return root.into_any_element();
        }
        // Stepped, not per frame: skeletons can stand in for a while.
        let delta = ease_in_out(super::theme::loop_phase(PULSE, window, cx));
        // Tailwind's pulse dips to half opacity halfway through each cycle.
        root.opacity(1. - 0.5 * (1. - (2. * delta - 1.).abs()))
            .into_any_element()
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

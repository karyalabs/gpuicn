//! Nova tooltips for pointer hover and keyboard focus.
use super::theme::{UiTheme, primitives::surface};
use gpui_kit::base::{
    Button, ElementExt as _, Placement, Positioner, ResolvedPosition, Tooltip, TooltipDefaults,
    TooltipOverlay, TooltipRequest,
};
use gpui_kit::{
    AnyView, AnyWindowHandle, App, AppContext as _, AvailableSpace, Background, Bounds, Context,
    DispatchPhase, ElementId, Entity, FocusHandle, Global, HitboxBehavior, InteractiveElement as _,
    IntoElement, MouseDownEvent, MouseMoveEvent, ParentElement, PathBuilder, Pixels, ScrollEvent,
    SharedString, StatefulInteractiveElement as _, Styled as _, Subscription, Task, WeakEntity,
    Window, div, point, px,
};
use std::{cell::Cell, rc::Rc};

/// Adds a tooltip to one existing button, preserving a single focus target.
/// The button must retain its own accessible name.
pub fn tooltip(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    trigger: Button,
    window: &mut Window,
    cx: &mut App,
) -> gpui_kit::Div {
    let state = window.use_keyed_state(id.into(), cx, TooltipState::new);
    state.update(cx, |state, _| state.label = label.into());
    let focus = state.read(cx).focus.clone();
    let overlay = state.read(cx).overlay.clone();
    let hover = state.clone();
    let measured = state.clone();
    div()
        .child(
            trigger
                .track_focus(&focus)
                .on_prepaint(move |bounds, _, cx| {
                    measured.update(cx, |state, _| state.bounds = bounds)
                })
                .on_hover(move |hovered, window, cx| {
                    hover.update(cx, |state, cx| {
                        state.hovered = *hovered;
                        state.update_visibility(window, cx);
                    })
                }),
        )
        .child(overlay)
}

struct TooltipState {
    focus: FocusHandle,
    overlay: Entity<TooltipOverlay>,
    bounds: Bounds<Pixels>,
    label: SharedString,
    hovered: bool,
    active: bool,
    dismissed: bool,
    _subscriptions: [Subscription; 2],
}

/// One Escape interceptor for every tooltip in the application, instead of
/// one keystroke hook per tooltip. Active tooltips register here while shown.
#[derive(Default)]
struct EscapeDismissal {
    shown: Vec<(AnyWindowHandle, WeakEntity<TooltipState>)>,
    _subscription: Option<Subscription>,
}
impl Global for EscapeDismissal {}
impl EscapeDismissal {
    fn install(cx: &mut App) {
        if cx.default_global::<Self>()._subscription.is_some() {
            return;
        }
        let subscription = cx.intercept_keystrokes(|event, window, cx| {
            if event.keystroke.key != "escape" {
                return;
            }
            let handle = window.window_handle();
            let shown = std::mem::take(&mut cx.global_mut::<Self>().shown);
            let mut dismissed = false;
            for (owner, state) in shown {
                if owner != handle {
                    cx.global_mut::<Self>().shown.push((owner, state));
                    continue;
                }
                state
                    .update(cx, |state, cx| {
                        if state.active {
                            state.overlay.update(cx, |overlay, cx| overlay.hide(cx));
                            state.active = false;
                            state.dismissed = true;
                            dismissed = true;
                        }
                    })
                    .ok();
            }
            if dismissed {
                cx.stop_propagation();
            }
        });
        cx.global_mut::<Self>()._subscription = Some(subscription);
    }
    fn shown(window: &Window, state: WeakEntity<TooltipState>, cx: &mut App) {
        let handle = window.window_handle();
        let shown = &mut cx.global_mut::<Self>().shown;
        shown.retain(|(_, other)| {
            other.upgrade().is_some() && other.entity_id() != state.entity_id()
        });
        shown.push((handle, state));
    }
    fn hidden(state: &WeakEntity<TooltipState>, cx: &mut App) {
        cx.global_mut::<Self>()
            .shown
            .retain(|(_, other)| other.entity_id() != state.entity_id());
    }
}

impl TooltipState {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        EscapeDismissal::install(cx);
        let focus = cx.focus_handle();
        let subscriptions = [
            cx.on_focus(&focus, window, |state, window, cx| {
                state.update_visibility(window, cx)
            }),
            cx.on_blur(&focus, window, |state, window, cx| {
                state.update_visibility(window, cx)
            }),
        ];
        Self {
            focus,
            overlay: cx.new(|_| TooltipOverlay::new()),
            bounds: Bounds::default(),
            label: SharedString::default(),
            hovered: false,
            active: false,
            dismissed: false,
            _subscriptions: subscriptions,
        }
    }
    fn update_visibility(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let active = self.hovered || self.focus.is_focused(window);
        if !active {
            self.dismissed = false;
        }
        let active = active && !self.dismissed;
        if active == self.active {
            return;
        }
        self.active = active;
        if active {
            let label = self.label.clone();
            let request =
                TooltipRequest::new(self.bounds, move |_, cx| text_tooltip(label.clone(), cx));
            self.overlay
                .update(cx, |overlay, cx| overlay.request_show(request, window, cx));
            EscapeDismissal::shown(window, cx.weak_entity(), cx);
        } else {
            self.overlay.update(cx, |overlay, cx| overlay.hide(cx));
            EscapeDismissal::hidden(&cx.weak_entity(), cx);
        }
    }
}

/// Creates a tooltip view for GPUI's native `.tooltip(...)` attachment point.
/// Use this on an existing interactive control to avoid nesting another button
/// and tab stop. The host control must retain its own full accessible name.
pub fn text_tooltip(label: gpui_kit::SharedString, cx: &mut App) -> gpui_kit::AnyView {
    use gpui_kit::AppContext as _;
    cx.new(|_| TextTooltip(label)).into()
}
struct TextTooltip(gpui_kit::SharedString);
impl gpui_kit::Render for TextTooltip {
    fn render(
        &mut self,
        _: &mut gpui_kit::Window,
        cx: &mut gpui_kit::Context<Self>,
    ) -> impl gpui_kit::IntoElement {
        use gpui_kit::ParentElement as _;
        let theme = UiTheme::read(cx);
        surface::tooltip(Tooltip::new("tooltip-content"), theme).child(
            div()
                .debug_selector(|| "tooltip-label".into())
                .child(self.0.clone()),
        )
    }
}

/// How far the arrow reaches out of the bubble, and how wide its base is.
const ARROW_DEPTH: f32 = 6.;
const ARROW_WIDTH: f32 = 12.;
/// Between the arrow's tip and the trigger.
const ARROW_GAP: f32 = 2.;
/// Kept between a tooltip and the window edge.
const EDGE_MARGIN: f32 = 8.;
/// Above popups, as Base draws its own tooltips.
const TOOLTIP_PRIORITY: usize = 200;

/// Hover tooltips anchored to their trigger: centered above it with an arrow
/// pointing at it, below it when there is no room above, and kept inside the
/// window. GPUI's native `.tooltip(...)` opens wherever the pointer happens to
/// be, below and to the right of it, which reads as detached from the thing it
/// describes.
pub trait AnchoredTooltip: ParentElement + Sized {
    /// Shows `build`'s view, usually [`text_tooltip`], after the hover delay
    /// and hides it when the pointer leaves, presses or scrolls. The element
    /// keeps its own hover, click and focus behavior.
    fn anchored_tooltip(self, build: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.child(anchor(Rc::new(build)))
    }
}
impl<T: ParentElement> AnchoredTooltip for T {}

type TipBuilder = Rc<dyn Fn(&mut Window, &mut App) -> AnyView>;

/// The one hovered trigger in the application: a pointer hovers one place at a
/// time, so one slot serves every window.
#[derive(Default)]
struct AnchoredTips {
    active: Option<ActiveTip>,
}
impl Global for AnchoredTips {}

struct ActiveTip {
    window: AnyWindowHandle,
    trigger: Bounds<Pixels>,
    /// Built once the delay passes, then kept so the tip does not rebuild on
    /// every frame.
    view: Option<AnyView>,
    dismissed: bool,
    _delay: Option<Task<()>>,
}

impl ActiveTip {
    fn is(&self, window: AnyWindowHandle, trigger: Bounds<Pixels>) -> bool {
        self.window == window && self.trigger == trigger
    }
}

fn contains(outer: Bounds<Pixels>, inner: Bounds<Pixels>) -> bool {
    outer.left() <= inner.left()
        && outer.top() <= inner.top()
        && outer.right() >= inner.right()
        && outer.bottom() >= inner.bottom()
}

/// An invisible layer over the trigger that watches the pointer without
/// taking it, and draws the tooltip on top of everything while it is due.
///
/// Hover is read in the mouse listeners, against the frame the pointer moved
/// over: a hitbox inserted during prepaint is not in that frame's hit test
/// yet, so asking it there always says "not hovered".
fn anchor(build: TipBuilder) -> impl IntoElement {
    gpui_kit::canvas(
        move |trigger, window, cx| {
            let hitbox = window.insert_hitbox(trigger, HitboxBehavior::Normal);
            let handle = window.window_handle();
            let view = cx
                .try_global::<AnchoredTips>()
                .and_then(|tips| tips.active.as_ref())
                .filter(|active| active.is(handle, trigger) && !active.dismissed)
                .and_then(|active| active.view.clone());
            let overlay = view.map(|view| {
                let mut overlay = bubble(view, trigger, cx).into_any_element();
                overlay.prepaint_as_root(trigger.origin, AvailableSpace::min_size(), window, cx);
                overlay
            });
            (hitbox, overlay)
        },
        move |trigger, (hitbox, overlay), window, cx| {
            if let Some(mut overlay) = overlay {
                overlay.paint(window, cx);
            }
            let view_id = window.current_view();
            let moved = hitbox.clone();
            let build = build.clone();
            window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble {
                    hover(
                        moved.is_hovered(window),
                        trigger,
                        &build,
                        view_id,
                        window,
                        cx,
                    );
                }
            });
            let pressed = hitbox.clone();
            window.on_mouse_event(move |_: &MouseDownEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble && pressed.is_hovered(window) {
                    dismiss(window.window_handle(), trigger, cx);
                    cx.notify(view_id);
                }
            });
            window.on_mouse_event(move |_: &ScrollEvent, phase, window, cx| {
                if phase == DispatchPhase::Bubble && hitbox.is_hovered(window) {
                    dismiss(window.window_handle(), trigger, cx);
                    cx.notify(view_id);
                }
            });
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// Starts the delay when the pointer enters a trigger and forgets the tip
/// when it leaves. Listeners run innermost first, so a trigger inside another
/// (a status emoji in a row with its own tooltip) claims the slot and the
/// outer one leaves it be while the pointer is over the inner one.
fn hover(
    hovered: bool,
    trigger: Bounds<Pixels>,
    build: &TipBuilder,
    view_id: gpui_kit::EntityId,
    window: &mut Window,
    cx: &mut App,
) {
    let handle = window.window_handle();
    let mouse = window.mouse_position();
    let tips = cx.default_global::<AnchoredTips>();
    let mine = tips
        .active
        .as_ref()
        .is_some_and(|active| active.is(handle, trigger));
    if !hovered {
        if mine {
            tips.active = None;
            cx.notify(view_id);
        }
        return;
    }
    let inner = tips.active.as_ref().is_some_and(|active| {
        !mine
            && active.window == handle
            && active.trigger.contains(&mouse)
            && contains(trigger, active.trigger)
    });
    if mine || inner {
        return;
    }
    let delay = TooltipDefaults::global(cx).show_delay();
    let build = build.clone();
    let task = window.spawn(cx, async move |cx| {
        cx.background_executor().timer(delay).await;
        cx.update(|window, cx| {
            let due = cx
                .global::<AnchoredTips>()
                .active
                .as_ref()
                .is_some_and(|active| active.is(handle, trigger) && !active.dismissed);
            if due {
                let view = build(window, cx);
                if let Some(active) = cx.global_mut::<AnchoredTips>().active.as_mut() {
                    active.view = Some(view);
                }
                cx.notify(view_id);
            }
        })
        .ok();
    });
    cx.global_mut::<AnchoredTips>().active = Some(ActiveTip {
        window: handle,
        trigger,
        view: None,
        dismissed: false,
        _delay: Some(task),
    });
}

fn dismiss(window: AnyWindowHandle, trigger: Bounds<Pixels>, cx: &mut App) {
    if let Some(active) = cx.default_global::<AnchoredTips>().active.as_mut()
        && active.is(window, trigger)
    {
        active.dismissed = true;
        active.view = None;
    }
}

/// The tip placed on its side of the trigger with the arrow drawn from the
/// edge facing it, at the trigger's center (or as near as the corners allow
/// when the tip is pushed in from the window edge).
fn bubble(view: AnyView, trigger: Bounds<Pixels>, cx: &App) -> impl IntoElement {
    let theme = UiTheme::read(cx);
    let fill = theme.colors.popover;
    let stroke = theme.control().surface_border;
    let corner = theme.radius.md;
    let resolved = Rc::new(Cell::new(None));
    let placed = resolved.clone();
    gpui_kit::deferred(
        Positioner::side(trigger)
            .placement(Placement::Top)
            .offset(px(ARROW_DEPTH + ARROW_GAP))
            .margin(px(EDGE_MARGIN))
            .on_position(move |position| placed.set(Some(position)))
            .child(
                div()
                    .debug_selector(|| "anchored-tooltip".into())
                    .relative()
                    .child(view)
                    .child(
                        gpui_kit::canvas(
                            |_, _, _| {},
                            move |_, _, window, _| {
                                let Some(ResolvedPosition {
                                    bounds,
                                    placement: Some(placement),
                                }) = resolved.get()
                                else {
                                    return;
                                };
                                paint_arrow(
                                    bounds, placement, trigger, corner, fill, stroke, window,
                                );
                            },
                        )
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                    ),
            ),
    )
    .with_priority(TOOLTIP_PRIORITY)
}

fn paint_arrow(
    bubble: Bounds<Pixels>,
    placement: Placement,
    trigger: Bounds<Pixels>,
    corner: Pixels,
    fill: impl Into<Background> + Copy,
    stroke: impl Into<Background> + Copy,
    window: &mut Window,
) {
    let half = px(ARROW_WIDTH / 2.);
    let low = bubble.left() + corner + half;
    let high = bubble.right() - corner - half;
    if low > high {
        return;
    }
    let x = trigger.center().x.clamp(low, high);
    let (edge, out) = match placement {
        Placement::Top => (bubble.bottom(), px(1.)),
        Placement::Bottom => (bubble.top(), px(-1.)),
        Placement::Left | Placement::Right => return,
    };
    // The border-colored triangle, then the fill inset by about a pixel
    // along its slanted sides. Both start a pixel inside the bubble, so the
    // fill covers the bubble's border where the arrow leaves it.
    let triangle = |inset: f32| {
        let base = edge - out;
        let mut path = PathBuilder::fill();
        path.move_to(point(x - half + px(inset), base));
        path.line_to(point(x + half - px(inset), base));
        path.line_to(point(x, edge + out * (ARROW_DEPTH - inset)));
        path.close();
        path.build().ok()
    };
    if let Some(path) = triangle(0.) {
        window.paint_path(path, stroke);
    }
    if let Some(path) = triangle(1.4) {
        window.paint_path(path, fill);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{IntoElement, Modifiers, Render, Styled, TestAppContext, px};
    use std::time::Duration;

    #[gpui_kit::test]
    fn focus_and_hover_show_tooltip_and_escape_dismisses_it(cx: &mut TestAppContext) {
        struct Probe;
        impl Render for Probe {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                div()
                    .child(tooltip(
                        "test-tooltip",
                        "Help",
                        Button::new("help").size(px(100.)).child("Help"),
                        window,
                        cx,
                    ))
                    .child(Button::new("after").child("After"))
            }
        }
        cx.update(super::super::theme::init);
        let (_, visual) = cx.add_window_view(|_, _| Probe);
        visual.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
        });
        visual.simulate_mouse_move(
            gpui_kit::point(px(200.), px(200.)),
            None,
            Modifiers::default(),
        );
        visual.simulate_keystrokes("tab");
        visual.executor().advance_clock(Duration::from_millis(550));
        visual.update(|window, cx| window.draw(cx).clear(cx));
        assert!(visual.debug_bounds("tooltip-label").is_some());
        visual.simulate_keystrokes("escape");
        assert!(visual.debug_bounds("tooltip-label").is_none());
        visual.simulate_keystrokes("tab shift-tab");
        visual.executor().advance_clock(Duration::from_millis(550));
        visual.update(|window, cx| window.draw(cx).clear(cx));
        assert!(visual.debug_bounds("tooltip-label").is_some());
        visual.simulate_keystrokes("tab");
        visual.update(|window, cx| window.draw(cx).clear(cx));
        assert!(visual.debug_bounds("tooltip-label").is_none());
        // Hover the first control while keyboard focus stays on the second.
        visual.simulate_mouse_move(
            gpui_kit::point(px(10.), px(10.)),
            None,
            Modifiers::default(),
        );
        visual.executor().advance_clock(Duration::from_millis(550));
        visual.update(|window, cx| window.draw(cx).clear(cx));
        assert!(visual.debug_bounds("tooltip-label").is_some());
        visual.simulate_keystrokes("escape");
        assert!(visual.debug_bounds("tooltip-label").is_none());
    }

    #[gpui_kit::test]
    fn anchored_tooltips_sit_centered_above_and_flip_below_at_the_top(cx: &mut TestAppContext) {
        struct Probe;
        impl Render for Probe {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .child(
                        div()
                            .id("top")
                            .absolute()
                            .top(px(0.))
                            .left(px(100.))
                            .size(px(40.))
                            .anchored_tooltip(|_, cx| text_tooltip("Top".into(), cx)),
                    )
                    .child(
                        div()
                            .id("middle")
                            .absolute()
                            .top(px(300.))
                            .left(px(300.))
                            .size(px(40.))
                            .anchored_tooltip(|_, cx| text_tooltip("Middle".into(), cx)),
                    )
            }
        }
        cx.update(super::super::theme::init);
        let (_, visual) = cx.add_window_view(|_, _| Probe);
        visual.update(|window, cx| window.draw(cx).clear(cx));
        let mut show = |at: gpui_kit::Point<Pixels>| {
            visual.simulate_mouse_move(at, None, Modifiers::default());
            visual.executor().advance_clock(Duration::from_millis(550));
            visual.run_until_parked();
            visual.update(|window, cx| window.draw(cx).clear(cx));
            visual.debug_bounds("anchored-tooltip")
        };

        let tip = show(gpui_kit::point(px(320.), px(320.))).expect("shown after the delay");
        assert!(tip.bottom() < px(300.), "{tip:?}");
        assert!((tip.center().x - px(320.)).abs() < px(1.), "{tip:?}");

        let tip = show(gpui_kit::point(px(120.), px(20.))).expect("shown below the top one");
        assert!(tip.top() > px(40.), "{tip:?}");
        assert!((tip.center().x - px(120.)).abs() < px(1.), "{tip:?}");

        visual.simulate_mouse_move(
            gpui_kit::point(px(600.), px(600.)),
            None,
            Modifiers::default(),
        );
        visual.update(|window, cx| window.draw(cx).clear(cx));
        assert!(visual.debug_bounds("anchored-tooltip").is_none());
    }
}

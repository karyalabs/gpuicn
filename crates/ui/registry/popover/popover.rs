//! Nova surfaces for GPUI Kit's anchored popover, and a popover that also
//! opens on hover for profile cards.
use super::{
    button::{ButtonSize, ButtonVariant, style_button},
    theme::{
        UiTheme, apply_style,
        locale::DirectionalStyled as _,
        primitives::{dismiss, focus, surface},
    },
};
use gpui_kit::base::{Align, Button, ElementExt as _, POPUP_PRIORITY, Placement, Positioner};
pub use gpui_kit::base::{Popover, PopoverState};
use gpui_kit::{
    AnyElement, App, Bounds, Context, Div, ElementId, FocusHandle, FontWeight,
    InteractiveElement as _, IntoElement, ParentElement as _, Pixels, RenderOnce, Role,
    SharedString, Stateful, StatefulInteractiveElement as _, StyleRefinement, Styled, Task, Window,
    div, prelude::FluentBuilder as _,
};
use std::{rc::Rc, time::Duration};

/// Create a popover and supply its trigger and content builder directly.
pub fn popover(id: impl Into<ElementId>) -> Popover {
    Popover::new(id)
}
/// Outline trigger compatible with Kit's selected-state projection.
pub fn popover_trigger(id: impl Into<ElementId>, cx: &App) -> Button {
    style_button(
        Button::new(id),
        false,
        ButtonVariant::Outline,
        ButtonSize::Default,
        UiTheme::read(cx),
    )
}
/// Named Nova content surface. Dismiss through the content builder's `PopoverState`.
pub fn popover_popup(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    cx: &App,
) -> Stateful<Div> {
    let t = UiTheme::read(cx);
    let popup = div()
        .id(id)
        .aria_label(label)
        .w(t.space(72.))
        .flex()
        .flex_col()
        .gap(t.space(2.5))
        .p(t.space(2.5));
    surface::popover(popup, t)
}
/// Medium-weight popover title.
pub fn popover_title(cx: &App) -> Div {
    let t = UiTheme::read(cx);
    div()
        .font_family(t.fonts.body.clone())
        .font_weight(FontWeight::MEDIUM)
        .text_size(t.text(14.))
        .text_color(t.colors.popover_foreground)
}
/// Muted popover description.
pub fn popover_description(cx: &App) -> Div {
    let t = UiTheme::read(cx);
    div()
        .font_family(t.fonts.body.clone())
        .text_size(t.text(14.))
        .text_color(t.colors.muted_foreground)
}

type OpenChange = Rc<dyn Fn(&bool, &mut Window, &mut App)>;
type Suppress = Rc<dyn Fn(&Window, &App) -> bool>;
type ContentBuilder = Box<dyn FnOnce(&mut Window, &mut App) -> AnyElement>;
type TriggerStyle = Box<dyn FnOnce(Button) -> Button>;

/// How long the pointer rests on a hover popover's trigger before it opens.
pub const HOVER_POPOVER_OPEN_DELAY: Duration = Duration::from_millis(500);
/// How long a hover popover stays open after the pointer leaves it.
pub const HOVER_POPOVER_CLOSE_DELAY: Duration = Duration::from_millis(300);

/// A popover that opens on hover and still opens by click and keyboard, for
/// profile cards on avatars and names.
///
/// The trigger is one focusable button with the accessible name you pass.
/// Resting the pointer on it opens the popup after `open_delay` without
/// moving focus, and the pointer can travel into the popup to use it; it
/// closes `close_delay` after the pointer leaves both. A click, Enter or
/// Space opens it and moves focus into it, and then it stays open until
/// Escape, a click outside, or the trigger again; Escape gives focus back to
/// the trigger. Build the popup with [`popover_popup`] or `preview_card_popup`.
#[derive(IntoElement)]
pub struct HoverPopover {
    id: ElementId,
    label: SharedString,
    trigger: Option<AnyElement>,
    trigger_style: Option<TriggerStyle>,
    content: Option<ContentBuilder>,
    open_delay: Duration,
    close_delay: Duration,
    placement: Placement,
    align: Align,
    on_open_change: Option<OpenChange>,
    passive: bool,
    open_requested: bool,
    suppress: Option<Suppress>,
    style: StyleRefinement,
}

/// Create a hover popover whose trigger is named `label`, such as "Priya Nair, online".
pub fn hover_popover(id: impl Into<ElementId>, label: impl Into<SharedString>) -> HoverPopover {
    HoverPopover {
        id: id.into(),
        label: label.into(),
        trigger: None,
        trigger_style: None,
        content: None,
        open_delay: HOVER_POPOVER_OPEN_DELAY,
        close_delay: HOVER_POPOVER_CLOSE_DELAY,
        placement: Placement::Bottom,
        align: Align::Start,
        on_open_change: None,
        passive: false,
        open_requested: false,
        suppress: None,
        style: StyleRefinement::default(),
    }
}

impl HoverPopover {
    /// Sets what the trigger shows, such as an `Avatar` or a name.
    pub fn trigger(mut self, child: impl IntoElement) -> Self {
        self.trigger = Some(child.into_any_element());
        self
    }
    /// Replaces the trigger's styling while keeping its behavior. The default
    /// is a bare inline box with a focus ring; include a visible focus
    /// treatment in your own.
    pub fn trigger_style_with(mut self, style: impl FnOnce(Button) -> Button + 'static) -> Self {
        self.trigger_style = Some(Box::new(style));
        self
    }
    /// Builds the popup each frame it is open.
    pub fn content<E: IntoElement>(
        mut self,
        content: impl FnOnce(&mut Window, &mut App) -> E + 'static,
    ) -> Self {
        self.content = Some(Box::new(move |window, cx| {
            content(window, cx).into_any_element()
        }));
        self
    }
    /// Sets how long the pointer rests before the popup opens, and how long
    /// it stays after the pointer leaves.
    pub fn open_on_hover(mut self, open_delay: Duration, close_delay: Duration) -> Self {
        self.open_delay = open_delay;
        self.close_delay = close_delay;
        self
    }
    /// Sets the preferred side (`gpui_kit::base::Placement`); the popup flips
    /// when that side has no room. Left and Right, and Start and End
    /// alignment, mirror in right-to-left layouts.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }
    /// Sets the alignment along the chosen side.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    /// Wraps the trigger as it is instead of making it a button, for a row
    /// that already has its own click and focus, such as a sidebar channel
    /// that opens on click and shows a preview on hover. Resting the pointer
    /// still opens the popup; clicks reach the row. The row's owner opens it
    /// from the keyboard with [`HoverPopover::open_requested`], and Escape
    /// closes it while focus is in the row or the popup. The accessible name
    /// passed to [`hover_popover`] is not used: name the row itself.
    pub fn passive(mut self) -> Self {
        self.passive = true;
        self
    }
    /// Opens the popup this frame as a click would, moving focus into it,
    /// when it is not already open: pass `true` once, in the frame after the
    /// key that asks for it (Space on a focused row). Closing gives focus
    /// back to whatever held it before.
    pub fn open_requested(mut self, requested: bool) -> Self {
        self.open_requested = requested;
        self
    }
    /// Keeps hover from opening the popup while `suppressed` says so, such
    /// as while a menu is open or something is being dragged: resting the
    /// pointer starts no timer and a pending one opens nothing. Asked when
    /// the pointer moves and again when the delay ends, so it needs no
    /// redraw to take effect. An open popup stays open.
    pub fn suppress_hover_when(
        mut self,
        suppressed: impl Fn(&Window, &App) -> bool + 'static,
    ) -> Self {
        self.suppress = Some(Rc::new(suppressed));
        self
    }
    /// Reports each open and close, whatever caused it.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for HoverPopover {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// Open state, pointer tracking and focus for one [`HoverPopover`].
#[doc(hidden)]
pub struct HoverPopoverState {
    open: bool,
    /// Opened by click or keyboard, so leaving with the pointer keeps it.
    pinned: bool,
    over_trigger: bool,
    over_content: bool,
    trigger_focus: FocusHandle,
    content_focus: FocusHandle,
    trigger_bounds: Bounds<Pixels>,
    content_bounds: Bounds<Pixels>,
    timer: Option<Task<()>>,
    on_open_change: Option<OpenChange>,
    /// What held focus before a keyboard open of a passive popover.
    return_focus: Option<FocusHandle>,
    suppress: Option<Suppress>,
}

impl HoverPopoverState {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            open: false,
            pinned: false,
            over_trigger: false,
            over_content: false,
            trigger_focus: cx.focus_handle().tab_stop(true),
            content_focus: cx.focus_handle(),
            trigger_bounds: Bounds::default(),
            content_bounds: Bounds::default(),
            timer: None,
            on_open_change: None,
            return_focus: None,
            suppress: None,
        }
    }
    /// Returns whether the popup is showing.
    pub fn is_open(&self) -> bool {
        self.open
    }
    fn set_open(&mut self, open: bool, pinned: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.timer = None;
        self.pinned = open && pinned;
        if open && pinned {
            self.content_focus.focus(window, cx);
        }
        if self.open == open {
            cx.notify();
            return;
        }
        self.open = open;
        if !open && self.content_focus.contains_focused(window, cx) {
            match self.return_focus.take() {
                Some(focus) => focus.focus(window, cx),
                None => self.trigger_focus.focus(window, cx),
            }
        }
        if let Some(handler) = self.on_open_change.clone() {
            handler(&open, window, cx);
        }
        cx.notify();
    }
    fn after(
        &mut self,
        delay: Duration,
        window: &mut Window,
        cx: &mut Context<Self>,
        then: impl FnOnce(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    ) {
        self.timer = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            _ = this.update_in(cx, |this, window, cx| then(this, window, cx));
        }));
    }
    fn suppressed(&self, window: &Window, cx: &App) -> bool {
        self.suppress
            .as_ref()
            .is_some_and(|suppressed| suppressed(window, cx))
    }
    fn pointer_moved(
        &mut self,
        open_delay: Duration,
        close_delay: Duration,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.pinned {
            return;
        }
        if self.over_trigger || self.over_content {
            if self.open || self.suppressed(window, cx) {
                self.timer = None;
            } else {
                self.after(open_delay, window, cx, |this, window, cx| {
                    if this.over_trigger && !this.suppressed(window, cx) {
                        this.set_open(true, false, window, cx);
                    }
                });
            }
        } else if self.open {
            self.after(close_delay, window, cx, |this, window, cx| {
                if !this.over_trigger && !this.over_content && !this.pinned {
                    this.set_open(false, false, window, cx);
                }
            });
        } else {
            self.timer = None;
        }
    }
}

impl RenderOnce for HoverPopover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = UiTheme::read(cx).clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| HoverPopoverState::new(cx));
        state.update(cx, |state, _| {
            state.on_open_change = self.on_open_change;
            state.suppress = self.suppress;
        });
        if self.open_requested && !state.read(cx).open {
            let focused = window.focused(cx);
            state.update(cx, |state, cx| {
                state.return_focus = focused;
                state.set_open(true, true, window, cx);
            });
        }
        let (open, focus, content_focus) = {
            let state = state.read(cx);
            (
                state.open,
                state.trigger_focus.clone(),
                state.content_focus.clone(),
            )
        };
        let (open_delay, close_delay) = (self.open_delay, self.close_delay);

        let measured = state.clone();
        let hover = state.clone();
        let click = state.clone();
        let escape = state.clone();
        let focus_theme = t.clone();
        let trigger: AnyElement = if self.passive {
            let trigger = div()
                .id((self.id.clone(), "trigger"))
                .flex_1()
                .min_w_0()
                .on_hover(move |over, window, cx| {
                    hover.update(cx, |state, cx| {
                        state.over_trigger = *over;
                        state.pointer_moved(open_delay, close_delay, window, cx);
                    })
                })
                .children(self.trigger);
            dismiss::on_escape(trigger, move |window, cx| {
                if !escape.read(cx).open {
                    return false;
                }
                escape.update(cx, |state, cx| state.set_open(false, false, window, cx));
                true
            })
            .into_any_element()
        } else {
            let trigger = Button::new((self.id.clone(), "trigger"))
                .track_focus(&focus)
                .tab_stop(true)
                .accessibility_label(self.label)
                .aria_expanded(open)
                .map(|button| match self.trigger_style {
                    Some(style) => style(button),
                    None => button
                        .flex_row_in(t.direction)
                        .items_center()
                        .rounded(t.radius.sm)
                        .cursor_pointer()
                        .focus_visible(move |style| focus::ring(style, &focus_theme)),
                })
                .on_hover(move |over, window, cx| {
                    hover.update(cx, |state, cx| {
                        state.over_trigger = *over;
                        state.pointer_moved(open_delay, close_delay, window, cx);
                    })
                })
                .on_click(move |_, window, cx| {
                    click.update(cx, |state, cx| {
                        let close = state.open && state.pinned;
                        state.set_open(!close, true, window, cx);
                    })
                })
                .children(self.trigger);
            let trigger = dismiss::on_escape(trigger, move |window, cx| {
                if !escape.read(cx).open {
                    return false;
                }
                escape.update(cx, |state, cx| state.set_open(false, false, window, cx));
                true
            });
            trigger.into_any_element()
        };

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .on_prepaint(move |bounds, _, cx| {
                measured.update(cx, |state, _| state.trigger_bounds = bounds)
            })
            .child(trigger);
        if open && let Some(content) = self.content {
            let anchor = state.read(cx).trigger_bounds;
            let hover = state.clone();
            let outside = state.clone();
            let escape = state.clone();
            let measured = state.clone();
            let passive = self.passive;
            let popup = dismiss::on_escape(div(), move |window, cx| {
                escape.update(cx, |state, cx| {
                    state.set_open(false, false, window, cx);
                    if !passive {
                        state.trigger_focus.focus(window, cx);
                    }
                });
                true
            })
            .id((self.id.clone(), "content"))
            .role(Role::Dialog)
            .tab_group()
            .track_focus(&content_focus)
            .font_family(t.fonts.body.clone())
            .on_prepaint(move |bounds, _, cx| {
                measured.update(cx, |state, _| state.content_bounds = bounds)
            })
            .on_hover(move |over, window, cx| {
                hover.update(cx, |state, cx| {
                    state.over_content = *over;
                    state.pointer_moved(open_delay, close_delay, window, cx);
                })
            })
            .on_mouse_down_out(move |event, window, cx| {
                let inside = {
                    let state = outside.read(cx);
                    state.trigger_bounds.contains(&event.position)
                        || state.content_bounds.contains(&event.position)
                };
                if !inside {
                    outside.update(cx, |state, cx| state.set_open(false, false, window, cx));
                }
            })
            .child(content(window, cx));
            root = root.child(
                gpui_kit::deferred(
                    Positioner::side(anchor)
                        .placement(t.direction.placement(self.placement))
                        .align(t.direction.align(self.align))
                        .offset(t.space(1.))
                        .occlude()
                        .child(popup),
                )
                .with_priority(POPUP_PRIORITY),
            );
        }
        apply_style(root, &self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{
        Modifiers, Render, TestAppContext, VisualTestContext, point, px, test::TestQueryExt as _,
    };

    struct Profile;
    impl Render for Profile {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .p(px(20.))
                .flex()
                .flex_col()
                .gap(px(200.))
                .child(
                    hover_popover("priya", "Priya Nair, online")
                        .trigger(div().debug_selector(|| "trigger".into()).size(px(32.)))
                        .content(|_, cx| {
                            popover_popup("priya.card", "Priya Nair", cx)
                                .debug_selector(|| "card".into())
                                .h(px(80.))
                                .child(
                                    super::super::button::Button::new("priya.message")
                                        .label("Message"),
                                )
                        }),
                )
                .child(div().debug_selector(|| "elsewhere".into()).size(px(10.)))
                .when(false, |root| root.child(popover_title(cx)))
        }
    }

    /// A row with its own click and focus, previewed by a passive popover.
    struct Row {
        clicks: usize,
        request: bool,
        focus: FocusHandle,
        busy: Rc<std::cell::Cell<bool>>,
    }
    impl Render for Row {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let request = std::mem::take(&mut self.request);
            let busy = self.busy.clone();
            div()
                .p(px(20.))
                .flex()
                .flex_col()
                .gap(px(200.))
                .child(
                    hover_popover("row", "unused")
                        .passive()
                        .open_requested(request)
                        .suppress_hover_when(move |_, _| busy.get())
                        .trigger(
                            div()
                                .id("row.button")
                                .track_focus(&self.focus)
                                .debug_selector(|| "row".into())
                                .size(px(32.))
                                .on_click(cx.listener(|this, _, _, _| this.clicks += 1)),
                        )
                        .content(|_, cx| {
                            popover_popup("row.card", "Preview", cx)
                                .debug_selector(|| "card".into())
                                .h(px(80.))
                        }),
                )
                .child(div().debug_selector(|| "elsewhere".into()).size(px(10.)))
        }
    }

    #[gpui_kit::test]
    fn a_passive_popover_leaves_clicks_to_the_row(cx: &mut TestAppContext) {
        cx.update(super::super::theme::init);
        let busy = Rc::new(std::cell::Cell::new(false));
        let (view, cx) = cx.add_window_view({
            let busy = busy.clone();
            move |_, cx| Row {
                clicks: 0,
                request: false,
                focus: cx.focus_handle(),
                busy,
            }
        });
        cx.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
        });
        let row = cx.debug_bounds("row").unwrap();
        cx.simulate_click(row.center(), Modifiers::none());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(view.read_with(cx, |row, _| row.clicks), 1);
        assert!(
            cx.debug_bounds("card").is_none(),
            "a click does not open it"
        );

        // Hover still opens it.
        cx.simulate_mouse_move(row.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_OPEN_DELAY);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_some(), "hover opens it");
        let elsewhere = cx.debug_bounds("elsewhere").unwrap();
        cx.simulate_mouse_move(elsewhere.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_CLOSE_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_none());

        // While suppressed, resting opens nothing, and a timer already
        // running when it starts opens nothing either.
        busy.set(true);
        cx.simulate_mouse_move(row.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_OPEN_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("card").is_none(),
            "suppressed hover opens nothing"
        );
        busy.set(false);
        cx.simulate_mouse_move(elsewhere.center(), None, Modifiers::none());
        cx.simulate_mouse_move(row.center(), None, Modifiers::none());
        busy.set(true);
        cx.executor().advance_clock(HOVER_POPOVER_OPEN_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("card").is_none(),
            "a pending open is dropped"
        );
        busy.set(false);
        cx.simulate_mouse_move(elsewhere.center(), None, Modifiers::none());
        cx.run_until_parked();

        // The row's owner opens it from the keyboard; Escape gives focus back.
        let focus = view.read_with(cx, |row, _| row.focus.clone());
        cx.update(|window, cx| focus.focus(window, cx));
        view.update(cx, |row, cx| {
            row.request = true;
            cx.notify();
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_some(), "the request opens it");
        cx.update(|window, _| assert!(!focus.is_focused(window), "focus moved into it"));
        cx.simulate_keystrokes("escape");
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_none(), "Escape closes it");
        cx.update(|window, _| assert!(focus.is_focused(window), "focus is back on the row"));
    }

    fn profile(cx: &mut TestAppContext) -> &mut VisualTestContext {
        cx.update(super::super::theme::init);
        let (_, cx) = cx.add_window_view(|_, _| Profile);
        cx.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
        });
        cx
    }

    // The test window does not track the pointer between frames, so a frame
    // drawn after a move reports the trigger as left. These checks draw only
    // where a frame's hover change is harmless, as GPUI Kit's own do.
    #[gpui_kit::test]
    fn hover_opens_after_a_delay_without_taking_focus(cx: &mut TestAppContext) {
        let cx = profile(cx);
        let trigger = cx.debug_bounds("trigger").unwrap();
        cx.simulate_mouse_move(trigger.center(), None, Modifiers::none());
        cx.executor()
            .advance_clock(HOVER_POPOVER_OPEN_DELAY - Duration::from_millis(10));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_none(), "not before the delay");
        cx.simulate_mouse_move(trigger.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_OPEN_DELAY);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let card = cx.debug_bounds("card").expect("open after the delay");
        cx.update(|window, cx| {
            assert!(
                window.focused(cx).is_none(),
                "hover keeps focus where it was"
            )
        });

        // The pointer can travel into the card without closing it.
        cx.simulate_mouse_move(card.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_CLOSE_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("card").is_some(),
            "still open over the card"
        );

        let elsewhere = cx.debug_bounds("elsewhere").unwrap();
        cx.simulate_mouse_move(elsewhere.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_CLOSE_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("card").is_none(),
            "closed after the pointer left"
        );
    }

    #[gpui_kit::test]
    fn a_quick_pass_over_the_trigger_opens_nothing(cx: &mut TestAppContext) {
        let cx = profile(cx);
        let trigger = cx.debug_bounds("trigger").unwrap();
        let elsewhere = cx.debug_bounds("elsewhere").unwrap();
        cx.simulate_mouse_move(trigger.center(), None, Modifiers::none());
        cx.simulate_mouse_move(elsewhere.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_OPEN_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_none());
    }

    #[gpui_kit::test]
    fn the_keyboard_opens_it_into_focus_and_escape_returns(cx: &mut TestAppContext) {
        let cx = profile(cx);
        cx.update(|window, cx| {
            let trigger = window.find_by_label("Priya Nair, online");
            assert_eq!(trigger.role(), Some(Role::Button));
            window.focus_next(cx);
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|window, _| {
            assert_eq!(
                window.find_by_label("Priya Nair, online").focused(),
                Some(true)
            );
        });
        let enter = gpui_kit::Keystroke::parse("enter").unwrap();
        cx.simulate_event(gpui_kit::KeyDownEvent {
            keystroke: enter.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        cx.simulate_event(gpui_kit::KeyUpEvent { keystroke: enter });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_some(), "Enter opens it");
        cx.update(|window, _| {
            assert_eq!(
                window.find_by_label("Priya Nair, online").focused(),
                Some(false)
            );
        });
        // Opened by the keyboard, it stays open when the pointer moves away.
        let elsewhere = cx.debug_bounds("elsewhere").unwrap();
        cx.simulate_mouse_move(elsewhere.center(), None, Modifiers::none());
        cx.executor().advance_clock(HOVER_POPOVER_CLOSE_DELAY * 2);
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_some());

        cx.simulate_keystrokes("escape");
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_none(), "Escape closes it");
        cx.update(|window, _| {
            assert_eq!(
                window.find_by_label("Priya Nair, online").focused(),
                Some(true)
            );
        });
    }

    #[gpui_kit::test]
    fn a_click_opens_it_and_a_click_outside_closes_it(cx: &mut TestAppContext) {
        let cx = profile(cx);
        let trigger = cx.debug_bounds("trigger").unwrap();
        cx.simulate_click(trigger.center(), Modifiers::none());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_some());
        cx.simulate_click(point(px(500.), px(500.)), Modifiers::none());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("card").is_none());
    }
}

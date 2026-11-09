//! Spinners and skeletons step a dozen times a second instead of asking for
//! every frame, and stop while their window is in the background.

use std::time::Duration;

use gpui_kit::{
    Context, IntoElement, ParentElement as _, Render, Styled as _, TestAppContext, Window, div, px,
};
use gpuicn::{skeleton::Skeleton, spinner::Spinner};

struct Loading {
    renders: usize,
}

impl Render for Loading {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders += 1;
        div()
            .child(Spinner::new("spin"))
            .child(Skeleton::new("skeleton").w(px(80.)))
    }
}

/// Plays `seconds` of frames, a frame each 16ms of simulated time, and
/// returns how many times something asked the view to draw again.
fn redraws_over(
    view: &gpui_kit::Entity<Loading>,
    cx: &mut gpui_kit::VisualTestContext,
    seconds: u64,
) -> usize {
    let asked = std::rc::Rc::new(std::cell::Cell::new(0));
    let count = asked.clone();
    let _watch = cx.update(|_, cx| cx.observe(view, move |_, _| count.set(count.get() + 1)));
    for _ in 0..seconds * 60 {
        cx.executor().advance_clock(Duration::from_millis(16));
        cx.run_until_parked();
        // A frame: what waited for it runs, then the window draws.
        cx.update(|window, cx| {
            window.simulate_next_frame(cx);
            window.draw(cx).clear(cx);
        });
    }
    asked.get()
}

#[gpui_kit::test]
fn loops_redraw_at_their_step_not_every_frame(cx: &mut TestAppContext) {
    cx.update(gpuicn::init);
    let (view, cx) = cx.add_window_view(|_, _| Loading { renders: 0 });
    cx.update(|window, cx| {
        window.activate_window();
        window.draw(cx).clear(cx);
    });
    let renders = redraws_over(&view, cx, 2);
    assert!((18..=30).contains(&renders), "{renders} redraws in 2s");
}

#[gpui_kit::test]
fn loops_stop_in_a_background_window(cx: &mut TestAppContext) {
    cx.update(gpuicn::init);
    let (view, cx) = cx.add_window_view(|_, _| Loading { renders: 0 });
    cx.deactivate_window();
    cx.update(|window, cx| window.draw(cx).clear(cx));
    // The step already asked for, at most, then nothing.
    let renders = redraws_over(&view, cx, 2);
    assert!(renders <= 1, "{renders} redraws in 2s");
}

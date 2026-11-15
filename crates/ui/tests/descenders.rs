//! Text inside a control keeps the bottoms of g, p and y.
//!
//! GPUI Kit's base `Button`, `Toggle` and `Tab` set a line box exactly the font
//! size. Glyphs reach further than that (1.3em from accent to descender in
//! Geist and IBM Plex Sans, and in GPUI's test font metrics), and `truncate()`
//! clips to the box, so a truncated label lost its descenders in every menu,
//! button and tab. Each case here measures the box a truncated label gets
//! inside one control and checks the glyphs fit in it.

use gpui_kit::{
    AppContext as _, Context, Entity, InteractiveElement as _, IntoElement, ParentElement, Render,
    Styled, TestAppContext, VisualTestContext, Window, div, px, size,
};
use gpuicn::{
    Button, ButtonSize,
    badge::Badge,
    collapsible::collapsible_trigger,
    item::Item,
    menu::{Menu, MenuItem, MenuState},
    tabs::{Tab, Tabs},
    theme::primitives::text,
    toolbar::{Toolbar, ToolbarButton},
};

const SIZES: [(ButtonSize, &str); 4] = [
    (ButtonSize::Xs, "button-xs"),
    (ButtonSize::Sm, "button-sm"),
    (ButtonSize::Default, "button-default"),
    (ButtonSize::Lg, "button-lg"),
];

fn label(selector: &'static str) -> impl IntoElement {
    div()
        .debug_selector(move || selector.into())
        .truncate()
        .child("Typography, gpy")
}

struct Gallery {
    menu: Entity<MenuState>,
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(8.))
            .p(px(16.))
            .children(
                SIZES.map(|(size, selector)| {
                    Button::new(selector).size(size).child(label(selector))
                }),
            )
            .child(Toolbar::new("toolbar", "Format", cx).button(
                ToolbarButton::new("toolbar.bold", "Bold", cx).child(label("toolbar-button")),
            ))
            .child(collapsible_trigger("details", false, cx).child(label("collapsible-trigger")))
            .child(
                Tabs::new("tabs")
                    .item(Tab::new("general", "general", "General").trailing(label("tab-trailing")))
                    .selected("general"),
            )
            .child(
                Item::new("item")
                    .title("Typography")
                    .description("Spacing, glyphs and type")
                    .on_click(|_, _, _| {}),
            )
            .child(
                div()
                    .debug_selector(|| "badge".into())
                    .child(Badge::new("pending")),
            )
            .child(Menu::new(&self.menu, "Servers"))
    }
}

fn open(cx: &mut TestAppContext) -> (Entity<Gallery>, VisualTestContext) {
    cx.update(gpuicn::init);
    let (view, cx) = cx.add_window_view(|_, cx| Gallery {
        menu: cx.new(|cx| {
            MenuState::new(
                [MenuItem::new("community", "Matty Community")
                    .description("community.matty.example")],
                cx,
            )
            .width(px(280.))
        }),
    });
    let mut cx = cx.clone();
    cx.simulate_resize(size(px(800.), px(900.)));
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.update(|window, cx| {
        let menu = view.read(cx).menu.clone();
        menu.update(cx, |menu, cx| menu.open(window, cx));
    });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.update(|window, cx| window.draw(cx).clear(cx));
    (view, cx)
}

/// The box drawn for `selector` is at least as tall as glyphs at `font_size`.
fn assert_glyphs_fit(cx: &mut VisualTestContext, selector: &'static str, font_size: f32) {
    let bounds = cx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} was not drawn"));
    let ink = px(font_size * text::INK);
    assert!(
        bounds.size.height >= ink - px(0.01),
        "{selector}: a {font_size}px label gets a {:?} line box, shorter than its {ink:?} glyphs; \
         truncate() would clip the bottoms of g, p and y. Set the text through \
         `text::control` or `text::line_height`.",
        bounds.size.height
    );
}

#[gpui_kit::test]
fn truncated_labels_inside_controls_keep_their_descenders(cx: &mut TestAppContext) {
    let (_view, mut cx) = open(cx);
    for (selector, font_size) in [
        ("button-xs", 12.),
        ("button-sm", 12.8),
        ("button-default", 14.),
        ("button-lg", 14.),
        ("toolbar-button", 14.),
        ("collapsible-trigger", 14.),
        ("tab-trailing", 14.),
        ("item-title", 13.5),
        ("item-description", 12.),
        ("menu-label", 14.),
        ("menu-description", 12.),
    ] {
        assert_glyphs_fit(&mut cx, selector, font_size);
    }
}

#[gpui_kit::test]
fn a_menu_row_with_a_description_stays_42px(cx: &mut TestAppContext) {
    let (_view, mut cx) = open(cx);
    let label = cx.debug_bounds("menu-label").expect("label drawn");
    let description = cx
        .debug_bounds("menu-description")
        .expect("description drawn");
    // 3px + 20px name + 16px description + 3px: the 42px the row had when
    // both lines were squeezed into font-size boxes with a gap and more padding.
    assert_eq!(description.bottom() - label.top(), px(36.));
}

#[gpui_kit::test]
fn line_heights_never_fall_below_the_glyphs(cx: &mut TestAppContext) {
    cx.update(|cx| {
        gpuicn::init(cx);
        let theme = gpuicn::UiTheme::read(cx).clone();
        for size in [
            9., 10., 10.5, 11., 12., 12.8, 13., 13.5, 14., 15., 16., 18., 20., 24.,
        ] {
            let size = px(size);
            let line = text::line_height(&theme, size);
            assert!(line >= size * text::INK, "{size:?} gets {line:?}");
        }
        assert_eq!(text::line_height(&theme, px(14.)), px(20.));
        assert_eq!(text::line_height(&theme, px(12.)), px(16.));
    });
}

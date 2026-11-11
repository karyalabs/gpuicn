//! The component gallery: every catalog preview in one window, grouped and
//! searchable. It runs natively (`showcase --catalog`) and in the browser,
//! where the website serves it as a full-page GPUI app.

use super::*;
use gpui_kit::{Entity, Pixels};
use gpuicn::badge::Badge;
use gpuicn::sidebar::{Sidebar, SidebarItem, sidebar_group_label};

gpui_kit::actions!(gallery, [Quit]);

/// One gallery entry. `build.rs` writes the table from registry.toml.
pub(crate) struct Entry {
    pub(crate) slug: &'static str,
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) category: &'static str,
    pub(crate) new: bool,
}

include!(concat!(env!("OUT_DIR"), "/catalog.rs"));

/// The icon browser has a preview but is not a registry component.
const ICONS: Entry = Entry {
    slug: "icons",
    title: "Icons",
    description: "Browse the Lucide icons used throughout the components.",
    category: "foundations",
    new: false,
};

/// Entries are addressed by position; the icon browser follows the catalog.
fn entry(index: usize) -> &'static Entry {
    COMPONENTS.get(index).unwrap_or(&ICONS)
}

fn entries() -> impl Iterator<Item = (usize, &'static Entry)> {
    (0..=COMPONENTS.len()).map(|index| (index, entry(index)))
}

fn position(slug: &str) -> Option<usize> {
    entries().find_map(|(index, entry)| (entry.slug == slug).then_some(index))
}

/// The gallery's theme: the Workspace app's monochrome pair natively, and in
/// the browser whatever the site's theme menu asks for.
#[cfg(not(target_family = "wasm"))]
fn gallery_theme(mode: ThemeMode) -> UiTheme {
    super::workspace::workspace_theme(mode)
}
#[cfg(target_family = "wasm")]
fn gallery_theme(mode: ThemeMode) -> UiTheme {
    super::site_theme::theme(mode, &requested_value("style").unwrap_or_default())
}

/// Installs the theme the framing page asked for.
#[cfg(target_family = "wasm")]
pub(super) fn set_theme(cx: &mut App, theme: UiTheme) {
    if *UiTheme::read(cx) != theme {
        UiTheme::set(cx, theme);
    }
}

pub(super) fn launch(cx: &mut App) {
    UiTheme::set(cx, gallery_theme(UiTheme::read(cx).mode));
    #[cfg(not(target_family = "wasm"))]
    {
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([gpui_kit::KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus([gpui_kit::Menu::new("gpuicn Showcase")
            .items([gpui_kit::MenuItem::action("Quit gpuicn Showcase", Quit)])]);
    }
    open(cx);
}

pub(super) fn open(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(1180.), px(780.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(800.), px(600.))),
            ..Default::default()
        },
        |window, cx| {
            window.set_window_title("gpuicn Showcase");
            cx.new(Gallery::new)
        },
    )
    .expect("failed to open the gallery window");
    cx.activate(true);
}

struct Gallery {
    selected: usize,
    query: String,
    preview: Entity<Showcase>,
    sidebar_width: Pixels,
}

impl Gallery {
    fn new(cx: &mut Context<Self>) -> Self {
        // In the browser `?component=calendar` opens the gallery on that entry.
        let selected = requested_value("component")
            .and_then(|slug| position(&slug))
            .or_else(|| position("button"))
            .unwrap_or(0);
        let demo = Demo::parse(entry(selected).slug).unwrap_or_default();
        Self {
            selected,
            query: String::new(),
            preview: cx.new(|_| Showcase::new(demo)),
            sidebar_width: px(240.),
        }
    }

    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected = index;
        self.preview = cx.new(|_| Showcase::new(Demo::parse(entry(index).slug).unwrap()));
        #[cfg(target_family = "wasm")]
        super::gallery_selected(entry(index).slug);
        cx.notify();
    }
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search = live_input(
            "gallery.search",
            self.query.clone(),
            "Find a component…",
            window,
            cx,
            |this, input, event, cx| {
                if matches!(event, InputEvent::Change) {
                    this.query = input.read(cx).value().to_string();
                    cx.notify();
                }
            },
        );
        let theme = UiTheme::read(cx).clone();
        let current = entry(self.selected);
        let dark = theme.mode == ThemeMode::Dark;
        let query = self.query.trim().to_lowercase();
        let mut sidebar = Sidebar::new("gallery.components", "Components")
            .header(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .p(px(8.))
                    .child(
                        div()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .child("gpuicn"),
                    )
                    .child(Input::new(&search).aria_label("Find a component")),
            )
            .footer(
                div()
                    .px(px(8.))
                    .py(px(4.))
                    .text_size(px(12.))
                    .text_color(theme.colors.muted_foreground)
                    .child(format!("{} components · drawn by GPUI", COMPONENTS.len())),
            );
        let mut matches = 0;
        let item = |index: usize, entry: &'static Entry, cx: &mut Context<Self>| {
            SidebarItem::new(("gallery.item", index), entry.title)
                .selected(index == self.selected)
                .when(entry.new, |item| {
                    item.trailing(Badge::new("New").variant(gpuicn::badge::BadgeVariant::Secondary))
                })
                .on_activate(cx.listener(move |this, _, _, cx| this.select(index, cx)))
        };
        if query.is_empty() {
            // Browsing: one labelled group per category, in registry.toml order.
            for (id, title) in CATEGORIES {
                let group: Vec<_> = entries()
                    .filter(|(_, entry)| entry.category == *id)
                    .collect();
                if group.is_empty() {
                    continue;
                }
                sidebar = sidebar.child(sidebar_group_label(*title, cx));
                for (index, entry) in group {
                    matches += 1;
                    sidebar = sidebar.child(item(index, entry, cx));
                }
            }
        } else {
            // Searching: a flat A to Z list of the matches.
            for (index, entry) in entries() {
                if entry.title.to_lowercase().contains(&query)
                    || entry.description.to_lowercase().contains(&query)
                    || entry.slug.contains(&query)
                {
                    matches += 1;
                    sidebar = sidebar.child(item(index, entry, cx));
                }
            }
        }
        if matches == 0 {
            sidebar = sidebar.child(
                div()
                    .p(px(8.))
                    .text_color(theme.colors.muted_foreground)
                    .child("No components found"),
            );
        }
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(16.))
            .px(px(24.))
            .py(px(18.))
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .child(
                                div()
                                    .text_size(px(20.))
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .child(current.title),
                            )
                            .when(current.new, |row| row.child(Badge::new("New"))),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.colors.muted_foreground)
                            .child(current.description),
                    ),
            )
            .child(
                Button::new("gallery.docs")
                    .aria_label(format!("Open the {} documentation", current.title))
                    .variant(ButtonVariant::Outline)
                    .label("Docs")
                    .on_click(cx.listener(|this, _, _, cx| {
                        open_docs(entry(this.selected).slug, cx);
                    })),
            )
            .child(
                Button::new("gallery.reset")
                    .aria_label("Reset example")
                    .variant(ButtonVariant::Outline)
                    .label("Reset")
                    .on_click(cx.listener(|this, _, _, cx| this.select(this.selected, cx))),
            )
            // In the browser the site's header has the theme switch.
            .when(cfg!(not(target_family = "wasm")), |header| {
                header.child(
                    Button::new("gallery.theme")
                        .aria_label("Toggle light and dark theme")
                        .variant(ButtonVariant::Outline)
                        .child(if dark { "Light" } else { "Dark" })
                        .on_click(cx.listener(move |_, _, _, cx| {
                            UiTheme::set(
                                cx,
                                gallery_theme(if dark {
                                    ThemeMode::Light
                                } else {
                                    ThemeMode::Dark
                                }),
                            );
                        })),
                )
            });
        let content = div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .child(header)
            .when(current.slug == "sidebar", |el| {
                el.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(4.))
                        .px(px(16.))
                        .py(px(8.))
                        .border_b_1()
                        .border_color(theme.colors.border)
                        .children(
                            [
                                ("workspace", "Workspace"),
                                ("docs", "Docs"),
                                ("mail", "Mail"),
                                ("floating", "Floating"),
                                ("mobile", "Sheet"),
                                ("loading", "Loading"),
                            ]
                            .into_iter()
                            .map(|(value, label)| {
                                Button::new(format!("gallery.sidebar.{value}"))
                                    .label(label)
                                    .aria_label(format!("Show {label} sidebar"))
                                    .variant(if self.preview.read(cx).sidebar_example == value {
                                        ButtonVariant::Secondary
                                    } else {
                                        ButtonVariant::Ghost
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.preview = cx.new(|_| {
                                            let mut view = Showcase::new(Demo::Sidebar);
                                            view.sidebar_example = value.into();
                                            view
                                        });
                                        cx.notify();
                                    }))
                            }),
                        ),
                )
            })
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .child(self.preview.clone()),
            );
        div()
            .size_full()
            .bg(theme.colors.background)
            .text_color(theme.colors.foreground)
            .font_family(theme.fonts.body)
            .text_size(px(14.))
            .child(
                gpuicn::resizable::Resizable::new(
                    "gallery.split",
                    "Resize component navigation",
                    self.sidebar_width,
                )
                .first(sidebar)
                .second(content)
                .on_resize(cx.listener(|this, value, _, cx| {
                    this.sidebar_width = *value;
                    cx.notify();
                }))
                .first_limits(gpuicn::resizable::PaneLimits::new(px(190.), px(360.)))
                .second_limits(gpuicn::resizable::PaneLimits::new(px(480.), px(2000.))),
            )
    }
}

/// Opens a component's page on the website. The icon browser's page is /icons/.
fn open_docs(slug: &str, cx: &mut App) {
    let path = if slug == "icons" {
        "icons/".to_owned()
    } else {
        format!("components/{slug}/")
    };
    // In the browser the gallery is framed by the site, which follows the
    // link; the previews are served from `demo/`, beside the pages.
    #[cfg(target_family = "wasm")]
    {
        let _ = cx;
        super::gallery_open(&format!("../{path}"));
    }
    #[cfg(not(target_family = "wasm"))]
    cx.open_url(&format!("https://karyalabs.dev/gpuicn/{path}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{Modifiers, TestAppContext, VisualTestContext, point};

    #[test]
    fn gallery_navigates_the_full_catalog_and_searches_without_reborrowing() {
        // Every catalog entry has a preview and every preview has an entry, so
        // a component in registry.toml can never ship unseen.
        for (_, entry) in entries() {
            assert!(
                Demo::parse(entry.slug).is_some(),
                "no preview for {}",
                entry.slug
            );
            assert!(
                CATEGORIES.iter().any(|(id, _)| *id == entry.category),
                "{} has an unknown category",
                entry.slug
            );
        }
        // App is the starter window: it has a preview for the screenshot
        // sheet but no catalog entry.
        for demo in Demo::ALL.iter().filter(|demo| demo.slug() != "app") {
            assert!(
                position(&demo.slug().replace('_', "-")).is_some(),
                "registry.toml has no entry for the {} preview",
                demo.slug()
            );
        }
        // Declared first so it is dropped after `cx`: GPUI Kit 0.8 deadlocks when
        // an app is dropped after its test scheduler.
        let dispatcher = gpui_kit::TestDispatcher::new(0);
        let mut cx = TestAppContext::build(dispatcher.clone(), None);
        cx.update(|cx| {
            gpuicn::init(cx);
            UiTheme::set(cx, UiTheme::neutral_light());
        });
        let window = cx.add_window(|_, cx| Gallery::new(cx));
        let mut visual = VisualTestContext::from_window(window.into(), &cx);
        visual.simulate_resize(size(px(1180.), px(780.)));
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            cx.update(|cx| UiTheme::switch(cx, mode));
            for (index, _) in entries() {
                window
                    .update(&mut cx, |view, _, cx| view.select(index, cx))
                    .unwrap();
                visual.update(|window, cx| window.draw(cx).clear(cx));
            }
        }
        visual.simulate_click(point(px(100.), px(69.)), Modifiers::default());
        for character in ["v", "i", "r", "t", "u", "a", "l"] {
            visual.simulate_input(character);
            cx.run_until_parked();
            visual.update(|window, cx| window.draw(cx).clear(cx));
        }
        assert_eq!(
            cx.read_window(&window, |view, cx| view.read(cx).query.clone())
                .unwrap(),
            "virtual"
        );
        visual.simulate_click(point(px(100.), px(126.)), Modifiers::default());
        visual.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            cx.read_window(&window, |view, cx| entry(view.read(cx).selected).slug)
                .unwrap(),
            "virtual-list"
        );
    }
}

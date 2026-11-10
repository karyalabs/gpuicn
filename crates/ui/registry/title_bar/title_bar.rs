//! Nova custom window title bar with platform window controls.
//!
//! Behavior follows GPUI Kit 0.8.1's `TitleBar`: the bar drags the window,
//! double-clicks zoom it, macOS keeps its traffic lights, and Windows and
//! client-decorated Linux windows get minimize, maximize and close controls.

use std::rc::Rc;

use gpui_icons::{LucideIcon, lucide};
use gpui_kit::{
    AnyElement, App, ClickEvent, Decorations, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled,
    TitlebarOptions, Window, WindowControlArea, WindowOptions, div, prelude::FluentBuilder as _,
    px,
};

use super::theme::{UiTheme, apply_style};

type CloseHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// Title bar height, matching GPUI Kit so the traffic lights stay centered.
pub const TITLE_BAR_HEIGHT: gpui_kit::Pixels = px(34.);

/// Window options for a window that draws a `TitleBar`. Spread your own options
/// over these: `WindowOptions { ..title_bar_window_options() }`.
pub fn title_bar_window_options() -> WindowOptions {
    gpui_kit::component::TitleBar::window_options()
}

/// macOS titlebar options that place the traffic lights inside a `TitleBar`.
pub fn title_bar_options() -> TitlebarOptions {
    gpui_kit::component::TitleBar::title_bar_options()
}

/// A draggable title bar that hosts application content, such as a search
/// field or workspace switcher, beside the platform window controls.
#[derive(IntoElement)]
pub struct TitleBar {
    style: gpui_kit::StyleRefinement,
    children: Vec<AnyElement>,
    on_close: Option<CloseHandler>,
}

impl TitleBar {
    /// Creates an empty title bar.
    pub fn new() -> Self {
        Self {
            style: gpui_kit::StyleRefinement::default(),
            children: Vec::new(),
            on_close: None,
        }
    }

    /// Runs instead of closing the window when the close control is clicked on
    /// Linux, for example to hide to the tray. Windows and macOS close natively.
    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for TitleBar {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Control {
    Minimize,
    Maximize,
    Restore,
    Close,
}

impl Control {
    fn icon(self) -> LucideIcon {
        match self {
            Self::Minimize => LucideIcon::Minus,
            Self::Maximize => LucideIcon::Square,
            Self::Restore => LucideIcon::Copy,
            Self::Close => LucideIcon::X,
        }
    }
    fn text(self) -> super::theme::UiText {
        use super::theme::UiText;
        match self {
            Self::Minimize => UiText::Minimize,
            Self::Maximize => UiText::Maximize,
            Self::Restore => UiText::Restore,
            Self::Close => UiText::Close,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Minimize => "Minimize",
            Self::Maximize => "Maximize",
            Self::Restore => "Restore",
            Self::Close => "Close",
        }
    }
    fn area(self) -> WindowControlArea {
        match self {
            Self::Minimize => WindowControlArea::Min,
            Self::Maximize | Self::Restore => WindowControlArea::Max,
            Self::Close => WindowControlArea::Close,
        }
    }
}

fn control(
    kind: Control,
    on_close: Option<CloseHandler>,
    theme: &UiTheme,
    cx: &App,
) -> impl IntoElement {
    let colors = theme.colors;
    let (hover_bg, hover_fg) = if kind == Control::Close {
        (colors.destructive, colors.destructive_foreground)
    } else {
        (colors.accent, colors.accent_foreground)
    };
    div()
        .id(SharedString::from(kind.label()))
        .aria_label(super::theme::locale::text(cx, kind.text()))
        .flex()
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .w(px(46.))
        .h_full()
        .text_color(colors.muted_foreground)
        .hover(move |style| style.bg(hover_bg).text_color(hover_fg))
        .when(cfg!(target_os = "windows"), |this| {
            this.window_control_area(kind.area())
        })
        .when(cfg!(target_os = "linux"), |this| {
            this.on_mouse_down(MouseButton::Left, |_, window, cx| {
                window.prevent_default();
                cx.stop_propagation();
            })
            .on_click(move |event, window, cx| {
                cx.stop_propagation();
                match kind {
                    Control::Minimize => window.minimize_window(),
                    Control::Maximize | Control::Restore => window.zoom_window(),
                    Control::Close => match &on_close {
                        Some(handler) => handler(event, window, cx),
                        None => window.remove_window(),
                    },
                }
            })
        })
        .child(
            lucide(kind.icon())
                .size(if kind == Control::Close {
                    theme.space(4.)
                } else {
                    theme.space(3.5)
                })
                .text_color(colors.muted_foreground),
        )
}

struct DragState {
    should_move: bool,
}

/// Whether this window draws its own minimize, maximize and close controls:
/// Windows, and Linux under client-side decorations. macOS keeps its traffic
/// lights, a server-decorated Linux window gets the window manager's, and the
/// web has none.
pub fn draws_window_controls(window: &Window) -> bool {
    let client_decorated = matches!(window.window_decorations(), Decorations::Client { .. });
    let is_linux = cfg!(target_os = "linux");
    !cfg!(target_os = "macos") && !cfg!(target_family = "wasm") && (!is_linux || client_decorated)
}

/// The left inset that keeps content clear of the macOS traffic lights, 80px
/// outside full screen; zero elsewhere.
pub fn traffic_light_inset(window: &Window) -> gpui_kit::Pixels {
    if cfg!(target_os = "macos") && !window.is_fullscreen() {
        px(80.)
    } else {
        px(0.)
    }
}

/// Makes `element` drag the window as a title bar does, for an app without a
/// `TitleBar` whose pane headers or sidebar top are the window's top edge:
/// dragging empty space moves the window, a double-click zooms it (macOS
/// follows the user's titlebar double-click preference), and on client-
/// decorated Linux a right-click opens the window menu. Buttons inside keep
/// their clicks. Does nothing on the web. `id` keys the drag state, so give
/// each area in a window its own.
pub fn window_drag_area<E>(
    id: impl Into<gpui_kit::ElementId>,
    element: E,
    window: &mut Window,
    cx: &mut App,
) -> E
where
    E: gpui_kit::StatefulInteractiveElement + gpui_kit::base::InteractiveElementExt + IntoElement,
{
    if cfg!(target_family = "wasm") {
        return element;
    }
    let state = window.use_keyed_state(id, cx, |_, _| DragState { should_move: false });
    let client_decorated = matches!(window.window_decorations(), Decorations::Client { .. });
    element
        .when(cfg!(target_os = "linux"), |el| {
            el.on_double_click(|_, window, _| window.zoom_window())
        })
        .when(cfg!(target_os = "macos"), |el| {
            el.on_double_click(|_, window, _| window.titlebar_double_click())
        })
        .when(cfg!(target_os = "linux") && client_decorated, |el| {
            el.on_mouse_down(MouseButton::Right, |event, window, _| {
                window.show_window_menu(event.position)
            })
        })
        .on_mouse_down_out(window.listener_for(&state, |state, _, _, _| {
            state.should_move = false;
        }))
        .on_mouse_down(
            MouseButton::Left,
            window.listener_for(&state, |state, _, _, _| state.should_move = true),
        )
        .on_mouse_up(
            MouseButton::Left,
            window.listener_for(&state, |state, _, _, _| state.should_move = false),
        )
        .on_mouse_move(window.listener_for(&state, |state, _, window, _| {
            if state.should_move {
                state.should_move = false;
                window.start_window_move();
            }
        }))
        .window_control_area(WindowControlArea::Drag)
}

/// The minimize, maximize (or restore) and close controls a `TitleBar` draws,
/// on their own, for the end of a pane header in a window without a title
/// bar. Renders nothing where [`draws_window_controls`] is false.
#[derive(IntoElement)]
pub struct WindowControls {
    style: gpui_kit::StyleRefinement,
    on_close: Option<CloseHandler>,
}

impl WindowControls {
    /// Creates the controls for the current window.
    pub fn new() -> Self {
        Self {
            style: gpui_kit::StyleRefinement::default(),
            on_close: None,
        }
    }

    /// Runs instead of closing the window when close is clicked on Linux, as
    /// [`TitleBar::on_close`] does.
    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Default for WindowControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for WindowControls {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for WindowControls {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let supported = window.window_controls();
        let maximized = window.is_maximized();
        let row =
            div()
                .flex()
                .h_full()
                .flex_shrink_0()
                .when(draws_window_controls(window), |row| {
                    row.when(supported.minimize, |row| {
                        row.child(control(Control::Minimize, None, &theme, cx))
                    })
                    .when(supported.maximize, |row| {
                        row.child(control(
                            if maximized {
                                Control::Restore
                            } else {
                                Control::Maximize
                            },
                            None,
                            &theme,
                            cx,
                        ))
                    })
                    .child(control(
                        Control::Close,
                        self.on_close.clone(),
                        &theme,
                        cx,
                    ))
                });
        apply_style(row, &self.style)
    }
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let is_web = cfg!(target_family = "wasm");
        let left_inset = traffic_light_inset(window).max(theme.space(3.));
        let bar = div()
            .id("title-bar")
            .flex_shrink_0()
            .w_full()
            .h(TITLE_BAR_HEIGHT)
            .flex()
            .items_center()
            .pl(left_inset)
            .border_b_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.sidebar)
            .text_color(theme.colors.foreground)
            .font_family(theme.fonts.body.clone())
            .text_size(theme.text(13.));
        let bar = window_drag_area("title-bar.drag", bar, window, cx);
        let mut controls = WindowControls::new();
        controls.on_close = self.on_close.clone();
        let bar = bar
            .child(
                div()
                    .id("title-bar-content")
                    .h_full()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .gap(theme.space(2.))
                    .when(!is_web, |content| {
                        content.window_control_area(WindowControlArea::Drag)
                    })
                    .children(self.children),
            )
            .child(controls);
        apply_style(bar, &self.style)
    }
}

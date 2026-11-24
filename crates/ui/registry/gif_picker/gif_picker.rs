//! Nova GIF picker: a search field and a two-column masonry grid of GIFs that
//! loads more as it scrolls.
//!
//! The application owns the GIF service: it answers `GifPickerEvent::Search`
//! and `LoadMore` with `set_gifs` and `extend_gifs`, and resolves each GIF's
//! preview image. gpuicn never fetches anything itself.

use gpui_icons::{LucideIcon, lucide};
use gpui_kit::base::TestSupportExt as _;
use gpui_kit::{
    App, AppContext as _, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    ImageSource, InteractiveElement as _, IntoElement, Keystroke, ObjectFit, ParentElement as _,
    Pixels, Render, RenderOnce, Role, ScrollHandle, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Subscription, Window, div, point, prelude::FluentBuilder as _, px,
};

use super::input::{Input, InputEvent, InputState};
use super::spinner::Spinner;
use super::theme::{
    UiText, UiTheme, apply_style,
    locale::{self, DirectionalStyled as _},
    primitives::{clipped_image, surface},
};

/// GIFs per grid row.
pub const COLUMNS: usize = 2;
/// How close to the end of the grid, in pixels, scrolling asks for more.
const LOAD_AHEAD: f32 = 240.;

/// One GIF: an id the picker reports, a title and a preview image.
#[derive(Clone)]
pub struct Gif {
    /// What `GifPickerEvent::Pick` reports, such as the service's id.
    pub id: SharedString,
    /// The accessible name and tooltip, such as "Happy Dance GIF".
    pub title: SharedString,
    /// The preview; `None` draws a placeholder until the application has it.
    pub image: Option<ImageSource>,
    /// The preview's width, for laying out the grid before it loads.
    pub width: f32,
    /// The preview's height.
    pub height: f32,
}

impl Gif {
    /// A GIF of `width` by `height` (any unit; only the ratio counts).
    pub fn new(
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            image: None,
            width,
            height,
        }
    }

    /// Sets the preview image.
    pub fn image(mut self, image: impl Into<ImageSource>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// The preview's height for drawing it `width` wide; a GIF with no size
    /// is square, and a very tall or flat one is held to a sensible shape.
    fn height_at(&self, width: f32) -> f32 {
        let ratio = if self.width > 0. && self.height > 0. {
            self.height / self.width
        } else {
            1.
        };
        width * ratio.clamp(0.25, 2.5)
    }
}

/// Notifications from a GIF picker.
#[derive(Clone, Debug, PartialEq)]
pub enum GifPickerEvent {
    /// The search text changed: the application looks the text up (an
    /// empty one means "trending") and answers with `set_gifs`. It may wait
    /// for typing to settle first.
    Search(SharedString),
    /// The grid scrolled near its end while `set_gifs` or `extend_gifs`
    /// said more exist: the application answers with `extend_gifs`.
    LoadMore,
    /// A GIF was chosen with Enter or a click: its id.
    Pick(SharedString),
    /// Escape was pressed with an empty query.
    Cancel,
}

/// Where a GIF sits in the grid.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Place {
    column: usize,
    top: f32,
    height: f32,
}

/// Retained GIF picker state; subscribe to `GifPickerEvent`.
pub struct GifPickerState {
    input: Entity<InputState>,
    gifs: Vec<Gif>,
    more: bool,
    loading: bool,
    /// More was asked for and has not come yet.
    asked: bool,
    message: Option<SharedString>,
    attribution: Option<SharedString>,
    highlighted: Option<usize>,
    scroll: ScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<GifPickerEvent> for GifPickerState {}

impl Focusable for GifPickerState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.read(cx).focus_handle(cx)
    }
}

impl GifPickerState {
    /// Creates an empty picker; it shows a spinner until `set_gifs`.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = locale::text(cx, UiText::SearchGifs);
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let window_handle = window.window_handle();
        let weak = cx.weak_entity();
        let subscriptions = vec![
            cx.subscribe(&input, |this, input, event, cx| {
                if let InputEvent::Change = event {
                    let query = input.read(cx).value();
                    this.loading = true;
                    cx.emit(GifPickerEvent::Search(query));
                    cx.notify();
                }
            }),
            // Arrow keys move through the grid while the search field has
            // focus, ahead of the field's own caret bindings.
            cx.intercept_keystrokes(move |event, window, cx| {
                if window.window_handle() != window_handle {
                    return;
                }
                let Some(state) = weak.upgrade() else {
                    return;
                };
                if !state.read(cx).focus_handle(cx).is_focused(window) {
                    return;
                }
                if state.update(cx, |state, cx| {
                    state.handle_key(&event.keystroke, window, cx)
                }) {
                    cx.stop_propagation();
                }
            }),
        ];
        Self {
            input,
            gifs: Vec::new(),
            more: false,
            loading: true,
            asked: false,
            message: None,
            attribution: None,
            highlighted: None,
            scroll: ScrollHandle::new(),
            _subscriptions: subscriptions,
        }
    }

    /// Replaces the grid with the answer to a search, scrolled to the top;
    /// `more` says whether scrolling may load more.
    pub fn set_gifs(
        &mut self,
        gifs: impl IntoIterator<Item = Gif>,
        more: bool,
        cx: &mut Context<Self>,
    ) {
        self.gifs = gifs.into_iter().collect();
        self.more = more;
        self.loading = false;
        self.asked = false;
        self.message = None;
        self.highlighted = None;
        self.scroll.set_offset(point(px(0.), px(0.)));
        cx.notify();
    }

    /// Adds the answer to `LoadMore` after the GIFs already shown.
    pub fn extend_gifs(
        &mut self,
        gifs: impl IntoIterator<Item = Gif>,
        more: bool,
        cx: &mut Context<Self>,
    ) {
        let known: std::collections::HashSet<SharedString> =
            self.gifs.iter().map(|gif| gif.id.clone()).collect();
        self.gifs
            .extend(gifs.into_iter().filter(|gif| !known.contains(&gif.id)));
        self.more = more;
        self.loading = false;
        self.asked = false;
        cx.notify();
    }

    /// Updates the preview images of GIFs already shown, keeping the grid
    /// and its scroll position, for example as the images load.
    pub fn set_images(
        &mut self,
        mut image: impl FnMut(&Gif) -> Option<ImageSource>,
        cx: &mut Context<Self>,
    ) {
        for gif in &mut self.gifs {
            gif.image = image(gif);
        }
        cx.notify();
    }

    /// The GIFs shown, in the order the application gave them.
    pub fn gifs(&self) -> &[Gif] {
        &self.gifs
    }

    /// Shows a spinner in place of the grid's end, or of an empty grid.
    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        self.loading = loading;
        cx.notify();
    }

    /// Shows a message, such as an error, in place of the grid; `None`
    /// shows the grid again.
    pub fn set_message(&mut self, message: Option<SharedString>, cx: &mut Context<Self>) {
        self.message = message;
        self.loading = false;
        self.asked = false;
        cx.notify();
    }

    /// Credits the GIF service beside the search field, such as
    /// "Powered by GIPHY".
    pub fn set_attribution(&mut self, text: Option<SharedString>, cx: &mut Context<Self>) {
        self.attribution = text;
        cx.notify();
    }

    /// Sets the search field's placeholder, such as "Search GIPHY".
    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let placeholder = placeholder.into();
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder, window, cx)
        });
    }

    /// The search text.
    pub fn query(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    /// Replaces the search text and emits `GifPickerEvent::Search` when it
    /// changed.
    pub fn set_query(
        &mut self,
        query: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let query = query.into();
        if self.query(cx) == query {
            return;
        }
        self.input
            .update(cx, |input, cx| input.set_value(query.clone(), window, cx));
        // `set_value` may not report a change, so search here; a second
        // search for the same text is harmless.
        self.loading = true;
        cx.emit(GifPickerEvent::Search(query));
        cx.notify();
    }

    /// The highlighted GIF, which Enter picks.
    pub fn highlighted(&self) -> Option<&Gif> {
        self.gifs.get(self.highlighted?)
    }

    /// Call each time the picker is shown: focuses the search field.
    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focus(window, cx);
    }

    /// Moves keyboard focus to the search field.
    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        self.input.read(cx).focus_handle(cx).focus(window, cx);
    }

    fn handle_key(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let modifiers = keystroke.modifiers;
        if modifiers.control || modifiers.alt || modifiers.platform || modifiers.shift {
            return false;
        }
        let direction = UiTheme::read(cx).direction;
        let column_width = column_width(UiTheme::read(cx));
        let places = masonry(&self.gifs, column_width, gap(UiTheme::read(cx)));
        let target = match direction.logical_key(keystroke.key.as_str()) {
            "up" => step(&places, self.highlighted, Step::Up),
            "down" => step(&places, self.highlighted, Step::Down),
            "left" => step(&places, self.highlighted, Step::Across(0)),
            "right" => step(&places, self.highlighted, Step::Across(1)),
            "enter" => {
                if let Some(id) = self.highlighted().map(|gif| gif.id.clone()) {
                    cx.emit(GifPickerEvent::Pick(id));
                }
                return true;
            }
            "escape" => {
                if self.query(cx).is_empty() {
                    cx.emit(GifPickerEvent::Cancel);
                    // Let a hosting popover close on the same Escape.
                    return false;
                }
                self.set_query("", window, cx);
                return true;
            }
            _ => return false,
        };
        if let Some(target) = target {
            self.highlight(target, &places, cx);
        }
        true
    }

    fn highlight(&mut self, index: usize, places: &[Place], cx: &mut Context<Self>) {
        self.highlighted = Some(index);
        if let Some(place) = places.get(index) {
            let theme = UiTheme::read(cx);
            let viewport = f32::from(body_height(theme));
            let pad = f32::from(gap(theme));
            let offset = -f32::from(self.scroll.offset().y);
            let top = place.top + pad;
            let bottom = top + place.height + pad;
            let y = if top < offset + pad {
                Some(top - pad)
            } else if bottom > offset + viewport {
                Some(bottom - viewport)
            } else {
                None
            };
            if let Some(y) = y {
                self.scroll.set_offset(point(px(0.), px(-y.max(0.))));
            }
        }
        cx.notify();
    }

    /// Asks for more once the grid is scrolled near its end, or is too short
    /// to scroll at all.
    fn ask_for_more(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.more || self.loading || self.asked || self.message.is_some() {
            return;
        }
        if self.gifs.is_empty() {
            return;
        }
        let scrolled = -f32::from(self.scroll.offset().y);
        let max = f32::from(self.scroll.max_offset().y);
        if max - scrolled > LOAD_AHEAD {
            return;
        }
        self.asked = true;
        cx.defer_in(window, |this, _, cx| {
            this.loading = true;
            cx.emit(GifPickerEvent::LoadMore);
            cx.notify();
        });
    }
}

/// A direction for the arrow keys.
#[derive(Clone, Copy)]
enum Step {
    Up,
    Down,
    /// To the given column, at the nearest height.
    Across(usize),
}

/// Where the arrow keys go from `from` in the grid `places`.
fn step(places: &[Place], from: Option<usize>, step: Step) -> Option<usize> {
    let Some(from) = from.filter(|from| *from < places.len()) else {
        return (!places.is_empty()).then_some(0);
    };
    let here = places[from];
    let in_column = |column: usize| {
        places
            .iter()
            .enumerate()
            .filter(move |(_, place)| place.column == column)
    };
    match step {
        Step::Up => in_column(here.column)
            .filter(|(_, place)| place.top < here.top)
            .max_by(|(_, a), (_, b)| a.top.total_cmp(&b.top))
            .map(|(index, _)| index),
        Step::Down => in_column(here.column)
            .filter(|(_, place)| place.top > here.top)
            .min_by(|(_, a), (_, b)| a.top.total_cmp(&b.top))
            .map(|(index, _)| index),
        Step::Across(column) if column == here.column => None,
        Step::Across(column) => {
            let middle = here.top + here.height / 2.;
            in_column(column)
                .min_by(|(_, a), (_, b)| {
                    let distance = |place: &Place| (place.top + place.height / 2. - middle).abs();
                    distance(a).total_cmp(&distance(b))
                })
                .map(|(index, _)| index)
        }
    }
}

/// Lays `gifs` out in `COLUMNS` columns `width` wide, each GIF going to the
/// shortest column so far, as a masonry grid does.
fn masonry(gifs: &[Gif], width: Pixels, gap: Pixels) -> Vec<Place> {
    let width = f32::from(width);
    let gap = f32::from(gap);
    let mut bottoms = [0f32; COLUMNS];
    gifs.iter()
        .map(|gif| {
            let column = (0..COLUMNS)
                .min_by(|a, b| bottoms[*a].total_cmp(&bottoms[*b]))
                .unwrap_or(0);
            let height = gif.height_at(width);
            let top = bottoms[column];
            bottoms[column] = top + height + gap;
            Place {
                column,
                top,
                height,
            }
        })
        .collect()
}

/// The space between GIFs, and around the grid.
fn gap(theme: &UiTheme) -> Pixels {
    theme.space(2.)
}

/// The picker's width: the emoji picker's, so the two can share a popover.
fn picker_width(theme: &UiTheme) -> Pixels {
    theme.space(76.)
}

/// The grid's height under the search field, which makes the picker as tall
/// as the emoji picker.
fn body_height(theme: &UiTheme) -> Pixels {
    theme.space(85.) + px(2.)
}

fn column_width(theme: &UiTheme) -> Pixels {
    let gap = gap(theme);
    (picker_width(theme) - gap * (COLUMNS as f32 + 1.)) / COLUMNS as f32
}

impl Render for GifPickerState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ask_for_more(window, cx);
        let theme = UiTheme::read(cx).clone();
        let colors = theme.colors;
        let id = ElementId::from(("gif-picker", cx.entity_id()));
        let dir = theme.direction;
        let gap = gap(&theme);
        let column_width = column_width(&theme);
        let places = masonry(&self.gifs, column_width, gap);
        let grid_height = places
            .iter()
            .map(|place| place.top + place.height)
            .fold(0f32, f32::max);
        let highlighted = self.highlighted;
        let cells = self
            .gifs
            .iter()
            .zip(&places)
            .enumerate()
            .map(|(index, (gif, place))| {
                let selected = highlighted == Some(index);
                let pick = gif.id.clone();
                let left = (column_width + gap) * place.column as f32;
                let cell = match &gif.image {
                    Some(image) => {
                        clipped_image(image.clone(), theme.radius.md, ObjectFit::Cover, &theme)
                    }
                    None => div().rounded(theme.radius.md).bg(colors.muted),
                };
                cell.id((id.clone(), SharedString::from(format!("gif-{index}"))))
                    .test_support()
                    .role(Role::ListBoxOption)
                    .aria_label(gif.title.clone())
                    .aria_selected(selected)
                    .when(selected, |cell| cell.aria_active_descendant())
                    .absolute()
                    .top(px(place.top))
                    .map(|cell| {
                        if dir.is_rtl() {
                            cell.right(left)
                        } else {
                            cell.left(left)
                        }
                    })
                    .w(column_width)
                    .h(px(place.height))
                    .border_2()
                    .border_color(if selected {
                        gpui_kit::Hsla::from(colors.ring)
                    } else {
                        gpui_kit::transparent_black()
                    })
                    .cursor_pointer()
                    .on_hover(cx.listener(move |this, hovered, _, cx| {
                        if *hovered && this.highlighted != Some(index) {
                            this.highlighted = Some(index);
                            cx.notify();
                        }
                    }))
                    .on_click(cx.listener(move |_, _, _, cx| {
                        cx.emit(GifPickerEvent::Pick(pick.clone()));
                    }))
            })
            .collect::<Vec<_>>();
        let centered = |child: gpui_kit::AnyElement| {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .px(theme.space(4.))
                .text_center()
                .text_color(colors.muted_foreground)
                .child(child)
        };
        let body = if let Some(message) = self.message.clone() {
            centered(message.into_any_element()).into_any_element()
        } else if self.gifs.is_empty() && self.loading {
            centered(
                Spinner::new((id.clone(), "loading"))
                    .label(locale::text(cx, UiText::Loading))
                    .into_any_element(),
            )
            .into_any_element()
        } else if self.gifs.is_empty() {
            centered(locale::text(cx, UiText::NoGifs).into_any_element()).into_any_element()
        } else {
            div()
                .id((id.clone(), "list"))
                .role(Role::ListBox)
                .aria_label(locale::text(cx, UiText::Gifs))
                .size_full()
                .overflow_y_scroll()
                .track_scroll(&self.scroll)
                .p(gap)
                .child(div().relative().w_full().h(px(grid_height)).children(cells))
                .when(self.loading, |list| {
                    list.child(
                        div()
                            .w_full()
                            .py(theme.space(2.))
                            .flex()
                            .justify_center()
                            .child(Spinner::new((id.clone(), "more"))),
                    )
                })
                .into_any_element()
        };
        div()
            .id(id.clone())
            .key_context("GifPicker")
            .w(picker_width(&theme))
            .flex()
            .flex_col()
            .overflow_hidden()
            .text_color(colors.popover_foreground)
            .font_family(theme.fonts.body.clone())
            .text_size(theme.text(14.))
            .child(
                div()
                    .flex_row_in(dir)
                    .items_center()
                    .gap(theme.space(2.))
                    .h(theme.space(9.))
                    .px(theme.space(3.))
                    .border_b_1()
                    .border_color(colors.border)
                    .child(
                        lucide(LucideIcon::Search)
                            .size(theme.space(4.))
                            .flex_shrink_0()
                            .text_color(colors.muted_foreground),
                    )
                    .child(
                        Input::new(&self.input)
                            .aria_label(locale::text(cx, UiText::SearchGifs))
                            .bordered(false)
                            .px_0(),
                    )
                    .when_some(self.attribution.clone(), |row, attribution| {
                        row.child(
                            div()
                                .flex_shrink_0()
                                .whitespace_nowrap()
                                .text_size(theme.text(11.))
                                .text_color(colors.muted_foreground)
                                .child(attribution),
                        )
                    }),
            )
            .child(div().h(body_height(&theme)).w_full().child(body))
    }
}

/// A GIF picker bound to retained `GifPickerState`.
///
/// Put it in a popover anchored to a composer button, or beside an
/// `EmojiPicker` under tabs, and close the popover on
/// `GifPickerEvent::Pick` or `Cancel`.
#[derive(IntoElement)]
pub struct GifPicker {
    state: Entity<GifPickerState>,
    bordered: bool,
    style: StyleRefinement,
}

impl GifPicker {
    /// Creates a picker for retained state.
    pub fn new(state: &Entity<GifPickerState>) -> Self {
        Self {
            state: state.clone(),
            bordered: true,
            style: StyleRefinement::default(),
        }
    }
    /// Draws the rounded border, on by default. Turn it off inside a popover surface.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }
}

impl Styled for GifPicker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for GifPicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx);
        let root = div().flex_none().overflow_hidden().rounded(theme.radius.xl);
        let root = if self.bordered {
            surface::popover(root, theme).rounded(theme.radius.xl)
        } else {
            root
        }
        .child(self.state);
        apply_style(root, &self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::{Gif, Place, Step, masonry, step};
    use gpui_kit::px;

    fn gif(id: &str, width: f32, height: f32) -> Gif {
        Gif::new(id.to_string(), id.to_string(), width, height)
    }

    #[test]
    fn each_gif_goes_to_the_shortest_column() {
        let gifs = [
            gif("tall", 100., 200.),
            gif("square", 100., 100.),
            gif("wide", 200., 100.),
            gif("next", 100., 100.),
        ];
        let places = masonry(&gifs, px(100.), px(10.));
        assert_eq!(
            places,
            vec![
                Place {
                    column: 0,
                    top: 0.,
                    height: 200.
                },
                Place {
                    column: 1,
                    top: 0.,
                    height: 100.
                },
                Place {
                    column: 1,
                    top: 110.,
                    height: 50.
                },
                // Column 0 ends at 210, column 1 at 170.
                Place {
                    column: 1,
                    top: 170.,
                    height: 100.
                },
            ]
        );
    }

    #[test]
    fn a_gif_with_no_size_is_square_and_extremes_are_held() {
        assert_eq!(gif("none", 0., 0.).height_at(100.), 100.);
        assert_eq!(gif("strip", 1000., 10.).height_at(100.), 25.);
        assert_eq!(gif("pole", 10., 1000.).height_at(100.), 250.);
    }

    #[test]
    fn arrows_move_within_and_across_columns() {
        let gifs = [
            gif("a", 100., 200.),
            gif("b", 100., 100.),
            gif("c", 100., 100.),
            gif("d", 100., 100.),
        ];
        let places = masonry(&gifs, px(100.), px(10.));
        // Nothing highlighted: any arrow starts at the first GIF.
        assert_eq!(step(&places, None, Step::Down), Some(0));
        assert_eq!(step(&places, Some(1), Step::Down), Some(2));
        assert_eq!(step(&places, Some(2), Step::Up), Some(1));
        assert_eq!(step(&places, Some(1), Step::Up), None);
        // "d" sits under "a" in column 0.
        assert_eq!(step(&places, Some(0), Step::Down), Some(3));
        assert_eq!(step(&places, Some(2), Step::Across(0)), Some(0));
        assert_eq!(step(&places, Some(0), Step::Across(1)), Some(1));
        assert_eq!(step(&places, Some(0), Step::Across(0)), None);
    }
}

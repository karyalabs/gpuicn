//! Diff: a virtualized patch viewer, unified or side by side, with syntax
//! highlighting, line selection and caller-defined actions on the selection.
//!
//! GPUI Kit's `diff` supplies parsing, virtualization, both layouts, syntax
//! and inline-change highlighting, text copy and keyboard scrolling, and reads
//! the gpuicn theme through the synced Kit theme. gpuicn adds Nova defaults,
//! a read-only mode and an action strip under the selected lines whose
//! buttons report [`DiffIntent`]s with the selected range. Git execution and
//! patch building stay in the application.

use std::rc::Rc;

use gpui_icons::{LucideIcon, lucide};
use gpui_kit::{
    App, ElementId, Entity, IntoElement, ParentElement as _, Refineable as _, RenderOnce,
    SharedString, Styled, Window, div, prelude::FluentBuilder as _,
};

pub use gpui_kit::component::diff::{
    Diff as KitDiff, DiffAnnotation, DiffChangeIndicator, DiffDecoration, DiffEvent, DiffFile,
    DiffFileStatus, DiffGutterMarker, DiffHoverHighlight, DiffHunkSeparator, DiffLinePosition,
    DiffLineRange, DiffMode, DiffParseError, DiffParseOptions, DiffSide, DiffState,
};

use super::button::{Button, ButtonSize, ButtonVariant};
use super::theme::UiTheme;

/// An action offered on a selection of lines or on a whole file, such as
/// "Stage lines" or "Discard file".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffAction {
    /// Stable ID reported back in [`DiffIntent`].
    pub id: SharedString,
    /// The button text.
    pub label: SharedString,
    /// An optional leading icon.
    pub icon: Option<LucideIcon>,
    /// Drawn as a destructive button; confirm before acting.
    pub destructive: bool,
}

impl DiffAction {
    /// An action with a stable ID and a visible label.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            destructive: false,
        }
    }

    /// A leading Lucide icon.
    pub fn icon(mut self, icon: LucideIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Marks the action as destructive.
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }
}

/// What the person asked to do, with the lines or file it applies to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffIntent {
    /// The [`DiffAction::id`].
    pub action: SharedString,
    /// The file's path.
    pub path: SharedString,
    /// The selected lines, or `None` for a file action.
    pub range: Option<DiffLineRange>,
}

type IntentHandler = Rc<dyn Fn(&DiffIntent, &mut Window, &mut App)>;

/// Renders a [`DiffState`] with Nova defaults. Without actions, or with
/// [`DiffView::read_only`], it is a viewer only: selection and copy still
/// work, and no intent is ever reported.
#[derive(IntoElement)]
pub struct DiffView {
    id: ElementId,
    state: Entity<DiffState>,
    read_only: bool,
    selection_actions: Vec<DiffAction>,
    file_actions: Vec<DiffAction>,
    on_intent: Option<IntentHandler>,
    soft_wrap: bool,
    style: gpui_kit::StyleRefinement,
}

impl DiffView {
    /// A view of `state`; keep the state in your view and replace its files
    /// with `DiffState::set_files` when the patch changes.
    pub fn new(id: impl Into<ElementId>, state: &Entity<DiffState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            read_only: false,
            selection_actions: Vec::new(),
            file_actions: Vec::new(),
            on_intent: None,
            soft_wrap: false,
            style: Default::default(),
        }
    }

    /// Hides every action, for commit and review diffs.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Actions shown under the selected lines once a selection ends.
    pub fn selection_actions(mut self, actions: impl IntoIterator<Item = DiffAction>) -> Self {
        self.selection_actions = actions.into_iter().collect();
        self
    }

    /// Actions shown in every file header.
    pub fn file_actions(mut self, actions: impl IntoIterator<Item = DiffAction>) -> Self {
        self.file_actions = actions.into_iter().collect();
        self
    }

    /// Receives each intent. Selection is cleared after a selection action.
    pub fn on_intent(
        mut self,
        handler: impl Fn(&DiffIntent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_intent = Some(Rc::new(handler));
        self
    }

    /// Wraps long lines instead of scrolling horizontally.
    pub fn soft_wrap(mut self, soft_wrap: bool) -> Self {
        self.soft_wrap = soft_wrap;
        self
    }
}

impl Styled for DiffView {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        &mut self.style
    }
}

/// Where the selection's action strip goes: under the last selected line.
/// `None` in read-only mode, without actions, or without a selection.
pub fn selection_anchor(
    state: &DiffState,
    read_only: bool,
    has_actions: bool,
) -> Option<(DiffLineRange, DiffLinePosition)> {
    if read_only || !has_actions {
        return None;
    }
    let range = state.selected_lines()?;
    let end = DiffLinePosition::new(range.path().clone(), range.end_side(), range.end());
    Some((range, end))
}

fn action_button(
    id: ElementId,
    action: &DiffAction,
    theme: &UiTheme,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Button {
    Button::new(id)
        .size(ButtonSize::Xs)
        .variant(if action.destructive {
            ButtonVariant::Destructive
        } else {
            ButtonVariant::Outline
        })
        .aria_label(action.label.clone())
        .when_some(action.icon, |button, icon| {
            button.child(lucide(icon).size(theme.spacing.unit * 3.5_f32).text_color(
                if action.destructive {
                    theme.colors.destructive_foreground
                } else {
                    theme.colors.foreground
                },
            ))
        })
        .label(action.label.clone())
        .on_click(move |_, window, cx| on_click(window, cx))
}

impl RenderOnce for DiffView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = UiTheme::read(cx).clone();
        let anchor = selection_anchor(
            self.state.read(cx),
            self.read_only,
            !self.selection_actions.is_empty(),
        );
        let mut diff = KitDiff::new(&self.state)
            .hover_highlight(DiffHoverHighlight::LineNumber)
            .change_indicator(DiffChangeIndicator::Bars)
            .soft_wrap(self.soft_wrap)
            .font_family(theme.fonts.mono.clone());

        if let Some((range, end)) = anchor {
            let actions = Rc::new(self.selection_actions);
            let handler = self.on_intent.clone();
            let state = self.state.clone();
            let strip_theme = theme.clone();
            diff = diff
                .annotations([DiffAnnotation::line(
                    ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "selection".into()),
                    end,
                )])
                .render_annotation(move |_, _, _| {
                    let mut strip = div()
                        .flex()
                        .flex_wrap()
                        .gap(strip_theme.spacing.unit * 2_f32)
                        .px(strip_theme.spacing.unit * 3_f32)
                        .py(strip_theme.spacing.unit * 2_f32)
                        .bg(strip_theme.colors.muted)
                        .border_y_1()
                        .border_color(strip_theme.colors.border)
                        .font_family(strip_theme.fonts.body.clone());
                    for action in actions.iter() {
                        let handler = handler.clone();
                        let state = state.clone();
                        let intent = DiffIntent {
                            action: action.id.clone(),
                            path: range.path().clone(),
                            range: Some(range.clone()),
                        };
                        strip = strip.child(action_button(
                            ElementId::Name(format!("diff.selection.{}", action.id).into()),
                            action,
                            &strip_theme,
                            move |window, cx| {
                                state.update(cx, |state, cx| state.set_selected_lines(None, cx));
                                if let Some(handler) = &handler {
                                    handler(&intent, window, cx);
                                }
                            },
                        ));
                    }
                    strip
                });
        }

        if !self.read_only && !self.file_actions.is_empty() {
            let actions = Rc::new(self.file_actions);
            let handler = self.on_intent;
            let header_theme = theme.clone();
            diff = diff.render_header_suffix(move |file, _, _| {
                let mut row = div().flex().gap(header_theme.spacing.unit);
                for action in actions.iter() {
                    let handler = handler.clone();
                    let intent = DiffIntent {
                        action: action.id.clone(),
                        path: file.path().clone(),
                        range: None,
                    };
                    row = row.child(action_button(
                        ElementId::Name(format!("diff.file.{}.{}", file.path(), action.id).into()),
                        action,
                        &header_theme,
                        move |window, cx| {
                            if let Some(handler) = &handler {
                                handler(&intent, window, cx);
                            }
                        },
                    ));
                }
                row
            });
        }

        let mut root = div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .overflow_hidden()
            .rounded(theme.radius.md)
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.background)
            .child(diff.size_full());
        root.style().refine(&self.style);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{AppContext as _, Context, Render, TestAppContext, VisualTestContext};

    const PATCH: &str = "diff --git a/src/lib.rs b/src/lib.rs
index 1111111..2222222 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,4 +1,5 @@
 fn main() {
-    println!(\"old\");
+    println!(\"new\");
+    println!(\"more\");
 }

";

    #[test]
    fn parses_a_git_patch() {
        let files = DiffFile::parse(PATCH).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path().as_ref(), "src/lib.rs");
        assert_eq!((files[0].additions(), files[0].deletions()), (2, 1));
    }

    struct Probe {
        state: Entity<DiffState>,
        read_only: bool,
    }

    impl Render for Probe {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size(gpui_kit::px(600.)).child(
                DiffView::new("probe", &self.state)
                    .read_only(self.read_only)
                    .selection_actions([
                        DiffAction::new("stage", "Stage lines").icon(LucideIcon::Plus)
                    ])
                    .file_actions([DiffAction::new("discard", "Discard").destructive(true)]),
            )
        }
    }

    #[gpui_kit::test]
    fn the_selection_strip_follows_the_selection_and_read_only_hides_it(cx: &mut TestAppContext) {
        cx.update(super::super::theme::init);
        let state = cx.new(|cx| DiffState::new(DiffFile::parse(PATCH).unwrap(), cx));
        state.read_with(cx, |state, _| {
            assert!(
                selection_anchor(state, false, true).is_none(),
                "nothing selected"
            );
        });
        let range = DiffLineRange::new("src/lib.rs", DiffSide::Modified, 2, 3);
        state.update(cx, |state, cx| {
            state.set_selected_lines(Some(range.clone()), cx)
        });
        state.read_with(cx, |state, _| {
            let (selected, end) = selection_anchor(state, false, true).unwrap();
            assert_eq!(selected, range);
            assert_eq!(end.line(), 3);
            assert!(selection_anchor(state, true, true).is_none(), "read-only");
            assert!(
                selection_anchor(state, false, false).is_none(),
                "no actions"
            );
        });

        for read_only in [false, true] {
            let probe_state = state.clone();
            let window = cx.add_window(move |_, _| Probe {
                state: probe_state,
                read_only,
            });
            let mut cx = VisualTestContext::from_window(window.into(), cx);
            for mode in [
                super::super::theme::ThemeMode::Light,
                super::super::theme::ThemeMode::Dark,
            ] {
                cx.update(|_, cx| UiTheme::switch(cx, mode));
                cx.update(|window, cx| window.draw(cx).clear(cx));
            }
        }
    }
}

//! The diff catalog preview: a two-file patch, unified or side by side, with
//! staging actions on selected lines, a stage button beside each hunk and
//! file actions in each header.

use crate::*;
use gpuicn::diff::{DiffAction, DiffFile, DiffLineRange, DiffMode, DiffSide, DiffState, DiffView};

const PATCH: &str = r#"diff --git a/src/repository.rs b/src/repository.rs
index 3b18e51..a7c2f04 100644
--- a/src/repository.rs
+++ b/src/repository.rs
@@ -12,10 +12,14 @@ impl Repository {
     pub fn open(git: &Git, path: &Path) -> Result<Self> {
         let repository = Self::discover(git, path)?;
-        let root = repository.work_dir.as_deref().unwrap();
-        if root == path {
+        let wanted = canonical(path);
+        let root = repository.work_dir.as_deref().unwrap_or(&repository.git_dir);
+        if canonical(root) == wanted {
             Ok(repository)
         } else {
             Err(Error::NotARepository(path.to_path_buf()))
         }
     }
+
+    /// Whether the repository has no work tree.
+    pub fn is_bare(&self) -> bool { self.work_dir.is_none() }
 }
diff --git a/.env b/.env
deleted file mode 100644
index 9c1f2ab..0000000
--- a/.env
+++ /dev/null
@@ -1,2 +0,0 @@
-DATABASE_URL=postgres://localhost/app
-API_TOKEN=do-not-commit
"#;

impl Showcase {
    pub(crate) fn diff_preview(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = UiTheme::read(cx).clone();
        let s = t.spacing.unit;
        let state = window
            .use_keyed_state("diff_preview.state", cx, |_, cx| {
                cx.new(|cx| DiffState::new(DiffFile::parse(PATCH).expect("sample patch parses"), cx))
            })
            .read(cx)
            .clone();
        let mode = window.use_keyed_state("diff_preview.mode", cx, |_, _| vec![SharedString::from("unified")]);
        let note = window.use_keyed_state("diff_preview.note", cx, |_, _| {
            SharedString::from("Drag across line numbers to select lines")
        });
        let mode_state = state.clone();
        let mode_value = mode.clone();
        let toggle = ToggleGroup::new("diff.mode")
            .aria_label("Layout")
            .segmented(true)
            .value(mode.read(cx).clone())
            .item(ToggleGroupItem::new("unified", "unified").aria_label("Unified").child("Unified"))
            .item(ToggleGroupItem::new("split", "split").aria_label("Side by side").child("Side by side"))
            .on_change(move |next, _, cx| {
                let Some(value) = next.first().cloned() else { return };
                mode_state.update(cx, |state, cx| {
                    state.set_mode(if value == "split" { DiffMode::Split } else { DiffMode::Unified }, cx)
                });
                mode_value.update(cx, |mode, cx| {
                    *mode = vec![value];
                    cx.notify();
                });
            });
        let note_events = note.clone();
        let view = DiffView::new("diff.view", &state)
            .selection_actions([
                DiffAction::new("stage", "Stage lines").icon(LucideIcon::Plus),
                DiffAction::new("discard", "Discard lines").icon(LucideIcon::Undo2).destructive(true),
            ])
            .file_actions([DiffAction::new("stage-file", "Stage file")])
            .hunk_action(
                DiffAction::new("stage-hunk", "Stage hunk").icon(LucideIcon::Plus),
                [
                    DiffLineRange::new("src/repository.rs", DiffSide::Original, 14, 24)
                        .with_end_side(DiffSide::Modified),
                    DiffLineRange::new(".env", DiffSide::Original, 1, 2),
                ],
            )
            .on_intent(move |intent, _, cx| {
                let what = match &intent.range {
                    Some(range) => format!("lines {}–{} of {}", range.start(), range.end(), intent.path),
                    None => intent.path.to_string(),
                };
                note_events.update(cx, |note, cx| {
                    *note = format!("{} · {what}", intent.action).into();
                    cx.notify();
                });
            });
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .gap(s * 2_f32)
            .p(s * 2_f32)
            .font_family(t.fonts.body.clone())
            .child(toggle)
            .child(div().flex_1().min_h_0().child(view))
            .child(
                div()
                    .text_size(px(12.) * t.text_scale)
                    .text_color(t.colors.muted_foreground)
                    .child(note.read(cx).clone()),
            )
    }
}

pub(crate) fn preview(view: &Showcase, window: &mut Window, cx: &mut Context<Showcase>) -> gpui_kit::AnyElement {
    view.diff_preview(window, cx).into_any_element()
}

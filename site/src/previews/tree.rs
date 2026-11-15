//! The tree catalog preview: a repository's files with change markers, a lazy
//! folder and multi-select.

use crate::*;
use gpuicn::tree::{Tree, TreeChildren, TreeEvent, TreeNode, TreeRowKind, TreeState};
use gpuicn::virtual_list::ListSelectionMode;

fn files() -> Vec<TreeNode> {
    vec![
        TreeNode::folder(
            "src",
            "src",
            [
                TreeNode::folder(
                    "src/components",
                    "components",
                    [
                        TreeNode::leaf("src/components/button.rs", "button.rs"),
                        TreeNode::leaf("src/components/tree.rs", "tree.rs"),
                        TreeNode::leaf("src/components/virtual_list.rs", "virtual_list.rs"),
                    ],
                ),
                TreeNode::leaf("src/lib.rs", "lib.rs"),
                TreeNode::leaf("src/main.rs", "main.rs"),
            ],
        ),
        TreeNode::lazy("vendor", "vendor"),
        TreeNode::folder(
            "docs",
            "docs",
            [TreeNode::leaf("docs/a-very-long-file-name-that-truncates-in-narrow-panes.md", "a-very-long-file-name-that-truncates-in-narrow-panes.md")],
        ),
        TreeNode::leaf(".env", ".env"),
        TreeNode::leaf("Cargo.toml", "Cargo.toml"),
        TreeNode::leaf("README.md", "README.md"),
    ]
}

fn marker(id: &str) -> Option<&'static str> {
    match id {
        "src/components/tree.rs" | ".env" => Some("A"),
        "src/lib.rs" | "Cargo.toml" => Some("M"),
        _ => None,
    }
}

impl Showcase {
    pub(crate) fn tree_preview(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = UiTheme::read(cx).clone();
        let s = t.spacing.unit;
        let state = window
            .use_keyed_state("tree_preview.state", cx, |_, _| {
                let state = TreeState::new(files(), ["src".into(), "src/components".into()])
                    .expect("unique IDs");
                state.set_selection_mode(ListSelectionMode::Multiple);
                state
            })
            .read(cx)
            .clone();
        let note = window.use_keyed_state("tree_preview.note", cx, |_, _| SharedString::default());
        let row_theme = t.clone();
        let loader = state.clone();
        let note_events = note.clone();
        let tree = Tree::new("files.tree", "Repository files", state.clone(), move |row, _, _| {
            let muted = row_theme.colors.muted_foreground;
            let icon = match &row.kind {
                TreeRowKind::Folder { expanded: true } => LucideIcon::FolderOpen,
                TreeRowKind::Folder { expanded: false } => LucideIcon::Folder,
                TreeRowKind::Loading => LucideIcon::LoaderCircle,
                TreeRowKind::Failed(_) => LucideIcon::TriangleAlert,
                TreeRowKind::Leaf => LucideIcon::File,
            };
            div()
                .w_full()
                .min_w_0()
                .flex()
                .items_center()
                .gap(s * 2_f32)
                .child(lucide(icon).size(s * 4_f32).flex_shrink_0().text_color(muted))
                .child(div().flex_1().min_w_0().truncate().child(row.label.clone()))
                .when_some(marker(&row.id), |el, marker| {
                    el.child(div().text_size(px(12.) * row_theme.text_scale).text_color(muted).child(marker))
                })
                .into_any_element()
        })
        .on_event(move |event, _, cx| {
            match event {
                // Children load synchronously here; an app would read them in
                // the background and call `set_children` when they arrive.
                TreeEvent::LoadChildren(id) => {
                    loader.set_children(
                        id,
                        TreeChildren::Loaded(
                            (1..=3)
                                .map(|i| TreeNode::leaf(format!("{id}/crate-{i}"), format!("crate-{i}")))
                                .collect(),
                        ),
                    );
                }
                TreeEvent::Activate(id) => {
                    note_events.update(cx, |note, _| *note = format!("Opened {id}").into());
                }
                _ => {}
            }
        });
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .font_family(t.fonts.body.clone())
            .child(div().flex_1().min_h_0().px(s * 2_f32).py(s).child(tree))
            .child(
                div()
                    .px(s * 4_f32)
                    .py(s * 3_f32)
                    .border_t_1()
                    .border_color(t.colors.border)
                    .text_size(px(12.) * t.text_scale)
                    .text_color(t.colors.muted_foreground)
                    .child(format!(
                        "{} selected · {}",
                        state.selected().len(),
                        if note.read(cx).is_empty() {
                            SharedString::from("Arrows move, Left and Right fold, type to jump")
                        } else {
                            note.read(cx).clone()
                        }
                    )),
            )
    }
}

pub(crate) fn preview(view: &Showcase, window: &mut Window, cx: &mut Context<Showcase>) -> gpui_kit::AnyElement {
    view.tree_preview(window, cx).into_any_element()
}

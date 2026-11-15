//! The commit graph catalog preview: a synthetic history of two thousand
//! commits with feature branches, merges, tags and a stash, laid out by a small
//! lane assigner the way an application would.

use crate::*;
use gpuicn::commit_graph::{
    CommitGraph, CommitGraphState, GraphEdge, GraphLabel, GraphLabelKind, GraphNodeKind,
    GraphRow,
};

struct Commit {
    id: usize,
    parents: Vec<usize>,
    summary: String,
}

const SUMMARIES: [&str; 8] = [
    "Fix scroll anchoring after refresh",
    "Add keyboard shortcuts to the toolbar",
    "Update dependencies",
    "Refactor the settings model",
    "Speed up status parsing",
    "Document the release process",
    "Handle detached HEAD in the sidebar",
    "Tidy error messages",
];

/// A deterministic history, newest first: a main line with feature branches
/// that fork, gain a few commits and merge back.
fn history(count: usize) -> Vec<Commit> {
    let mut seed = 0x2545_f491_u32;
    let mut next = move |modulo: u32| {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        (seed % modulo) as usize
    };
    let mut commits: Vec<Commit> = Vec::with_capacity(count);
    let push = |commits: &mut Vec<Commit>, parents: Vec<usize>, summary: String| {
        let id = commits.len();
        commits.push(Commit { id, parents, summary });
        id
    };
    let mut main = push(&mut commits, Vec::new(), "Initial commit".into());
    let mut open: Vec<(usize, usize)> = Vec::new(); // (tip, number) of unmerged branches
    let mut branch = 0;
    while commits.len() < count {
        match next(10) {
            0 | 1 if open.len() < 3 => {
                branch += 1;
                let tip = push(&mut commits, vec![main], format!("Start feature {branch}"));
                open.push((tip, branch));
            }
            2..=4 if !open.is_empty() => {
                let slot = next(open.len() as u32);
                let (tip, number) = open[slot];
                let summary = SUMMARIES[next(SUMMARIES.len() as u32)];
                open[slot].0 = push(&mut commits, vec![tip], format!("{summary} (feature {number})"));
            }
            5 if !open.is_empty() => {
                let (tip, number) = open.remove(next(open.len() as u32));
                main = push(&mut commits, vec![main, tip], format!("Merge feature {number}"));
            }
            _ => {
                let summary = SUMMARIES[next(SUMMARIES.len() as u32)];
                main = push(&mut commits, vec![main], summary.into());
            }
        }
    }
    commits.reverse();
    commits
}

/// Assigns lanes newest first: a commit takes the lane that expects it, its
/// first parent continues that lane, and other parents get their own.
fn layout(commits: &[Commit]) -> Vec<GraphRow> {
    let mut lanes: Vec<Option<(usize, usize)>> = Vec::new(); // (expected commit, color)
    let mut colors = 0;
    let mut rows = Vec::with_capacity(commits.len());
    for (index, commit) in commits.iter().enumerate() {
        let waiting: Vec<usize> = (0..lanes.len())
            .filter(|&l| lanes[l].is_some_and(|(id, _)| id == commit.id))
            .collect();
        let lane = match waiting.first() {
            Some(&lane) => lane,
            None => {
                colors += 1;
                let free = lanes.iter().position(Option::is_none).unwrap_or(lanes.len());
                if free == lanes.len() {
                    lanes.push(None);
                }
                lanes[free] = Some((commit.id, colors));
                free
            }
        };
        let color = lanes[lane].map_or(0, |(_, color)| color);
        let mut row = GraphRow::new(
            SharedString::from(format!("{:07x}", commit.id * 2_654_435 % 0xfff_ffff)),
            commit.summary.clone(),
            lane,
        )
        .color(color)
        .kind(if commit.parents.len() > 1 {
            GraphNodeKind::Merge
        } else {
            GraphNodeKind::Commit
        });
        for (l, slot) in lanes.iter().enumerate() {
            if let Some((id, c)) = *slot {
                if waiting.contains(&l) {
                    row = row.incoming(GraphEdge::new(l, lane, c));
                } else if id != commit.id {
                    row = row.incoming(GraphEdge::straight(l, c));
                }
            }
        }
        // Lanes that merged into this commit end here; this lane continues
        // with the first parent.
        for &l in &waiting {
            lanes[l] = None;
        }
        if index == 0 {
            row.incoming.clear();
        }
        // Lanes that pass by this commit continue straight down.
        let passing: Vec<usize> = (0..lanes.len()).filter(|&l| lanes[l].is_some()).collect();
        let mut from_node = Vec::new();
        for (n, &parent) in commit.parents.iter().enumerate() {
            let existing = lanes.iter().position(|s| s.is_some_and(|(id, _)| id == parent));
            let target = match existing {
                Some(l) => l,
                None if n == 0 => {
                    lanes[lane] = Some((parent, color));
                    lane
                }
                None => {
                    colors += 1;
                    let free = lanes.iter().position(Option::is_none).unwrap_or(lanes.len());
                    if free == lanes.len() {
                        lanes.push(None);
                    }
                    lanes[free] = Some((parent, colors));
                    free
                }
            };
            from_node.push(target);
        }
        for &l in &passing {
            if let Some((_, c)) = lanes[l] {
                row = row.outgoing(GraphEdge::straight(l, c));
            }
        }
        for &l in &from_node {
            if let Some((_, c)) = lanes[l] {
                row = row.outgoing(GraphEdge::new(lane, l, c));
            }
        }
        while lanes.last().is_some_and(Option::is_none) {
            lanes.pop();
        }
        rows.push(row);
    }
    rows
}

fn rows() -> Vec<GraphRow> {
    let mut rows = layout(&history(2_000));
    let (lane, color) = (rows[0].lane, rows[0].color);
    rows[0].labels.push(GraphLabel::new(GraphLabelKind::Head, "main"));
    rows[0].labels.push(GraphLabel::new(GraphLabelKind::Remote, "origin/main"));
    rows[0].incoming.push(GraphEdge::straight(lane, color));
    for (n, row) in rows.iter_mut().filter(|r| r.lane == 0).step_by(80).skip(1).enumerate() {
        row.labels.push(GraphLabel::new(GraphLabelKind::Tag, format!("v1.{}", 30 - n)));
    }
    if let Some(tip) = rows.iter_mut().find(|r| r.lane > 0) {
        tip.labels.push(GraphLabel::new(GraphLabelKind::Branch, "feature/graph"));
    }
    // Uncommitted changes sit above the checked-out commit.
    let working = GraphRow::new("working-tree", "Uncommitted changes", lane)
        .kind(GraphNodeKind::WorkingTree)
        .color(color)
        .outgoing(GraphEdge::straight(lane, color));
    rows.insert(0, working);
    rows
}

impl Showcase {
    pub(crate) fn commit_graph_preview(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let t = UiTheme::read(cx).clone();
        let s = t.spacing.unit;
        let state = window
            .use_keyed_state("commit_graph_preview.state", cx, |_, _| {
                CommitGraphState::new(rows()).expect("synthetic history has unique IDs")
            })
            .read(cx)
            .clone();
        let muted = t.colors.muted_foreground;
        let mono = t.fonts.mono.clone();
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .p(s * 2_f32)
            .font_family(t.fonts.body.clone())
            .child(CommitGraph::new(
                "commit_graph.preview",
                "History",
                &state,
                move |row, _, _, _| {
                    div()
                        .w_full()
                        .min_w_0()
                        .flex()
                        .items_center()
                        .gap(s * 3_f32)
                        .child(div().flex_1().min_w_0().truncate().child(row.label.clone()))
                        .child(
                            div()
                                .flex_shrink_0()
                                .font_family(mono.clone())
                                .text_color(muted)
                                .child(match &row.id {
                                    gpui_kit::ElementId::Name(name) => name.clone(),
                                    _ => SharedString::default(),
                                }),
                        )
                        .into_any_element()
                },
            ))
    }
}

pub(crate) fn preview(
    view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> gpui_kit::AnyElement {
    view.commit_graph_preview(window, cx).into_any_element()
}

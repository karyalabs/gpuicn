# Tree parity

A native desktop composition, not a shadcn port. Implementation:
`crates/ui/registry/tree/tree.rs`, built on gpuicn's
[Virtual List](virtual-list.md) and GPUI Kit 0.9.1.

Tree flattens the expanded part of a caller-owned hierarchy into a Virtual
List, so it inherits that list's stable identity, single and multiple
selection, Shift and Cmd/Ctrl gestures, focus, scrolling, right-click
selection and the rule that only visible rows render. On top it adds
indentation, a disclosure chevron, Left/Right folding, typeahead and lazily
loaded children. GPUI Kit's own `tree` keeps single selection only and has no
lazy children or typeahead, so gpuicn does not wrap it.

Keep `TreeState` in your view. Nodes are `TreeNode` values with unique string
IDs; `replace_nodes` swaps in new data and keeps expansion, selection, focus
and the scroll anchor for IDs that survive. Duplicate IDs return
`DuplicateTreeNodeId` and change nothing.

A folder made with `TreeNode::lazy` asks for its children the first time it
opens: the tree shows a "Loading…" row and emits `TreeEvent::LoadChildren`.
Load them however you like and call `set_children` with
`TreeChildren::Loaded`, or with `TreeChildren::Failed(message)` to show the
message in a row that retries (emits `LoadChildren` again) when activated.
The tree never loads anything itself.

Keyboard: Up and Down move through visible rows, Right expands a folder and
then moves into its first child, Left collapses a folder or moves to the
parent, typing a label's first letters jumps to the next match (the search
resets after 0.9 s), Space toggles selection, Enter activates. Activating a
folder also toggles it. Clicking the chevron toggles without changing
selection. `reveal` expands every ancestor of a node and scrolls it into view.

The row renderer draws icon, label and any trailing status; the tree supplies
indentation (four spacing units per level, `indent` overrides) and the
chevron. Rows have one fixed height, as in Virtual List.

## Limits

- Rows are exposed to accessibility as list box options with their labels,
  like Virtual List; tree roles, levels and expanded state are not reported
  yet.
- Variable-height rows are not supported (the Virtual List limit).
- Qualification is local on Linux and in unit tests; native macOS pointer,
  VoiceOver and 100,000-node measurements are still to be recorded.

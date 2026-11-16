# Commit graph parity

A native desktop composition, not a shadcn port. Implementation:
`crates/ui/registry/commit_graph/commit_graph.rs`, on gpuicn's `VirtualList`.

## What it does

- Rows are `VirtualList` rows, so selection (single or multiple), arrow,
  Home/End and Page keys, typeahead-free activation with Enter or
  double-click, focus rings, right-click selection for `ContextMenu`, and
  scroll anchoring on `replace_rows` all match the list.
- Each row paints only its own slice of the graph: incoming segments from
  the row's top edge to the node, the node, and outgoing segments to the
  bottom edge. Rows meet exactly at their edges, so the graph stays aligned
  through scrolling, resizing and refreshes, and only visible rows paint.
- Lane changes bend with a vertical cubic curve, so lanes leave and enter
  rows upright.
- Node kinds differ by shape: filled circle (commit), ring with a dot
  (merge), square (stash), dashed hollow circle (uncommitted changes). Ref
  labels carry an icon and a badge treatment per kind (HEAD, branch, remote,
  tag, stash), so no meaning depends on color alone.
- The lane palette is eight evenly spaced hues at the saturation and
  lightness of the theme's `info` color, so it follows the theme in light
  and dark. Palette indices wrap.
- The column is as wide as the widest row's lanes, up to `max_lanes`
  (12 by default). Lanes past the limit are drawn at the last column and the
  row gets a dotted clipping marker. Out-of-range lanes or colors are
  clamped and never panic.

## Left to the caller

Graph layout (which lane each commit and edge uses), loading history in
pages, the row text, and what activation or a context menu does. The catalog
preview includes a compact lane assigner over a synthetic 2,000-commit
history as a starting point.

## Limits

- Fixed row height, like `VirtualList`.
- Per-row accessibility is the list's: the accessible name is the row's
  `label`; the graph itself is decorative.
- Native macOS pointer and VoiceOver passes are still to be recorded.

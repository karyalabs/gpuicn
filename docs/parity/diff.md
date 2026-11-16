# Diff parity

A native desktop composition, not a shadcn port. Implementation:
`crates/ui/registry/diff/diff.rs`, wrapping GPUI Kit 0.9.1's `diff`.

GPUI Kit supplies the viewer: Git patch parsing (`DiffFile::parse`), one
retained `DiffState` for any number of files, virtualized rows, unified and
side-by-side layouts (`DiffState::set_mode`) with synchronized scrolling,
syntax highlighting, inline change highlighting, hunk headers, collapsed
unchanged regions, per-file collapse, line-number selection with the pointer
and Shift+Up/Down, text copy, keyboard scrolling, and conflict decorations.
It reads gpuicn's theme through the synced Kit theme, so colors, radius and
the mono font follow the app theme.

gpuicn's `DiffView` adds:

- Nova defaults: a bordered rounded surface, change bars instead of +/−
  signs, line-number hover, the theme's mono font.
- Selection actions. When lines are selected, an action strip appears under
  the last selected line with the caller's `DiffAction` buttons. A press
  clears the selection and reports a `DiffIntent` with the action ID, the
  file path and the selected `DiffLineRange` (side and 1-based lines).
- File actions in every file header, reported with `range: None`.
- `read_only(true)` for commit and review diffs: selection and copy still
  work, no action is shown and no intent is reported.

The application owns everything Git: producing the patch, mapping a range to
a hunk or lines, building and applying partial patches, and confirming
destructive actions. Every re-parse should go through `set_files`, which
keeps scroll position and collapse state where paths survive.

## Limits

- Kit's selection is one contiguous range per file side; multiple disjoint
  ranges are not supported.
- Kit renders rows itself, so per-row accessibility follows Kit's diff.
- Native macOS pointer, VoiceOver and very large patch measurements are
  still to be recorded.

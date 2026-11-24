# GIF Picker parity

shadcn/ui has no GIF picker. Implementation:
`crates/ui/registry/gif_picker/gif_picker.rs`, built on gpuicn's `Input` and
`Spinner`. It follows the GIF tab of Mattermost's emoji picker, which draws
GIPHY's React grid.

`GifPicker` draws a search row and, under it, a two-column masonry grid: each
GIF goes to the shorter column at its own aspect ratio (held between 1:4 and
5:2), with a muted placeholder until its preview arrives. It is as wide and as
tall as `EmojiPicker`, so the two can sit under tabs in one popover without
the popover changing size.

The application owns the GIF service, and gpuicn never fetches:

- `GifPickerEvent::Search` reports the search text as it changes (an empty
  text means trending); the application answers with `set_gifs`.
- When the grid scrolls within 240 pixels of its end, or is too short to
  scroll, and the last answer said more exist, the picker emits one
  `GifPickerEvent::LoadMore` and shows a spinner until `extend_gifs`.
  GIFs already shown are not added twice.
- `set_images` updates previews as they load without moving the grid.
- `set_attribution` credits the service beside the search field, and
  `set_message` shows an error in place of the grid.

Keyboard: the arrow keys move the highlight while the search field keeps
focus (Up and Down within a column, Left and Right to the nearest GIF in the
other column, swapped in right-to-left layouts), Enter picks, and Escape clears
the search or, when it is empty, emits `Cancel` and lets a hosting popover
close.

Verified: masonry placement, the aspect ratio limits and arrow movement in
unit tests; the catalog preview in light and dark themes.

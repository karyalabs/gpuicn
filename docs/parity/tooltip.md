# Tooltip parity

Visual reference: shadcn/ui 4.19.0, Neutral Nova. Implementation:
`crates/ui/registry/tooltip/tooltip.rs`, using GPUI Kit 0.6.4.

`tooltip` attaches a Nova surface to one Kit button and shows it on pointer hover or keyboard focus. Escape dismisses it. Kit supplies the delayed overlay and viewport-aware placement. `AnchoredTooltip::anchored_tooltip` puts a pointer-hover tooltip on any element, such as a list row or an emoji, without nesting a button: centered above the element with an arrow pointing at it, below it when there is no room above, kept 8px inside the window. Use it with `text_tooltip` instead of GPUI's native `.tooltip(...)`, which opens below and to the right of wherever the pointer is. A tooltip on an element inside another's wins while the pointer is over it; pressing or scrolling hides it.

The Rust source and Usage example define the supported API. Browser DOM/CSS behavior,
exact exit animations and cross-platform screen-reader parity are not implied by
the native component. See [migration qualification](../internal/validation/gpui-kit-migration.md)
for the checks completed on this revision.

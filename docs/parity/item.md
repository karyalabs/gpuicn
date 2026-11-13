# Item parity

Visual reference: shadcn/ui 4.19.0, Neutral Nova (`item.tsx`). Implementation:
`crates/ui/registry/item/item.rs`, using GPUI Kit 0.9.0.

`Item` is one row: optional media, a title with a muted description (clamped to
two lines) and actions at the trailing edge. `.stacked(true)` puts the actions
under the copy at full width, for wide controls such as a tag field.
`ItemGroup` lays items in a column with `ItemSeparator` hairlines between them;
its default `Outline` variant is one bordered card with `lg` corners, the way
settings pages group rows. Variants `Default`, `Outline` and `Muted` and sizes
`Default` (min 48px, 10px by 16px padding) and `Sm` follow shadcn.

Differences from shadcn: shadcn composes `ItemMedia`, `ItemContent`,
`ItemTitle`, `ItemDescription` and `ItemActions` as children; gpuicn takes them
as builder slots (`media`, `title`, `description`, `actions`) and puts any extra
children inside the copy column. A row with `.on_click` is a button whose
accessible name is its title; a row without one is a group named by its title,
so a control inside it is announced in context. The application still labels
each control.

The Rust source and Usage example define the supported API. Verified: the
control stays on the title's row at 320px wide, and stacked rows put it under
the copy (unit tests in `item.rs`).

`Reveal` has no shadcn counterpart (shadcn reaches for Tailwind's
`group-hover:opacity-100 group-focus-within:opacity-100`). It draws row actions
at zero opacity until the row marked `.group(name)` is hovered or keyboard focus
is on an action or inside the element whose focus handle is passed to
`Reveal::focus_within`. The actions stay in layout, in the tab order and in the
accessibility tree while hidden. A pointer click that leaves focus behind does
not keep them shown; hover covers the pointer. Verified: keyboard focus on an
action or on its row reveals it, pointer focus and focus elsewhere do not (unit
test in `item.rs`).

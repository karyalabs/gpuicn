/**
 * The theme menu's choices. A style is `<base>-<accent>-<radius in rem>`, such
 * as `zinc-blue-0.5`; the page keeps it in `data-style` on <html> and every
 * GPUI preview builds its theme from it (site/src/site_theme.rs).
 */

/** shadcn base colors, as `gpuicn theme set --base-color` accepts them. */
export const bases = [
  { id: "neutral", label: "Neutral", swatch: "#737373" },
  { id: "stone", label: "Stone", swatch: "#79716b" },
  { id: "zinc", label: "Zinc", swatch: "#71717b" },
  { id: "mauve", label: "Mauve", swatch: "#79697b" },
  { id: "olive", label: "Olive", swatch: "#6b7a5c" },
  { id: "mist", label: "Mist", swatch: "#5f7d86" },
  { id: "taupe", label: "Taupe", swatch: "#7c6d67" },
] as const

/** Keep the colors in step with `ACCENTS` in site/src/site_theme.rs. */
export const accents = [
  { id: "none", label: "Base", light: "#171717", dark: "#e5e5e5" },
  { id: "blue", label: "Blue", light: "#2563eb", dark: "#3b82f6" },
  { id: "violet", label: "Violet", light: "#7c3aed", dark: "#8b5cf6" },
  { id: "rose", label: "Rose", light: "#e11d48", dark: "#f43f5e" },
  { id: "orange", label: "Orange", light: "#ea580c", dark: "#f97316" },
  { id: "green", label: "Green", light: "#16a34a", dark: "#22c55e" },
] as const

/** Corner radius in rem; 0.625 is the default. */
export const radii = ["0", "0.3", "0.5", "0.625", "0.75", "1"] as const

export const defaultStyle = "neutral-none-0.625"

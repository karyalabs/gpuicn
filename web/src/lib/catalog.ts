import catalog from "../generated/catalog.json"
import { u } from "./base"
import { release } from "./release"

export type CatalogComponent = {
  slug: string
  name: string
  description: string
  /** The id of one of `categories`. */
  category: string
  /**
   * The release that first shipped the component, such as "0.7.0", or "next"
   * before its first release. Read from `since` in registry.toml.
   */
  added: string
  /** Height of the preview frame on the component's page, in pixels. */
  previewHeight: number
}

/**
 * A component shows a "New" badge while the current release is within this
 * many minor versions of the one that added it: at 0.11.x, everything added
 * in 0.9.0 or later. Nothing here is edited by hand. `cargo xtask
 * new-component` writes `since = "next"` to registry.toml and the release
 * bump stamps the version.
 */
export const newForMinorVersions = 3

const minorIndex = (version: string) => {
  const [major = 0, minor = 0] = version.replace(/^v/, "").split(".").map(Number)
  return { major, minor }
}

export function isNew(component: CatalogComponent) {
  if (component.added === "next") return true
  const added = minorIndex(component.added)
  const current = minorIndex(release)
  return added.major === current.major && current.minor - added.minor < newForMinorVersions
}

/** "Added in 0.9.6", or "Unreleased" before the first release that ships it. */
export function addedLabel(component: CatalogComponent) {
  return component.added === "next" ? "Unreleased" : `Added in ${component.added}`
}

/**
 * Every component with a page, A to Z. Generated from registry.toml by
 * scripts/sync-catalog.mjs; add a component there, not here.
 */
export const components: CatalogComponent[] = catalog.components

/** The groups of the catalog, in display order, each with its components. */
export const categories = catalog.categories.map((category) => ({
  ...category,
  components: components.filter((component) => component.category === category.id),
})).filter((category) => category.components.length > 0)

export const guide = [
  { to: u("/introduction/"), label: "Introduction", blurb: "What gpuicn is, and how installing source differs from adding a crate." },
  { to: u("/installation/"), label: "Installation", blurb: "Create an app, install the CLI, add a component, open a window." },
  { to: u("/theming/"), label: "Theming", blurb: "The tokens every component reads, and how to change them at runtime." },
  { to: u("/showcase/"), label: "Example app", blurb: "gpuicn Workspace for macOS and Linux, built from this catalog." },
]

export const reference = [
  { to: u("/components/"), label: "Components", blurb: `All ${components.length} components, each with a live preview, usage, API, and platform notes.` },
  { to: u("/icons/"), label: "Icons", blurb: "The complete Lucide library as Rust names, rendered by GPUI." },
  { to: u("/registry/"), label: "Registry", blurb: "The JSON files behind gpuicn add." },
  { to: u("/changelog/"), label: "Changelog", blurb: "What each release added, changed, and fixed." },
]

export function componentHref(slug: string) {
  return u(`/components/${slug}/`)
}

export function findComponent(slug: string) {
  return components.find((component) => component.slug === slug)
}

export function categoryOf(component: CatalogComponent) {
  return categories.find((category) => category.id === component.category)!
}

export function previewHeight(slug: string) {
  return findComponent(slug)?.previewHeight ?? 288
}

export function rustModule(slug: string) {
  return slug.replaceAll("-", "_")
}

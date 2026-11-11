export type ComponentDocs = {
  /** What the page's preview can show: the overview first, then the named examples. `id` is the preview's `?example=`. */
  examples: { id: string; title: string; blurb: string; code: string }[]
  usage: string
  usageCall: string
  installComponents: string[]
  source: string
  api: { signature: string; description: string }[]
  parity: string
  parityHtml: string
  modules: string[]
}

export type Setup = {
  dependencies: string
  config: string
  aliases: string
  starter: string
  theme: string
}

export type Icon = {
  name: string
  rust: string
  tags: string[]
  categories: string[]
  aliases: string[]
}

const documents = import.meta.glob<ComponentDocs>("../generated/components/*.json", {
  import: "default",
  eager: true,
})

export function componentDocs(slug: string): ComponentDocs {
  const docs = documents[`../generated/components/${slug}.json`]
  if (!docs) throw new Error(`Run bun run sync:static: missing generated docs for ${slug}`)
  return docs
}

export { default as setup } from "../generated/setup.json"
export { default as icons } from "../generated/icons.json"

export { default as changelog } from "../generated/changelog.json"

// The Worker in front of the site's static files (wrangler.jsonc).
//
// The site lives at https://karyalabs.dev/gpuicn/. Its old addresses,
// ui.imajha.com and ui.jha.sh, had the same pages at the root, so a request
// there is sent to the same page under /gpuicn: on the new address when
// REDIRECT_OLD_HOSTS is "true", and on the old host itself until then, which
// keeps the old addresses working while the new one is being set up.

const HOME = "https://karyalabs.dev"
const BASE = "/gpuicn"

export default {
  async fetch(request, env) {
    const url = new URL(request.url)
    if (url.hostname === "karyalabs.dev") return env.ASSETS.fetch(request)

    const inBase = url.pathname === BASE || url.pathname.startsWith(`${BASE}/`)
    const path = inBase ? url.pathname : `${BASE}${url.pathname}`
    if (env.REDIRECT_OLD_HOSTS === "true") {
      return Response.redirect(`${HOME}${path}${url.search}`, 301)
    }
    if (inBase) return env.ASSETS.fetch(request)
    // Temporary while the new address is checked, so not a permanent redirect.
    return Response.redirect(`${url.origin}${path}${url.search}`, 302)
  },
}

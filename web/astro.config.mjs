// @ts-check
import { defineConfig } from "astro/config"

// A static site: every page is plain HTML with inlined CSS, and the only
// client scripts are the small ones under src/scripts (theme, previews,
// search, copy buttons). Cloudflare serves web/dist as static assets; the
// site lives under /gpuicn on karyalabs.dev, so it builds into dist/gpuicn
// and links go through `u` (src/lib/base.ts).
export default defineConfig({
  site: "https://karyalabs.dev",
  base: "/gpuicn",
  outDir: "./dist/gpuicn",
  trailingSlash: "always",
  build: { inlineStylesheets: "always" },
  // Whitespace between Shiki's spans is code; keep it.
  compressHTML: false,
  devToolbar: { enabled: false },
})

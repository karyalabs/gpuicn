/**
 * The site is served under a path (`/gpuicn` on karyalabs.dev), set as
 * `base` in astro.config.mjs. Every root-relative link goes through `u`.
 */
export const base = import.meta.env.BASE_URL.replace(/\/$/, "")

/** `u("/components/")` is `/gpuicn/components/`. */
export const u = (path: string) => `${base}${path}`

// Shoots a small still of every catalog preview, light and dark, for the cards
// on the website's home page and component index. Each still is trimmed to
// what the component drew, so cards can center it.
//
//   bun scripts/thumbnails.mjs [--dist web/public/demo] [--out web/public/thumbs] [--only button,dialog]
//
// Run it after `trunk build` and before the Astro build; the site shows a
// plain card for any component without a still. Needs `playwright` on
// NODE_PATH and its Chromium (set CHROMIUM_PATH to use another binary).
// Headless Chromium has no WebGPU, so the previews draw with WebGL.

import { createServer } from "node:http";
import { readFileSync, existsSync, mkdirSync, writeFileSync, statSync } from "node:fs";
import { extname, join, resolve } from "node:path";
import { chromium } from "playwright";

const args = Object.fromEntries(
  process.argv.slice(2).map((arg, i, all) =>
    arg.startsWith("--") ? [arg.slice(2), all[i + 1]?.startsWith("--") || all[i + 1] === undefined ? "true" : all[i + 1]] : [],
  ).filter((pair) => pair.length),
);
const dist = resolve(args.dist ?? "web/public/demo");
const out = resolve(args.out ?? "web/public/thumbs");
const only = args.only ? new Set(args.only.split(",")) : null;
const wait = Number(args.wait ?? "1200");

if (!existsSync(join(dist, "index.html"))) {
  console.error(`No built previews at ${dist}; run trunk build first.`);
  process.exit(2);
}
mkdirSync(out, { recursive: true });

// The preview list and each frame's height come from registry.toml.
const manifest = Bun.TOML.parse(readFileSync(resolve("registry.toml"), "utf8"));
const wide = new Set(["sidebar", "virtual_list", "message_list"]);
const demos = Object.entries(manifest.components)
  .filter(([module]) => module !== "theme" && module !== "app")
  .map(([module, meta]) => ({
    slug: module.replaceAll("_", "-"),
    width: wide.has(module) ? 960 : 640,
    height: Math.min(wide.has(module) ? 600 : meta.preview_height ?? 288, 720),
  }))
  .filter((demo) => !only || only.has(demo.slug));

const types = { ".html": "text/html", ".js": "text/javascript", ".wasm": "application/wasm", ".css": "text/css", ".ttf": "font/ttf", ".svg": "image/svg+xml", ".png": "image/png", ".json": "application/json" };
const server = createServer((request, response) => {
  const url = new URL(request.url, "http://localhost");
  let path = join(dist, url.pathname.replace(/^\/gpuicn\/demo\//, "/"));
  if (!existsSync(path) || statSync(path).isDirectory()) path = join(dist, "index.html");
  response.writeHead(200, {
    "Content-Type": types[extname(path)] ?? "application/octet-stream",
    "Cross-Origin-Embedder-Policy": "require-corp",
    "Cross-Origin-Opener-Policy": "same-origin",
  });
  response.end(readFileSync(path));
});
await new Promise((ready) => server.listen(0, "127.0.0.1", ready));
const origin = `http://127.0.0.1:${server.address().port}`;

const browser = await chromium.launch({
  executablePath: process.env.CHROMIUM_PATH || undefined,
  args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--no-sandbox", "--disable-gpu-sandbox"],
});

// Crops a screenshot to the pixels that differ from its corner color, plus a
// margin. Decoded in the browser, so no PNG library is needed here.
async function trim(page, png) {
  return page.evaluate(async (src) => {
    const image = new Image();
    await new Promise((ok, err) => { image.onload = ok; image.onerror = err; image.src = src; });
    const canvas = document.createElement("canvas");
    canvas.width = image.width;
    canvas.height = image.height;
    const context = canvas.getContext("2d");
    context.drawImage(image, 0, 0);
    const { data } = context.getImageData(0, 0, image.width, image.height);
    let [left, top, right, bottom] = [image.width, image.height, -1, -1];
    for (let y = 0; y < image.height; y++) {
      for (let x = 0; x < image.width; x++) {
        const i = (y * image.width + x) * 4;
        if (Math.abs(data[i] - data[0]) + Math.abs(data[i + 1] - data[1]) + Math.abs(data[i + 2] - data[2]) > 6) {
          if (x < left) left = x;
          if (x > right) right = x;
          if (y < top) top = y;
          if (y > bottom) bottom = y;
        }
      }
    }
    if (right < 0) return src;
    const margin = 12;
    left = Math.max(0, left - margin);
    top = Math.max(0, top - margin);
    right = Math.min(image.width - 1, right + margin);
    bottom = Math.min(image.height - 1, bottom + margin);
    const cut = document.createElement("canvas");
    cut.width = right - left + 1;
    cut.height = bottom - top + 1;
    cut.getContext("2d").drawImage(canvas, left, top, cut.width, cut.height, 0, 0, cut.width, cut.height);
    return cut.toDataURL("image/png");
  }, `data:image/png;base64,${png.toString("base64")}`);
}

const jobs = demos.flatMap((demo) => ["light", "dark"].map((theme) => ({ ...demo, theme })));
const failed = [];
async function worker() {
  const tool = await browser.newPage();
  for (let job = jobs.shift(); job; job = jobs.shift()) {
    const { slug, theme, width, height } = job;
    const page = await browser.newPage({ viewport: { width, height }, deviceScaleFactor: 1 });
    try {
      await page.goto(`${origin}/gpuicn/demo/?demo=${slug}&theme=${theme}&width=${width}&height=${height}&backend=webgl`);
      await page.waitForSelector("html[data-gpuicn-ready]", { timeout: 60_000 });
      await page.waitForTimeout(wait);
      const still = await trim(tool, await page.screenshot());
      writeFileSync(join(out, `${slug}-${theme}.png`), Buffer.from(still.split(",")[1], "base64"));
    } catch (error) {
      failed.push(`${slug}-${theme}: ${String(error).split("\n")[0]}`);
    } finally {
      await page.close();
    }
  }
  await tool.close();
}
try {
  await Promise.all(Array.from({ length: Number(args.jobs ?? "4") }, worker));
} finally {
  await browser.close();
  server.close();
}
console.log(`Wrote ${demos.length * 2 - failed.length} thumbnails to ${out}`);
if (failed.length) {
  console.error(`Missing:\n${failed.join("\n")}`);
  process.exit(1);
}

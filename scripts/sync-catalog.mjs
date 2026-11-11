import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";
import { dirname } from "node:path";

const root = fileURLToPath(new URL("../", import.meta.url));
const read = (path) => readFileSync(`${root}${path}`, "utf8");
// One preview file per component; `cargo xtask new-component` writes it.
const previewSource = (module) => read(`site/src/previews/${module}.rs`);
const usageExamples = read("crates/ui/examples/usage.rs");
const registry = JSON.parse(read("registry.json"));
const componentByModule = new Map(registry.items.map((item) => [item.name.replaceAll("-", "_"), item]));
const moduleName = (file) => file.path.split("/").at(-1).replace(".rs", "");
const installedUsage = usageExamples.replace("use gpuicn as ui;", '#[path = "../ui/mod.rs"]\nmod ui;');
assert.notEqual(installedUsage, usageExamples, "Missing Usage crate-alias anchor");
mkdirSync(`${root}fixtures/registry-install/src/bin`, { recursive: true });
writeFileSync(`${root}fixtures/registry-install/src/bin/generated-usage.rs`, installedUsage);
const sidebarExamples = [
  ["workspace", "Workspace", "An inset layout with a team switcher, nested projects, a toggle rail, and an account menu."],
  ["docs", "Documentation", "A searchable documentation tree with independent sections and off-canvas navigation."],
  ["mail", "Mail", "A folder rail beside a searchable message list. Filter unread mail and open a message."],
  ["floating", "Floating right", "A rounded sidebar on the right with an icon-only collapsed state."],
  ["mobile", "Mobile sheet", "Modal navigation with its own open state, focus containment, and Escape to dismiss."],
  ["loading", "Loading", "Fixed navigation with stable placeholders until the menu loads."],
];
const exampleList = read("site/src/examples/mod.rs");
const namedExamples = (module) => {
  const path = `site/src/examples/${module}.rs`;
  if (!existsSync(`${root}${path}`)) return [];
  const found = [...read(path).matchAll(/((?:^\/\/\/.*\n)+)^pub\(crate\) (fn (\w+)\([\s\S]*?^\})/gm)].map(([, comment, code, name]) => {
    const [title, ...rest] = comment.split("\n").map((line) => line.replace(/^\/\/\/ ?/, ""));
    const blurb = rest.join(" ").trim();
    assert(title && blurb, `${path}: ${name} needs a title line, a blank line, and a sentence`);
    return { id: name.replaceAll("_", "-"), title, blurb, code };
  });
  // The showcase draws what `examples!` lists; the page shows what the file holds.
  const listed = exampleList.match(new RegExp(`^    ${module}: ([^;]+);`, "m"))?.[1].split(",").map((name) => name.trim().replaceAll("_", "-"));
  assert.deepEqual(found.map((example) => example.id), listed, `${path} and examples! in site/src/examples/mod.rs must list the same functions in the same order`);
  return found;
};
const destination = `${root}web/src/generated/components`;
mkdirSync(destination, { recursive: true });
// The website's catalog: registry.toml in its own order, grouped by category.
// web/src/lib/catalog.ts reads this file and names no component itself.
const manifest = Bun.TOML.parse(read("registry.toml"));
// These have no page: theme installs with every component, and app is the
// starter covered by the Installation page.
const withoutPage = new Set(["theme", "app"]);
const titleCase = (module) => module.split("_").map((word) => word[0].toUpperCase() + word.slice(1)).join(" ");
writeFileSync(`${root}web/src/generated/catalog.json`, JSON.stringify({
  categories: manifest.categories,
  components: Object.entries(manifest.components)
    .filter(([module]) => !withoutPage.has(module))
    .map(([module, meta]) => ({
      slug: module.replaceAll("_", "-"),
      name: meta.title ?? titleCase(module),
      description: meta.description,
      category: meta.category,
      added: meta.since,
      previewHeight: meta.preview_height ?? 288,
    }))
    .sort((a, b) => a.slug.localeCompare(b.slug)),
}, null, 2) + "\n");
writeFileSync(`${root}web/src/generated/setup.json`, JSON.stringify({
  dependencies: read("fixtures/registry-install/Cargo.toml").split("[dependencies]\n")[1].split("\n[workspace]")[0].trim(),
  config: read("fixtures/registry-install/components.json").trim(),
  aliases: read("fixtures/registry-install/tsconfig.json").trim(),
  starter: read("fixtures/registry-install/src/main.rs").replace("#![allow(dead_code)]\n\n", "").trim(),
  theme: read("crates/ui/registry/theme/theme.rs"),
}, null, 2));

// Read rustfmt's method boundaries, not Rust expressions. Cargo checks these
// same methods; the assertions below fail if their layout or coverage changes.
for (const item of registry.items.filter((item) => item.name !== "theme")) {
  const module = item.name.replaceAll("-", "_");
  const source = read(`crates/ui/registry/${module}/${module}.rs`);
  const showcase = previewSource(module);
  const preview = showcase.match(new RegExp(`^    pub\\(crate\\) fn ${module}_preview\\([\\s\\S]*?^    \\}`, "m"))?.[0]?.replace("pub(crate) fn", "fn");
  assert(preview, `Missing compiled preview for ${item.name}`);
  const usage = usageExamples.match(new RegExp(`^mod ${module} \\{\\n([\\s\\S]*?)^\\}`, "m"))?.[1].replace(/^    /gm, "").trim();
  assert(usage?.includes("fn example("), `Missing compile-checked usage for ${item.name}`);
  const usageModules = [...usage.matchAll(/^use crate::ui::([^;]+);$/gm)].map((match) => {
    const importedModule = match[1].match(/^([a-z][a-z0-9_]*)::/)?.[1];
    assert(importedModule, `${item.name} Usage must import from an installed module, not crate::ui root`);
    assert(componentByModule.has(importedModule), `${item.name} Usage imports unknown module ${importedModule}`);
    return importedModule;
  });
  const includedModules = new Set(item.files.map(moduleName));
  const extraComponents = [...new Set(usageModules.filter((name) => !includedModules.has(name)))]
    .map((name) => componentByModule.get(name).name)
    .filter((name, _, names) => !names.some((other) =>
      other !== name && componentByModule.get(other.replaceAll("-", "_")).files.some((file) => moduleName(file) === name.replaceAll("-", "_"))
    ));
  const installComponents = [item.name, ...extraComponents];
  const installFiles = installComponents.flatMap((name) => componentByModule.get(name.replaceAll("-", "_")).files);
  const installedModules = new Set(installFiles.map(moduleName));
  assert(usageModules.every((name) => installedModules.has(name)), `${item.name} install command does not cover its Usage imports`);

  const api = [...source.matchAll(/(?:^\s*\/\/\/[^\n]*\n)*(?:^\s*#\[[^\n]*\]\n)*^\s*pub (?:const )?fn [\s\S]*?\{/gm)]
    .map(([entry]) => {
      const description = [...entry.matchAll(/\/\/\/ ?([^\n]*)/g)].map(([, line]) => line).join(" ");
      const signature = entry.slice(entry.indexOf("pub "), -1).trim();
      return { signature, description };
    });
  assert(api.length > 0, `Missing public API for ${item.name}`);
  assert.equal(api.length, [...source.matchAll(/\bpub (?:const )?fn /g)].length, `Incomplete API for ${item.name}`);

  const parityName = readdirSync(`${root}docs/parity`).find((name) => name.replaceAll("_", "-") === `${item.name}.md`);
  assert(parityName, `Missing platform notes for ${item.name}`);
  const parity = read(`docs/parity/${parityName}`);
  const sidebarExample = (name) => {
    const method = showcase.match(new RegExp(`^    (?:pub\\(crate\\) )?fn sidebar_${name}_preview\\([\\s\\S]*?^    \\}`, "m"))?.[0]?.replace("pub(crate) fn", "fn");
    assert(method, `Missing Sidebar ${name} example`);
    return method.replace(/^    /gm, "");
  };
  // What a component page can show in its preview: the overview first, then
  // the functions in site/src/examples/<module>.rs. Each function's doc
  // comment is its title, a blank line, and a sentence about it. Sidebar's
  // are whole layouts drawn by its own preview.
  const examples = item.name === "sidebar"
    ? sidebarExamples.map(([id, title, blurb]) =>
      ({ id, title, blurb, code: sidebarExample(id === "docs" || id === "mail" ? id : "application") }))
    : [{ id: "", title: "Overview", blurb: "", code: preview.replace(/^    /gm, "") }, ...namedExamples(module)];

  const helperModules = new Set(installFiles.flatMap((file) =>
    [...read(`crates/ui/${file.path}`).matchAll(/#\[path = "([^"/]+)\.rs"\]/g)].map((match) => match[1])
  ));
  writeFileSync(`${destination}/${item.name}.json`, `${JSON.stringify({
    examples,
    usage,
    usageCall: `example(${usage.match(/fn example\(([^)]*)\)/)?.[1].split(",").map((arg) => arg.trim().split(":")[0]).filter(Boolean).join(", ") ?? ""})`,
    installComponents,
    source,
    api,
    parity,
    modules: [...new Set(installFiles.filter((file) => file.path.endsWith(".rs") && !helperModules.has(moduleName(file))).map(moduleName))],
  }, null, 2)}\n`);
}

mkdirSync(`${root}web/public/examples`, { recursive: true });
// The downloadable showcase source: the shell plus every preview file.
writeFileSync(`${root}web/public/examples/showcase.rs`, [read("site/src/main.rs"), ...["previews", "examples"].flatMap((folder) => readdirSync(`${root}site/src/${folder}`).sort().map((file) => `// ---- site/src/${folder}/${file} ----\n${read(`site/src/${folder}/${file}`)}`))].join("\n"));
copyFileSync(`${root}fixtures/registry-install/src/main.rs`, `${root}web/public/examples/hello.rs`);
mkdirSync(`${root}web/public/fonts`, { recursive: true });
mkdirSync(`${root}fixtures/registry-install/assets/fonts`, { recursive: true });
for (const font of readdirSync(`${root}site/assets/fonts`)) {
  copyFileSync(`${root}site/assets/fonts/${font}`, `${root}web/public/fonts/${font}`);
  copyFileSync(`${root}site/assets/fonts/${font}`, `${root}fixtures/registry-install/assets/fonts/${font}`);
}
copyFileSync(`${root}LICENSES/Geist-OFL-1.1`, `${root}web/public/fonts/OFL.txt`);
const metadata = JSON.parse(execFileSync("cargo", ["metadata", "--locked", "--format-version", "1"], { cwd: root, maxBuffer: 8 * 1024 * 1024 }));
const iconRoot = dirname(metadata.packages.find((item) => item.name === "gpui-icons").manifest_path);
const iconManifest = JSON.parse(readFileSync(`${iconRoot}/RELEASE-MANIFEST.json`, "utf8"));
assert.equal(iconManifest.icons.length, 1924, "The catalog requires the complete pinned gpui-icons library");
mkdirSync(`${root}web/public/icons/lucide`, { recursive: true });
for (const icon of iconManifest.icons) {
  copyFileSync(`${iconRoot}/assets/lucide/${icon.canonical_name}.svg`, `${root}web/public/${icon.asset_path}`);
}
copyFileSync(`${iconRoot}/LICENSE`, `${root}web/public/icons/LICENSE`);
writeFileSync(`${root}web/src/generated/icons.json`, JSON.stringify(iconManifest.icons.map((icon) => ({
  name: icon.canonical_name,
  rust: icon.rust_name,
  tags: icon.tags,
  categories: icon.categories,
  aliases: icon.aliases.map((alias) => typeof alias === "string" ? alias : alias.name),
}))));
console.log(`Synced ${registry.items.length - 1} compiled previews, APIs, and starter assets.`);

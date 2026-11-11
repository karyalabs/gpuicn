//! Turns registry.toml, the one list of components, into the gallery's
//! catalog table, so the gallery never names a component by hand.

use std::{env, fmt::Write as _, fs, path::Path};

/// A component is "new" while the current release is within this many minor
/// versions of the one that added it. web/src/lib/catalog.ts uses the same rule.
const NEW_FOR_MINOR_VERSIONS: u64 = 3;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let registry = Path::new(&manifest_dir).join("../registry.toml");
    println!("cargo:rerun-if-changed={}", registry.display());
    let manifest: toml::Table = fs::read_to_string(&registry)
        .expect("read registry.toml")
        .parse()
        .expect("parse registry.toml");
    let current = minor_line(&env::var("CARGO_PKG_VERSION").unwrap());

    let mut out = String::from("pub(crate) const CATEGORIES: &[(&str, &str)] = &[\n");
    for category in manifest["categories"].as_array().expect("[[categories]]") {
        let text = |key: &str| category[key].as_str().expect("category id and title");
        writeln!(out, "    ({:?}, {:?}),", text("id"), text("title")).unwrap();
    }
    out.push_str("];\n\npub(crate) const COMPONENTS: &[Entry] = &[\n");
    let components = manifest["components"].as_table().expect("[components]");
    // `toml::Table` iterates in key order, which is the catalog's A to Z.
    for (module, meta) in components {
        // Theme installs with every component and App is the starter; neither
        // has a preview.
        if module == "theme" || module == "app" {
            continue;
        }
        let text = |key: &str| meta.get(key).and_then(|value| value.as_str());
        let title = text("title").map_or_else(|| title_case(module), str::to_owned);
        let since = text("since").expect("since");
        let new = since == "next" || {
            let added = minor_line(since);
            added.0 == current.0 && current.1.saturating_sub(added.1) < NEW_FOR_MINOR_VERSIONS
        };
        writeln!(
            out,
            "    Entry {{ slug: {:?}, title: {:?}, description: {:?}, category: {:?}, new: {new} }},",
            module.replace('_', "-"),
            title,
            text("description").expect("description"),
            text("category").expect("category"),
        )
        .unwrap();
    }
    out.push_str("];\n");
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("catalog.rs"),
        out,
    )
    .unwrap();
}

fn minor_line(version: &str) -> (u64, u64) {
    let mut parts = version.split('.').map(|part| part.parse().unwrap_or(0));
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

fn title_case(module: &str) -> String {
    module
        .split('_')
        .map(|word| format!("{}{}", word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
}

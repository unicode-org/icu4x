// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;
use std::path::Path;

use cargo_metadata::{CargoOpt, MetadataCommand};
use diplomat_tool::config::Config;
use diplomat_tool::DocsUrlGenerator;

fn main() -> std::io::Result<()> {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../"));

    let Some(lang) = std::env::args().nth(1) else {
        panic!("Missing argument <language>");
    };

    let config_path = root.join("ffi/capi/config.toml");

    let mut library_config = Config::default();
    library_config.read_file(&config_path).unwrap();

    let metadata = MetadataCommand::new()
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    let dep_versions = metadata
        .packages
        .iter()
        .find(|p| p.name == "icu_capi")
        .unwrap()
        .dependencies
        .iter()
        .filter_map(|d| {
            if d.name.starts_with("diplomat") {
                return None;
            }
            Some((d.name.replace("-", "_"), {
                let c = &d.req.comparators[0];
                let mut version = c.major.to_string();
                if let Some(minor) = c.minor {
                    version = format!("{version}.{minor}");
                }
                if let Some(patch) = c.patch {
                    version = format!("{version}.{patch}");
                    if !c.pre.is_empty() {
                        version = format!("{version}-{}", c.pre);
                    }
                }
                version
            }))
        })
        .chain([(
            "icu".into(),
            metadata
                .packages
                .iter()
                .find(|p| p.name == "icu")
                .unwrap()
                .version
                .to_string(),
        )])
        .collect::<HashMap<_, _>>();

    let docs_url_gen = DocsUrlGenerator::with_base_urls(
        None,
        dep_versions
            .into_iter()
            .map(|(k, v)| {
                let url = if v.contains('-') {
                    // prerelease
                    format!("https://unicode-org.github.io/icu4x/rustdoc/{k}")
                } else {
                    format!("https://docs.rs/{k}/{v}")
                };
                (k, url)
            })
            .collect(),
    );

    diplomat_tool::gen(
        &root.join("ffi/capi/src/lib.rs"),
        lang.as_str(),
        &{
            let mut include = if lang != "demo_gen" {
                root.join("ffi/capi/bindings").join(&lang)
            } else {
                root.join("tools/web-demo/gen")
            };

            std::fs::remove_dir_all(&include)?;
            if lang == "cpp" {
                include = include.join("icu4x");
            }
            std::fs::create_dir_all(&include)?;
            include
        },
        &docs_url_gen,
        library_config,
        false,
    )
}

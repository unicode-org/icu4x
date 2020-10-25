// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use toml::Value;

const DOCS_PATH: &str = "https://unicode-org.github.io/icu4x-docs/doc/";

fn get_directories(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(entries)
}

fn get_crate_name(path: &Path) -> String {
    let mut cargo_path = path.to_path_buf();
    cargo_path.push("Cargo.toml");
    let cargo_toml = fs::read_to_string(cargo_path).unwrap();
    let value: Value = cargo_toml.parse().unwrap();
    let table = value.as_table().unwrap();
    table
        .get("package")
        .unwrap()
        .get("name")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn extract_doc_comment(input: &str) -> String {
    let lines: Vec<String> = input
        .lines()
        .skip_while(|line| !line.starts_with("//!"))
        .take_while(|line| line.starts_with("//!"))
        .map(|line| {
            line.replace("//! ", "")
                .replace("//!", "")
                .replace(": ../", &format!(": {}/doc/", DOCS_PATH))
                .replace(": ./", &format!(": {}/icu_locid/", DOCS_PATH))
                .replace("(../", &format!("({}/doc/", DOCS_PATH))
                .replace("(./", &format!("({}/icu_locid/", DOCS_PATH))
        })
        .collect();
    lines.join("\n")
}

const HEADER: &str = r#"# $CRATE_NAME$ [![crates.io](http://meritbadge.herokuapp.com/$CRATE_NAME$)](https://crates.io/crates/$CRATE_NAME$)

"#;

const FOOTER: &str = r#"
# More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
"#;

const PATHS: &[&str] = &["../../components/", "../../utils/"];

fn main() {
    for path in PATHS {
        let paths = get_directories(path).unwrap();
        for path in paths {
            let crate_name = get_crate_name(&path);

            let mut lib_path = path.clone();
            lib_path.push("src/lib.rs");
            let lib_rs = fs::read_to_string(lib_path).unwrap();

            let mut result = HEADER.replace("$CRATE_NAME$", &crate_name);

            result.push_str(&extract_doc_comment(&lib_rs));
            result.push_str(FOOTER);

            let mut readme_path = path.clone();
            readme_path.push("README.md");

            fs::write(readme_path, result).expect("Unable to write file");
        }
    }
}

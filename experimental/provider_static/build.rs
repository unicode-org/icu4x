// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde_json::{json, Map, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

const DATA_PATH: &str = "../../provider/testdata/data/json/";

fn handle_path(map: &mut Map<String, Value>, path: &Path) {
    let me = path.file_stem().unwrap().to_str().unwrap();
    if path.is_dir() {
        let mut children = Map::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            handle_path(&mut children, &entry.path());
        }
        map.insert(
            me.into(),
            json!({
                "ty": "Dir",
                "contents": children,
            }),
        );
    } else {
        let bytes = fs::read(path).unwrap();
        // This serializes each file as a string; it is also possible to serialize them
        // as a normal json value if we'd like by converting to serde_json::Value first
        let s = str::from_utf8(&bytes).unwrap();
        map.insert(
            me.into(),
            json!({
                "ty": "File",
                "contents": s,
            }),
        );
    }
}

fn main() {
    let mut value: Map<String, Value> = Map::new();
    let path = PathBuf::from(DATA_PATH);
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        handle_path(&mut value, &entry.path());
    }
    let output = serde_json::to_string_pretty(&value).unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("static_data.json");
    fs::write(&dest_path, output).unwrap();
}

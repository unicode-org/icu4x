// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

const DATA_PATH: &str = "../../provider/testdata/data/json/";

fn handle_path(map: &mut HashMap<String, String>, key: &str, path: &Path) {
    let me = path.file_stem().unwrap().to_str().unwrap();
    let me_key = String::new() + key + "/" + me;
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            handle_path(&mut *map, &me_key, &entry.path());
        }
    } else {
        let bytes = fs::read(path).unwrap();
        // This serializes each file as a string; it is also possible to serialize them
        // as a normal json value if we'd like by converting to serde_json::Value first
        let s = str::from_utf8(&bytes).unwrap();
        map.insert(me_key, s.into());
    }
}

fn main() {
    let mut value: HashMap<String, String> = HashMap::new();
    let path = PathBuf::from(DATA_PATH);
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        handle_path(&mut value, "", &entry.path());
    }
    let output = bincode::serialize(&value).unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("static_data.bincode");
    fs::write(&dest_path, output).unwrap();
}

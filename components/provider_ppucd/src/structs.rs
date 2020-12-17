// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use serde::Deserialize;
use tinystr;
use crate::error::Error;

macro_rules! data_key {
    (uniset, $sub_category:literal, $version:tt) => {
        data_key!(icu_provider::DataCategory::Uniset, $sub_category, $version)
    };

    // this was copied over from `provider` crate, but maybe should be refacotred

    ($category:expr, $sub_category:literal, $version:tt) => {
        icu_provider::DataKey {
            category: $category,
            sub_category: tinystr::tinystr16!($sub_category),
            version: $version,
        }
    };
}

pub mod key {
    use icu_provider::DataKey;
    pub const WSPACE_V1: DataKey = data_key!(uniset, "wspace", 1);
}

#[derive(Deserialize, Debug)]
pub struct PpucdResource {
    pub properties: Vec<i32>,
}


#[test]
fn test_basic() {
    let json_str: &str = 
        r#"{ "properties": [9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287, 8288, 12288, 12289] } "#;
    let deserialize_result: Result<PpucdResource, serde_json::Error> = serde_json::from_str(json_str);
    let resource = deserialize_result.unwrap();
    println!("***** parsed struct = {:?}", resource);
}

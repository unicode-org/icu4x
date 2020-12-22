// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use serde::{Deserialize, Serialize};
use tinystr;
use crate::error::Error;
use icu_uniset::UnicodeSet;
use std::convert::TryInto;

//
// data key structs - the structs used directly by users of data provider
//

macro_rules! data_key {
    (uniset, $sub_category:literal, $version:tt) => {
        data_key!(icu_provider::DataCategory::Uniset, $sub_category, $version)
    };

    // this was copied over from `provider` crate, but maybe should be refactored

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

// struct FakePropertyUnicodeSet {
//     set: UnicodeSet,
// }

// Have to use inversion lists as the data argument for the property enum types
// because cannot create a DataResponse directly (b/c CloneableAny trait is private
// in Provider) and the .with_owned_payload() method of Provider requires the payload
// type to implement Clone, Debug, Serialize. Would want to use UnicodeSet as payload
// type but implementing these traits would make UnicodeSet depend on serde & serde_json,
// which I think we want to avoid to keep that building block lib clean and lean.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum PropertyUnicodeSetV1 {
    Wspace(Vec<u32>),
}


//
// JSON structs - the structs that define the schema of JSON data and
//     effecively become the containers of data between JSON and Rust
//     via serde_json
//

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct PpucdProperty {
    pub name: String,
    pub inv_list: Vec<u32>,
}

impl PpucdProperty {
    /// Default empty nameless property
    pub fn default() -> PpucdProperty {
        PpucdProperty {
            name: String::new(),
            inv_list: vec![],
        }
    }

    /// Converts a UnicodeSet into an inversion list since UnicodeSet does not
    /// expose a public method to convert to inversion lists.
    pub fn uniset_to_inv_list(s: &UnicodeSet) -> Vec<u32> {
        let mut start_code_point: i32 = -1;
        let mut end_code_point: i32 = -1;
        let mut inv_list: Vec<u32> = vec![];
        for ch in s.iter() {
            let cp = ch as i32;
            if start_code_point < 0 && end_code_point < 0 {
                start_code_point = cp;
                end_code_point = cp;
            }
            else if cp == end_code_point + 1 {
                end_code_point = end_code_point + 1;
            } else {
                inv_list.push(start_code_point as u32);
                inv_list.push((end_code_point + 1) as u32);
                start_code_point = cp;
                end_code_point = cp;
            }
        }
        if start_code_point >= 0 && end_code_point >= 0 {
            inv_list.push(start_code_point as u32);
            inv_list.push((end_code_point + 1) as u32);
        }
        inv_list
    }

    /// Converts a UnicodeSet into a corresponding PpucdProperty struct for 
    /// serde JSON de-/serialization.
    pub fn from_uniset(s: &UnicodeSet, name: &str) -> PpucdProperty {
        let inv_list = PpucdProperty::uniset_to_inv_list(s);
        PpucdProperty {
            name: String::from(name),
            inv_list,
        }
    }
}

impl TryInto<UnicodeSet> for PpucdProperty {
    type Error = crate::error::Error;
    fn try_into(self) -> Result<UnicodeSet, Self::Error> {
        UnicodeSet::from_inversion_list(self.inv_list)
            .map_err(|e| crate::error::Error::UnisetConversion(e))
    }
}

#[test]
fn test_uniset_to_inv_list() {
    let inv_list: Vec<u32> = vec![9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287, 8288, 12288, 12289];
    let inv_list_clone = (&inv_list).clone();
    let s: UnicodeSet = UnicodeSet::from_inversion_list(inv_list_clone).unwrap();
    let round_trip_inv_list = PpucdProperty::uniset_to_inv_list(&s);
    assert_eq!(round_trip_inv_list, inv_list);
}
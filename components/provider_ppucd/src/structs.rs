// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_uniset::UnicodeSet;
use std::convert::TryInto;

//
// data key structs - the structs used directly by users of data provider
//

pub mod key {
    use icu_provider::resource_key;
    use icu_provider::ResourceKey;
    pub const WSPACE_V1: ResourceKey = resource_key!(uniset, "WSpace", 1);
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnicodeProperties<'s> {
    pub props: Vec<UnicodeProperty<'s>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnicodeProperty<'s> {
    pub name: &'s str,
    pub inv_list: Vec<u32>,
}

impl Default for UnicodeProperty<'static> {
    fn default() -> UnicodeProperty<'static> {
        UnicodeProperty {
            name: "",
            inv_list: vec![],
        }
    }
}

impl<'s> UnicodeProperty<'s> {
    /// Default empty nameless property

    pub fn from_uniset(set: &UnicodeSet, name: &'s str) -> UnicodeProperty<'s> {
        let inv_list = set.get_inversion_list();
        UnicodeProperty {
            name,
            inv_list,
        }
    }
}

impl<'s> TryInto<UnicodeSet> for UnicodeProperty<'s> {
    type Error = crate::error::Error;
    fn try_into(self) -> Result<UnicodeSet, Self::Error> {
        UnicodeSet::from_inversion_list(self.inv_list)
            .map_err(crate::error::Error::UnisetConversion)
    }
}

#[test]
fn test_uniset_to_inv_list() {
    let inv_list: Vec<u32> = vec![
        9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287,
        8288, 12288, 12289,
    ];
    let inv_list_clone = (&inv_list).clone();
    let s: UnicodeSet = UnicodeSet::from_inversion_list(inv_list_clone).unwrap();
    let round_trip_inv_list = s.get_inversion_list();
    assert_eq!(round_trip_inv_list, inv_list);
}

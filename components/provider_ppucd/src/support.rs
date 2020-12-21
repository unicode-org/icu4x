// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSet;
use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use crate::structs::key;
use crate::structs::PpucdProperty;


const FAKE_PAYLOAD: &str = "I am a payload?! :|";
const FAKE_PATH: &str = "some-ppucd-file.txt";

#[derive(Debug)]
pub struct PpucdDataProvider<'d> {
    pub ppucd_prop_data: PpucdProperty,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl<'d> PpucdDataProvider<'d> {
    pub fn new(ppucd_prop_path: &str) -> Self {
        let ppucd_prop_path_string = ppucd_prop_path.to_string();
        let data_rdr: BufReader<File> =
            File::open(&ppucd_prop_path)
                .map(BufReader::new)
                .unwrap();
        let data: PpucdProperty =
            serde_json::from_reader(data_rdr)
                .unwrap();
        PpucdDataProvider {
            ppucd_prop_data: data,
            _phantom: PhantomData,
        }
    }
}

impl<'d> DataProvider<'d> for PpucdDataProvider<'d> {
    fn load(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError> {
        const UND: LanguageIdentifier = langid!("und");
        // let result: Result<UnicodeSet, DataError> = 
        //     (&self).ppucd_prop_data
        //         .try_into()
        //         .map_err(|e| DataError::UnavailableEntry(req.clone()));
        // let set: UnicodeSet = result.unwrap();
        // let set: UnicodeSet =
        //     UnicodeSet::from_inversion_list(
        //         (&self).ppucd_prop_data.inv_list.clone()
        //     ).unwrap();
        let data_key_str = req.data_key.sub_category.as_str();
        let data: &PpucdProperty = &self.ppucd_prop_data;
        // Note: cannot return as a struct/enum containing a field of type
        // UnicodeSet because UnicodeSet does not implement Clone, Debug, Serialize, etc.
        let payload: PpucdProperty = data.clone();
        if payload.name != data_key_str {
            let data_err: DataError = req.clone().into();
            return Err(data_err);
        }
        Ok(
            DataResponseBuilder {
                data_langid: UND,
            }
            .with_owned_payload(payload)
        )
    }
}

impl<'d> TryFrom<&'d str> for PpucdDataProvider<'d> {
    type Error = serde_json::error::Error;
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let data: PpucdProperty = serde_json::from_str(s)?;
        Ok(PpucdDataProvider {
            ppucd_prop_data: data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> From<File> for PpucdDataProvider<'d> {
    fn from(prop_file: File) -> Self {
        let data_rdr: BufReader<File> =
            BufReader::new(prop_file);
        let data: PpucdProperty =
            serde_json::from_reader(data_rdr)
                .unwrap();
        PpucdDataProvider {
              ppucd_prop_data: data,
              _phantom: PhantomData,
        }
    }
}

impl<'d> TryInto<String> for PpucdDataProvider<'d> {
    type Error = serde_json::error::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        let data: PpucdProperty = self.ppucd_prop_data;
        serde_json::to_string(&data)
    }
}

#[test]
fn test_json_serde() {
    let json_str: &str =
        r#"{
            "name": "wspace",
            "inv_list" : [9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287, 8288, 12288, 12289]
            }"#;
    let deserialize_result: Result<PpucdProperty, serde_json::Error> = serde_json::from_str(json_str);
    let ppucd_property = deserialize_result.unwrap();
    let exp_property = PpucdProperty {
        name: String::from("wspace"),
        inv_list: vec![9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287, 8288, 12288, 12289],
    };
    assert_eq!(exp_property, ppucd_property);
}

#[test]
fn test_json_serde_manual_file_parse() {
    let ppucd_property_files_root_path = "tests/testdata/wspace.json";
    let json_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let deserialize_result: Result<PpucdProperty, serde_json::Error> = serde_json::from_str(&json_str);
    let ppucd_property = deserialize_result.unwrap();
    let exp_property = PpucdProperty {
        name: String::from("wspace"),
        inv_list: vec![9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287, 8288, 12288, 12289],
    };
    assert_eq!(exp_property, ppucd_property);
}

// How to make this test work? A little confused what to do to make this
// answer work: https://stackoverflow.com/questions/57234140/how-to-assert-errors-in-rust
// But I can see that the data key is being compared to the data before being returned,
// so skipping for now.
// #[test]
// fn test_ppucd_provider_resp_manual_file_parse_error() {
//     let ppucd_property_files_root_path = "tests/testdata/wspace)bad.json";
//     let ppucd_property_file = File::open(ppucd_property_files_root_path).unwrap();
//     let ppucd_provider: PpucdDataProvider = PpucdDataProvider::from( ppucd_property_file );
//     const UND: LanguageIdentifier = langid!("und");
//     let data_req = DataRequest {
//         data_key: key::WSPACE_V1,
//         data_entry: DataEntry {
//             variant: None,
//             langid: UND,
//         },
//     };
//     let exp_err = DataError::UnsupportedDataKey(key::WSPACE_V1);
//     assert_eq!(ppucd_provider.load(&data_req), exp_err);
// }

#[test]
fn test_ppucd_provider_resp_manual_file_parse() {
    let ppucd_property_files_root_path = "tests/testdata/wspace.json";
    let ppucd_property_file = File::open(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::from( ppucd_property_file );
    const UND: LanguageIdentifier = langid!("und");
    let data_req = DataRequest {
        data_key: key::WSPACE_V1,
        data_entry: DataEntry {
            variant: None,
            langid: UND,
        },
    };
    let resp: DataResponse = 
        ppucd_provider
            .load(&data_req)
            .unwrap();
    
    let ppucd_property_cow: Cow<PpucdProperty> =
        resp
            .take_payload()
            .unwrap();
    let exp_property = PpucdProperty {
        name: String::from("wspace"),
        inv_list: vec![9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287, 8288, 12288, 12289],
    };
    assert_eq!(exp_property, ppucd_property_cow.into_owned());
}

// fn test_ppucd_provider_resp_fs_provider_dir_load() {
//     let ppucd_property_files_root_path = "tests/testdata/wspace.json";
//     let ppucd_provider = PpucdDataProvider::new( &ppucd_property_files_root_path );
//     // TODO: make the type and data actually correspond to the data request
//     const UND: LanguageIdentifier = langid!("und");
//     let resp: DataResponse = 
//         ppucd_provider
//             .load(&DataRequest {
//                 data_key: key::WSPACE_V1,
//                 data_entry: DataEntry {
//                     variant: None,
//                     langid: UND,
//                 },
//             })
//             .unwrap();
//     // println!("data resp: {:?}", resp);
//     let some_payload: Cow<&str> =
//         resp
//             .take_payload()
//             .unwrap();   
// }
// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::prelude::*;
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
        Ok(
            DataResponseBuilder {
                data_langid: UND,
            }
            .with_borrowed_payload(&FAKE_PAYLOAD)
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
}

#[test]
fn test_json_serde_manual_file_load() {
    let ppucd_property_file_path = "tests/testdata/wspace.json";
    let json_str = std::fs::read_to_string(ppucd_property_file_path).unwrap();
    let deserialize_result: Result<PpucdProperty, serde_json::Error> = serde_json::from_str(&json_str);
    let ppucd_property = deserialize_result.unwrap();
    
}

#[test]
fn test_basic_fake_resp() {
    let ppucd_property_file_path = "tests/testdata/wspace.json";
    let ppucd_provider = PpucdDataProvider::new( &ppucd_property_file_path );
    // TODO: make the type and data actually correspond to the data request
    const UND: LanguageIdentifier = langid!("und");
    let fake_resp_result: Result<DataResponse, DataError> = 
        ppucd_provider
            .load(&DataRequest {
                data_key: key::WSPACE_V1,
                data_entry: DataEntry {
                    variant: None,
                    langid: UND,
                },
            });
    println!("data resp: {:?}", fake_resp_result);
    let fake_resp: DataResponse = fake_resp_result.unwrap();
    let fake_payload_result: Result<Cow<&str>, DataError> =
        fake_resp
            .take_payload();
    let fake_payload: Cow<&str> =
        fake_payload_result
            .unwrap();
}
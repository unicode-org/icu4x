// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use crate::structs::PpucdResource;


const FAKE_PAYLOAD: &str = "I am a payload?! :|";
const FAKE_PATH: &str = "some-ppucd-file.txt";

#[derive(Debug)]
pub struct PpucdDataProvider<'d> {
    pub ppucd_data: PpucdResource,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl<'d> PpucdDataProvider<'d> {
    pub fn new(ppucd_path: &String) -> Self {
        let ppucd_path_string = ppucd_path.to_string();
        let fake_resource: PpucdResource = PpucdResource::default();
        let data_rdr: BufReader<File> =
            File::open(&ppucd_path)
                .map(BufReader::new)
                .unwrap();
        let data: PpucdResource =
            serde_json::from_reader(data_rdr)
                .unwrap();
        PpucdDataProvider {
            ppucd_data: fake_resource,
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
        let data: PpucdResource = serde_json::from_str(s)?;
        Ok(PpucdDataProvider {
            ppucd_data: data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> TryInto<String> for PpucdDataProvider<'d> {
    type Error = serde_json::error::Error;
    fn try_into(self) -> Result<String, Self::Error> {
        let data: PpucdResource = self.ppucd_data;
        serde_json::to_string(&data)
    }
}
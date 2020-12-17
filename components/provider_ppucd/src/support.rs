// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::marker::PhantomData;
use crate::structs::PpucdResource;


const FAKE_PAYLOAD: &str = "I am a payload?! :|";
const FAKE_PATH: &str = "some-ppucd-file.txt";

#[derive(Debug)]
pub struct PpucdDataProvider<'d> {
    pub ppucd_path: String,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl<'d> PpucdDataProvider<'d> {
    pub fn new(ppucd_path: &String) -> Self {
        let ppucd_path_string = ppucd_path.to_string();
        PpucdDataProvider {
            ppucd_path: ppucd_path_string,
            _phantom: PhantomData,
        }
    }
}

impl<'d> DataProvider<'d> for PpucdDataProvider<'d> {
    fn load(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError> {
        const UND: LanguageIdentifier = langid!("und");
        // Fake payload
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
    /// Attempt to parse a JSON string.
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let data: PpucdResource = serde_json::from_str(s)?;
        Ok(PpucdDataProvider {
            ppucd_path: String::from(FAKE_PATH),
            _phantom: PhantomData,
        })
    }
}
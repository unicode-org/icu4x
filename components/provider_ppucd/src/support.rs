// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//
//use icu_provider::prelude::*;

#[derive(Debug)]
pub struct PpucdDataProvider {
    pub ppucd_path: String,
}

impl PpucdDataProvider {
    pub fn new(ppucd_path: &String) -> Self {
        let ppucd_path_string = ppucd_path.to_string();
        PpucdDataProvider {
            ppucd_path: ppucd_path_string,
        }
    }
}

// impl DataProvider<'a, 'd> for PpucdDataProvider {
//     fn load<'a>(&'a self, req: &DataRequest) -> Result<DataResponse<'d>, Error> {
//     }
// }
// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::parse_ppucd;
use crate::structs::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct PpucdDataProvider<'d> {
    pub ppucd_props: UnicodeProperties,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl<'d> PpucdDataProvider<'d> {
    pub fn new(prop_str: &str) -> Self {
        let data: UnicodeProperties = parse_ppucd::parse(prop_str.to_string());
        PpucdDataProvider {
            ppucd_props: data,
            _phantom: PhantomData,
        }
    }

    pub fn from_prop(ppucd_prop: UnicodeProperty) -> Self {
        PpucdDataProvider {
            ppucd_props: UnicodeProperties {
                props: vec![ppucd_prop],
            },
            _phantom: PhantomData,
        }
    }
}

// Similar to provider_cldr for plurals
impl<'d> DataProvider<'d, UnicodeProperty> for PpucdDataProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, UnicodeProperty>, DataError> {
        let resc_key: &ResourceKey = &req.resource_path.key;
        let resc_key_str: &str = resc_key.sub_category.as_str();
        let props_data: &UnicodeProperties = &self.ppucd_props;
        let matching_prop: Option<&UnicodeProperty> =
            props_data.props.iter().find(|p| p.name == resc_key_str);
        let owned_matching_prop: Option<UnicodeProperty> = matching_prop.cloned();
        let prop = match owned_matching_prop {
            Some(p) => p,
            None => return Err(req.clone().into()),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata { data_langid: None },
            payload: Some(Cow::Owned(prop)),
        })
        // let prop_as_result: Result<UnicodeProperty, DataError> =
        //     owned_matching_prop.ok_or_else(|| DataError::from(req.clone()));
        // prop_as_result.map(|p| DataResponseBuilder { data_langid: UND }.with_owned_payload(p))
    }
}

impl<'d> TryFrom<&'d str> for PpucdDataProvider<'d> {
    type Error = DataError;
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let props_data: UnicodeProperties = parse_ppucd::parse(String::from(s));
        Ok(PpucdDataProvider {
            ppucd_props: props_data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> From<File> for PpucdDataProvider<'d> {
    fn from(prop_file: File) -> Self {
        let mut file_contents = String::new();
        let _result = (&prop_file).read_to_string(&mut file_contents);
        let props_data: UnicodeProperties = parse_ppucd::parse(file_contents);
        PpucdDataProvider {
            ppucd_props: props_data,
            _phantom: PhantomData,
        }
    }
}

#[test]
fn test_ppucd_provider_parse() {
    let ppucd_property_files_root_path = "tests/testdata/ppucd-wspace-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);
    // const UND: LanguageIdentifier = langid!("und");
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: key::WSPACE_V1,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };
    let mut resp: DataResponse<UnicodeProperty> = ppucd_provider.load_payload(&data_req).unwrap();

    let ppucd_property_cow: Cow<UnicodeProperty> = resp.take_payload().unwrap();
    let exp_prop_uniset: UnicodeProperty = UnicodeProperty {
        name: String::from("WSpace"),
        inv_list: vec![
            9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240,
            8287, 8288, 12288, 12289,
        ],
    };
    assert_eq!(exp_prop_uniset, ppucd_property_cow.into_owned());
}

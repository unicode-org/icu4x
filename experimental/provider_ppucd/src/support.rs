// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parse_ppucd;
use icu_provider::iter::IterableDataProviderCore;
use icu_provider::prelude::*;
use icu_uniset::provider::*;

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub struct UnicodeProperties<'s> {
    pub props: Vec<UnicodePropertyV1<'s>>,
}

/// Provides Unicode binary properties by reading from a ppucd.txt data file.
///
/// # Example
///
/// Note: The following example shows a proof of councept of how PpucdDataProvider _could_ work.
///
/// ```ignore
/// use icu_provider::prelude::*;
/// use icu_provider_ppucd::PpucdDataProvider;
/// use icu_uniset::provider::{key, UnicodePropertyV1};
/// use std::borrow::Cow;
///
/// let ppucd_data = std::fs::read_to_string(icu_testdata::paths::ppucd_path())
///     .expect("Data should be present");
/// // TODO(#454): Enable this part of the docs test once PpucdDataProvider is made faster.
/// let provider = PpucdDataProvider::new(&ppucd_data);
///
/// let result: Cow<UnicodePropertyV1> = provider
///     .load_payload(&DataRequest::from(key::ASCII_HEX_DIGIT_V1))
///     .expect("Data should be well-formed")
///     .payload
///     .take()
///     .expect("Key should be present");
///
/// assert_eq!(result, Cow::Owned(UnicodePropertyV1 {
///     name: Cow::Borrowed("AHex"),
///     inv_list: vec![48, 58, 65, 71, 97, 103],
/// }));
/// ```
#[derive(Debug)]
pub struct PpucdDataProvider<'s> {
    pub ppucd_props: UnicodeProperties<'s>,
}

impl<'s> PpucdDataProvider<'s> {
    pub fn new(prop_str: &'s str) -> Self {
        let data: UnicodeProperties = parse_ppucd::parse(prop_str);
        PpucdDataProvider { ppucd_props: data }
    }

    pub fn from_prop(ppucd_prop: UnicodePropertyV1<'s>) -> Self {
        PpucdDataProvider {
            ppucd_props: UnicodeProperties {
                props: vec![ppucd_prop],
            },
        }
    }
}

impl<'d, 's> DataProvider<'d, 's, UnicodePropertyMarker> for PpucdDataProvider<'s> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, UnicodePropertyMarker>, DataError> {
        let resc_key: &ResourceKey = &req.resource_path.key;
        let resc_key_str: &str = resc_key.sub_category.as_str();
        let props_data: &UnicodeProperties = &self.ppucd_props;
        let matching_prop: Option<&UnicodePropertyV1> =
            props_data.props.iter().find(|p| p.name == resc_key_str);
        let owned_matching_prop: Option<UnicodePropertyV1> = matching_prop.cloned();
        let prop = match owned_matching_prop {
            Some(p) => p,
            None => return Err(req.clone().into()),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata { data_langid: None },
            payload: Some(DataPayload::from_partial_owned(std::rc::Rc::from(prop))),
        })
    }
}

impl<'s> TryFrom<&'s str> for PpucdDataProvider<'s> {
    type Error = DataError;
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        let props_data: UnicodeProperties = parse_ppucd::parse(s);
        Ok(PpucdDataProvider {
            ppucd_props: props_data,
        })
    }
}

icu_provider::impl_dyn_provider!(PpucdDataProvider<'s>, {
    _ => UnicodePropertyMarker,
}, SERDE_SE, 'd, 's);

impl<'d> IterableDataProviderCore for PpucdDataProvider<'d> {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[test]
fn test_ppucd_provider_parse() {
    use icu_uniset::UnicodeSet;
    use std::borrow::Cow;

    let ppucd_property_files_root_path = "tests/testdata/ppucd-wspace-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: key::WHITE_SPACE_V1,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };
    let resp: DataResponse<UnicodePropertyMarker> = ppucd_provider.load_payload(&data_req).unwrap();

    let ppucd_property_cow: DataPayload<UnicodePropertyMarker> = resp.take_payload().unwrap();
    let exp_prop_uniset: UnicodePropertyV1 = UnicodePropertyV1 {
        name: Cow::Borrowed("WSpace"),
        inv_list: UnicodeSet::from_inversion_list(vec![
            9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240,
            8287, 8288, 12288, 12289,
        ])
        .unwrap(),
    };
    assert_eq!(exp_prop_uniset, ppucd_property_cow.get().clone());
}

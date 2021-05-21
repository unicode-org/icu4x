// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider returning multilingual "Hello World" strings for testing.

use crate::iter::IterableDataProviderCore;
use crate::prelude::*;
use icu_locid::LanguageIdentifier;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

pub mod key {
    use crate::resource::ResourceKey;
    pub const HELLO_WORLD_V1: ResourceKey = resource_key!(icu4x, "helloworld", 1);
}

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct HelloWorldV1<'s> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub message: Cow<'s, str>,
}

impl Default for HelloWorldV1<'_> {
    fn default() -> Self {
        HelloWorldV1 {
            message: Cow::Borrowed("(und) Hello World"),
        }
    }
}

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::{key, HelloWorldProvider, HelloWorldV1};
/// use icu_provider::prelude::*;
/// use icu_locid_macros::langid;
///
/// let provider = HelloWorldProvider::new_with_placeholder_data();
///
/// let german_hello_world: DataPayload<HelloWorldV1> = provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("de")),
///             }
///         }
///     })
///     .unwrap()
///     .take_payload()
///     .unwrap();
///
/// assert_eq!("Hallo Welt", german_hello_world.message);
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct HelloWorldProvider<'s> {
    map: HashMap<LanguageIdentifier, Cow<'s, str>>,
}

impl<'s> HelloWorldProvider<'s> {
    /// Creates a [`HelloWorldProvider`] pre-populated with hardcoded data from Wiktionary.
    pub fn new_with_placeholder_data() -> HelloWorldProvider<'static> {
        // Data from https://en.wiktionary.org/wiki/Hello_World#Translations
        // Note: we don't want to use langid!() because icu_langid_macros is heavy.
        HelloWorldProvider {
            map: [
                ("bn", "ওহে বিশ্ব"),
                ("cs", "Ahoj světe"),
                ("de", "Hallo Welt"),
                ("el", "Καλημέρα κόσμε"),
                ("en", "Hello World"),
                ("eo", "Saluton, Mondo"),
                ("fa", "سلام دنیا‎"),
                ("fi", "hei maailma"),
                ("is", "Halló, heimur"),
                ("ja", "こんにちは世界"),
                ("la", "Ave, munde"),
                ("ro", "Salut,lume!"),
                ("ru", "Привет, мир"),
                ("vi", "Xin chào thế giới"),
                ("zh", "你好世界"),
            ]
            .iter()
            .map(|(loc, value)| {
                (
                    LanguageIdentifier::from_str(loc).unwrap(),
                    Cow::Borrowed(*value),
                )
            })
            .collect(),
        }
    }
}

impl<'d, 's> DataProvider<'d, HelloWorldV1<'s>> for HelloWorldProvider<'s>
where
    's: 'd,
{
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, HelloWorldV1<'s>>, DataError> {
        req.resource_path.key.match_key(key::HELLO_WORLD_V1)?;
        let langid = req.try_langid()?;
        let data = self
            .map
            .get(langid)
            .map(|s| HelloWorldV1 { message: s.clone() })
            .ok_or_else(|| DataError::UnavailableResourceOptions(req.clone()))?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: Some(langid.clone()),
            },
            payload: Some(DataPayload {
                cow: Cow::Owned(data),
            }),
        })
    }
}

impl_dyn_provider!(HelloWorldProvider<'static>, {
    _ => HelloWorldV1<'static>,
}, ERASED, 'd, 's);

#[cfg(feature = "provider_serde")]
impl_dyn_provider!(HelloWorldProvider<'s>, {
    _ => HelloWorldV1<'s>,
}, SERDE_SE, 'd, 's);

impl<'d> IterableDataProviderCore for HelloWorldProvider<'d> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        resc_key.match_key(key::HELLO_WORLD_V1)?;
        let list: Vec<ResourceOptions> = self
            .map
            .keys()
            .map(|langid| ResourceOptions {
                variant: None,
                langid: Some(langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

/// Adds entries to a [`HelloWorldProvider`] from [`ErasedDataStruct`](crate::erased::ErasedDataStruct)
impl crate::export::DataExporter<'_, dyn crate::erased::ErasedDataStruct>
    for HelloWorldProvider<'static>
{
    fn put_payload(
        &mut self,
        req: &DataRequest,
        payload: &dyn crate::erased::ErasedDataStruct,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        req.resource_path.key.match_key(key::HELLO_WORLD_V1)?;
        let langid = req.try_langid()?;
        let data: &HelloWorldV1 = payload.downcast_ref()?;
        self.map.insert(langid.clone(), data.message.clone());
        Ok(())
    }

    fn include_resource_options(&self, _resc_options: &ResourceOptions) -> bool {
        true
    }
}

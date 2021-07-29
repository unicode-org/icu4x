// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider returning multilingual "Hello World" strings for testing.

use crate::iter::IterableDataProviderCore;
use crate::prelude::*;
use crate::yoke::{self, *};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::str::FromStr;
use icu_locid::LanguageIdentifier;
use litemap::LiteMap;

pub mod key {
    use crate::resource::ResourceKey;
    pub const HELLO_WORLD_V1: ResourceKey = resource_key!(Core, "helloworld", 1);
}

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct HelloWorldV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub message: Cow<'data, str>,
}

impl Default for HelloWorldV1<'_> {
    fn default() -> Self {
        HelloWorldV1 {
            message: Cow::Borrowed("(und) Hello World"),
        }
    }
}

/// Marker type for [`HelloWorldV1`].
pub struct HelloWorldV1Marker;

impl<'data> DataMarker<'data> for HelloWorldV1Marker {
    type Yokeable = HelloWorldV1<'static>;
    type Cart = HelloWorldV1<'data>;
}

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::{key, HelloWorldProvider, HelloWorldV1Marker};
/// use icu_provider::prelude::*;
/// use icu_locid_macros::langid;
///
/// let provider = HelloWorldProvider::new_with_placeholder_data();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("de")),
///             }
///         }
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct HelloWorldProvider<'data> {
    map: LiteMap<LanguageIdentifier, Cow<'data, str>>,
}

impl<'data> HelloWorldProvider<'data> {
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

impl<'data, 't> DataProvider<'data, HelloWorldV1Marker> for HelloWorldProvider<'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, HelloWorldV1Marker>, DataError> {
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
            payload: Some(DataPayload::from_partial_owned(Rc::from(data))),
        })
    }
}

impl_dyn_provider!(HelloWorldProvider<'static>, {
    _ => HelloWorldV1Marker,
}, ERASED);

#[cfg(feature = "provider_serde")]
impl_dyn_provider!(HelloWorldProvider<'data>, {
    _ => HelloWorldV1Marker,
}, SERDE_SE, 'data);

impl<'data> IterableDataProviderCore for HelloWorldProvider<'data> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        resc_key.match_key(key::HELLO_WORLD_V1)?;
        let list: Vec<ResourceOptions> = self
            .map
            .iter_keys()
            .map(|langid| ResourceOptions {
                variant: None,
                langid: Some(langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

/// Adds entries to a [`HelloWorldProvider`] from [`ErasedDataStruct`](crate::erased::ErasedDataStruct)
impl<'data> crate::export::DataExporter<'static, crate::erased::ErasedDataStructMarker>
    for HelloWorldProvider<'static>
{
    fn put_payload(
        &mut self,
        req: DataRequest,
        payload: DataPayload<'static, crate::erased::ErasedDataStructMarker>,
    ) -> Result<(), DataError> {
        req.resource_path.key.match_key(key::HELLO_WORLD_V1)?;
        let langid = req.try_langid()?;
        let downcast_payload: DataPayload<HelloWorldV1Marker> = payload.downcast()?;
        self.map.insert(
            langid.clone(),
            Cow::Owned(downcast_payload.get().message.to_string()),
        );
        Ok(())
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::erased::ErasedDataStruct;
use crate::iter::DataExporter;
use crate::prelude::*;
use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

// Re-export symbols from structs::icu4x
// TODO(#415): Move these symbols to this file.
pub use crate::structs::icu4x::*;

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Example
///
/// ```
/// use icu_provider::hello_world::{key, HelloWorldProvider};
/// use icu_provider::prelude::*;
/// use icu_locid_macros::langid;
///
/// let provider = HelloWorldProvider::new_with_placeholder_data();
///
/// let german_hello_world = provider.load_payload(&DataRequest {
///     resource_path: ResourcePath {
///         key: key::HELLO_WORLD_V1,
///         options: ResourceOptions {
///             variant: None,
///             langid: Some(langid!("de")),
///         }
///     }
/// }).unwrap().take_payload().unwrap();
///
/// assert_eq!("Hallo Welt", german_hello_world.message);
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct HelloWorldProvider<'s> {
    map: HashMap<LanguageIdentifier, Cow<'s, str>>,
}

impl<'s> HelloWorldProvider<'s> {
    /// Creates a HelloWorldProvider pre-populated with hardcoded data from Wiktionary.
    pub fn new_with_placeholder_data() -> HelloWorldProvider<'static> {
        let mut map = HashMap::new();
        // Data from https://en.wiktionary.org/wiki/Hello_World#Translations
        map.insert(langid!("bn"), Cow::Borrowed("ওহে বিশ্ব"));
        map.insert(langid!("cs"), Cow::Borrowed("Ahoj světe"));
        map.insert(langid!("de"), Cow::Borrowed("Hallo Welt"));
        map.insert(langid!("el"), Cow::Borrowed("Καλημέρα κόσμε"));
        map.insert(langid!("en"), Cow::Borrowed("Hello World"));
        map.insert(langid!("eo"), Cow::Borrowed("Saluton, Mondo"));
        map.insert(langid!("fa"), Cow::Borrowed("سلام دنیا‎"));
        map.insert(langid!("fi"), Cow::Borrowed("hei maailma"));
        map.insert(langid!("is"), Cow::Borrowed("Halló, heimur"));
        map.insert(langid!("ja"), Cow::Borrowed("こんにちは世界"));
        map.insert(langid!("la"), Cow::Borrowed("Ave, munde"));
        map.insert(langid!("ro"), Cow::Borrowed("Salut,lume!"));
        map.insert(langid!("ru"), Cow::Borrowed("Привет, мир"));
        map.insert(langid!("vi"), Cow::Borrowed("Xin chào thế giới"));
        map.insert(langid!("zh"), Cow::Borrowed("你好世界"));
        HelloWorldProvider { map }
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
        let langid = req.get_langid()?;
        let data = self
            .map
            .get(langid)
            .map(|s| HelloWorldV1 { message: s.clone() })
            .ok_or_else(|| DataError::UnavailableResourceOptions(req.clone()))?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: Some(langid.clone()),
            },
            payload: Some(Cow::Owned(data)),
        })
    }
}

impl_erased!(HelloWorldProvider<'static>, 'd);

impl<'d> IterableDataProvider<'d> for HelloWorldProvider<'d> {
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

/// Adds entries to a HelloWorldProvider as a DataExporter
impl DataExporter for HelloWorldProvider<'static> {
    fn put_payload(
        &mut self,
        req: &DataRequest,
        payload: &dyn ErasedDataStruct,
    ) -> Result<(), Box<dyn std::error::Error>> {
        req.resource_path.key.match_key(key::HELLO_WORLD_V1)?;
        let langid = req.get_langid()?;
        let data: &HelloWorldV1 = payload.downcast_ref()?;
        self.map.insert(langid.clone(), data.message.clone());
        Ok(())
    }

    fn include_resource_options(&self, _resc_options: &ResourceOptions) -> bool {
        true
    }
}

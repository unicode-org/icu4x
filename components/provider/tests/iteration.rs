// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::erased::ErasedDataStruct;
use icu_provider::iter::DataExporter;
use icu_provider::prelude::*;
use icu_provider::structs::icu4x::{key::HELLO_V1, HelloV1};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Default)]
struct HelloWorldProvider<'s> {
    map: HashMap<LanguageIdentifier, Cow<'s, str>>,
}

impl<'d, 's> DataProvider<'d, HelloV1<'s>> for HelloWorldProvider<'s>
where
    's: 'd,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, HelloV1<'s>>, DataError> {
        req.resource_path.key.match_key(HELLO_V1)?;
        let langid = req.get_langid()?;
        let data = self
            .map
            .get(langid)
            .map(|s| HelloV1 { hello: s.clone() })
            .ok_or_else(|| DataError::UnavailableResourceOptions(req.clone()))?;
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: Some(langid.clone()),
            },
            payload: Some(Cow::Owned(data)),
        })
    }
}

icu_provider::impl_erased!(HelloWorldProvider<'static>, 'd);

impl<'d> IterableDataProvider<'d> for HelloWorldProvider<'d> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        resc_key.match_key(HELLO_V1)?;
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
        req.resource_path.key.match_key(HELLO_V1)?;
        let langid = req.get_langid()?;
        let data: &HelloV1 = payload.downcast_ref()?;
        self.map.insert(langid.clone(), data.hello.clone());
        Ok(())
    }

    fn include_resource_options(&self, _resc_options: &ResourceOptions) -> bool {
        true
    }
}

fn get_sample_hello_world_provider() -> HelloWorldProvider<'static> {
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

#[test]
fn test_supported_langids() {
    let provider = get_sample_hello_world_provider();
    let mut supported_langids: Vec<LanguageIdentifier> = provider
        .supported_options_for_key(&HELLO_V1)
        .unwrap()
        .map(|resc_options| resc_options.langid.unwrap())
        .collect();
    supported_langids.sort();

    assert_eq!(
        supported_langids,
        vec![
            langid!("bn"),
            langid!("cs"),
            langid!("de"),
            langid!("el"),
            langid!("en"),
            langid!("eo"),
            langid!("fa"),
            langid!("fi"),
            langid!("is"),
            langid!("ja"),
            langid!("la"),
            langid!("ro"),
            langid!("ru"),
            langid!("vi"),
            langid!("zh")
        ]
    );
}

#[test]
fn test_export() {
    let source_provider = get_sample_hello_world_provider();
    let mut dest_provider = HelloWorldProvider::default();

    dest_provider
        .put_key_from_provider(&HELLO_V1, &source_provider)
        .unwrap();

    assert_eq!(source_provider, dest_provider);
}

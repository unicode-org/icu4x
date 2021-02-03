// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::hello_world::*;
use icu_provider::prelude::*;

#[test]
fn test_supported_langids() {
    let provider = HelloWorldProvider::new_with_placeholder_data();
    let mut supported_langids: Vec<LanguageIdentifier> = provider
        .supported_options_for_key(&key::HELLO_WORLD_V1)
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

#[cfg(feature = "erased")]
#[test]
fn test_export() {
    use icu_provider::iter::DataExporter;

    let source_provider = HelloWorldProvider::new_with_placeholder_data();
    let mut dest_provider = HelloWorldProvider::default();

    dest_provider
        .put_key_from_provider(&key::HELLO_WORLD_V1, &source_provider)
        .unwrap();

    assert_eq!(source_provider, dest_provider);
}

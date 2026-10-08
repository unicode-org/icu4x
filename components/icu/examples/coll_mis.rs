// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();

use icu_benchmark_macros::println;

use icu::collator::Collator;
use icu::locale::locale;
use icu_collator::options::CollatorOptions;
use icu_collator::provider::*;
use icu_collator::CollatorPreferences;
use icu_normalizer::provider::*;
use icu_provider_adapters::fork::ForkByMarkerProvider;
use icu_provider_blob::BlobDataProvider;

fn main() {
    let mis_blob = std::fs::read("mis_collation.postcard").unwrap();
    let root_blob = std::fs::read("root_collation.postcard").unwrap();

    let blob_provider = ForkByMarkerProvider::new(
        BlobDataProvider::try_new_from_blob(mis_blob.clone().into()).unwrap(),
        BlobDataProvider::try_new_from_blob(root_blob.into()).unwrap(),
    );

    struct MyProvider {
        blob: BlobDataProvider,
        collator: icu::collator::provider::Baked,
        normalizer: icu::normalizer::provider::Baked,
    }

    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationMetadataV1,
        &self.blob.as_deserializing()
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationTailoringV1,
        &self.blob.as_deserializing()
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationDiacriticsV1,
        &self.collator
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationJamoV1,
        &self.collator
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationReorderingV1,
        &self.collator
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationRootV1,
        &self.collator
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        CollationSpecialPrimariesV1,
        &self.collator
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        NormalizerNfdDataV1,
        &self.normalizer
    );
    icu_provider_adapters::delegate::data_provider_to_field!(
        MyProvider,
        NormalizerNfdTablesV1,
        &self.normalizer
    );

    let prefs = CollatorPreferences::from(locale!("mis"));
    let options = CollatorOptions::default();

    let blob_or_baked_provider = MyProvider {
        blob: BlobDataProvider::try_new_from_blob(mis_blob.into()).unwrap(),
        collator: icu::collator::provider::Baked,
        normalizer: icu::normalizer::provider::Baked,
    };

    let default_collator = Collator::try_new(prefs, options).unwrap();
    let custom_blob_collator =
        Collator::try_new_with_buffer_provider(&blob_provider, prefs, options).unwrap();
    let custom_baked_collator =
        Collator::try_new_unstable(&blob_or_baked_provider, prefs, options).unwrap();

    println!("Default: {:?}", default_collator.compare("ı", "I"));
    println!(
        "Custom All Blob: {:?}",
        custom_blob_collator.as_borrowed().compare("ı", "I")
    );
    println!(
        "Custom Blob Baked Hybrid: {:?}",
        custom_baked_collator.as_borrowed().compare("ı", "I")
    );
}

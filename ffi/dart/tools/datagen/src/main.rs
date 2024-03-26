// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Precompiled datagen

use clap::Parser;
use icu_datagen::prelude::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use icu_provider_blob::export::BlobExporter;
use icu_provider_blob::BlobDataProvider;
use serde::Deserialize;
use std::path::PathBuf;
use yoke::{trait_hack::YokeTraitHack, Yokeable};

#[derive(Parser)]
#[command(name = "icu4x-datagen-fast")]
#[command(author = "The ICU4X Project Developers", version = option_env!("CARGO_PKG_VERSION"))]
#[command(about = format!("Learn more at: https://docs.rs/icu_datagen/{}", option_env!("CARGO_PKG_VERSION").unwrap_or("")), long_about = None)]
pub struct Cli {
    #[arg(long, short, num_args = 1..)]
    #[arg(
        help = "Include these resource keys in the output. Accepts multiple arguments.\n\
                  Set to 'all' for all keys, or 'none' for no keys."
    )]
    keys: Vec<String>,

    #[arg(long, short, num_args = 0..)]
    #[arg(
        help = "Include this locale in the output. Accepts multiple arguments. \
                  Set to 'full' or 'modern' for the respective CLDR locale sets, 'none' for no locales, \
                  or 'recommended' for the recommended set of locales."
    )]
    locales: Vec<String>,

    #[arg(long = "out", short, value_name = "PATH")]
    #[arg(help = "Path to output file. Must be non-existent.")]
    output: PathBuf,
}

struct ReexportableBlobDataProvider(BlobDataProvider);

impl<M: KeyedDataMarker> DataProvider<M> for ReexportableBlobDataProvider
where
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        self.0.as_deserializing().load(req)
    }
}

impl<M: KeyedDataMarker> IterableDataProvider<M> for ReexportableBlobDataProvider
where
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        self.0.supported_locales_for_key(M::KEY)
    }
}

icu_datagen::make_exportable_provider!(ReexportableBlobDataProvider);

fn main() -> eyre::Result<()> {
    let matches = Cli::parse();

    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let keys = matches
        .keys
        .iter()
        .map(|k| icu_datagen::key(k).ok_or(eyre::eyre!(k.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    let locales = matches
        .locales
        .iter()
        .map(|l| {
            l.parse::<LanguageIdentifier>()
                .map(LocaleFamily::with_descendants)
        })
        .collect::<Result<Vec<_>, _>>()?;

    DatagenDriver::new()
        .with_keys(keys)
        .with_locales_and_fallback(locales, Default::default())
        .export(
            &ReexportableBlobDataProvider(BlobDataProvider::try_new_from_static_blob(
                include_bytes!(concat!(core::env!("OUT_DIR"), "/all.blob")),
            )?),
            BlobExporter::new_with_sink(Box::new(std::fs::File::create(&matches.output)?)),
        )?;

    Ok(())
}

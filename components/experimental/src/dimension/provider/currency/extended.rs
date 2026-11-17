// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_plurals::provider::PluralElementsPackedCow;

icu_provider::data_marker!(
    /// Provides plural display names for currencies.
    ///
    /// This data is keyed by both locale and currency, with a currency code marker attribute.
    CurrencyExtendedDataV1,
    CurrencyExtendedData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "currency",
);

/// Plural currency display names data struct.
pub type CurrencyExtendedData<'data> = PluralElementsPackedCow<'data, str>;

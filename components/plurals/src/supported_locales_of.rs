// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

/// Returns which locales out of the given list are supported by this crate.
///
/// # Examples
///
/// ```
/// use icu::locale::locale;
/// use icu_provider::DataLocale;
///
/// let supported_locales = icu::plurals::supported_locales_of([
///     DataLocale::from(locale!("ru")), // Russian is supported and has its own rules
///     DataLocale::from(locale!("sr-RS")), // sr-RS inherits from sr which has its own rules
///     DataLocale::from(locale!("ja")), // Japanese is supported but inherits from root
///     DataLocale::from(locale!("tlh")), // Klingon is not supported
/// ]).unwrap();
///
/// assert_eq!(
///     supported_locales,
///     [
///         DataLocale::from(locale!("ru")),
///         DataLocale::from(locale!("sr-RS")),
///         DataLocale::from(locale!("ja")),
///     ]
/// );
/// ```
pub fn supported_locales_of(
    locales: impl IntoIterator<Item = DataLocale>,
) -> Result<Vec<DataLocale>, DataError> {
    let mut result = Vec::new();
    for locale in locales {
        let mut metadata = DataRequestMetadata::default();
        metadata.silent = true;
        let response = crate::provider::Baked.dry_load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            metadata,
        });
        println!("{locale:?} => {:?}", response);
        match response {
            Ok(DataResponseMetadata { locale: None, .. }) => {
                // locale: None means exact match
                result.push(locale)
            }
            Ok(DataResponseMetadata { locale: Some(resolved_locale), .. }) if !resolved_locale.is_default() => {
                result.push(locale)
            }
            _ => ()
        }
    }
    Ok(result)
}

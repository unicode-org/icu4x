// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use std::collections::HashSet;
use std::path::Path;

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        fn markers_for_bin_inner(bytes: &[u8]) -> Result<HashSet<DataMarkerInfo>, DataError> {
            use std::sync::OnceLock;
            use crate as icu;
            static LOOKUP: OnceLock<std::collections::HashMap<&'static str, Option<DataMarkerInfo>>> = OnceLock::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    ("core/helloworld@1", Some(icu_provider::hello_world::HelloWorldV1Marker::INFO)),
                    $(
                        ($path, Some(<$marker>::INFO)),
                    )+
                    $(
                        #[cfg(feature = "experimental")]
                        ($epath, Some(<$emarker>::INFO)),
                        #[cfg(not(feature = "experimental"))]
                        ($epath, None),
                    )+

                ]
                .into_iter()
                .collect()
            });

            use memchr::memmem::*;

            const LEADING_TAG: &[u8] = icu_provider::leading_tag!().as_bytes();
            const TRAILING_TAG: &[u8] = icu_provider::trailing_tag!().as_bytes();

            let trailing_tag = Finder::new(TRAILING_TAG);

            find_iter(bytes, LEADING_TAG)
                .map(|tag_position| tag_position + LEADING_TAG.len())
                .filter_map(|marker_start| bytes.get(marker_start..))
                .filter_map(move |marker_fragment| {
                    trailing_tag
                        .find(marker_fragment)
                        .and_then(|end| marker_fragment.get(..end))
                })
                .map(std::str::from_utf8)
                .filter_map(Result::ok)
                .filter_map(|p| {
                    match lookup.get(p) {
                        Some(Some(marker)) => Some(Ok(*marker)),
                        Some(None) => Some(Err(DataError::custom("This marker requires the `experimental` Cargo feature").with_display_context(p))),
                        None => None,
                    }
                })
                .collect::<Result<_, _>>()
        }
    }
}
icu_registry::registry!(cb);

/// Parses a compiled binary and returns a list of [`DataMarkerInfo`]s that it uses *at runtime*.
///
/// This function is intended to be used for binaries that use `AnyProvider` or `BufferProvider`,
/// for which the compiler cannot remove unused data. Using this function on a binary that only
/// uses compiled data (through the `compiled_data` feature or manual baked data) will not return
/// any markers, as the markers only exist in the type system.
///
/// # Example
///
/// ```no_run
/// # use icu_provider::DataMarker;
/// # use std::path::Path;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(
///     icu::markers_for_bin(Path::new("target/release/my-app"))?,
///     std::collections::HashSet::from_iter([
///         icu::list::provider::AndListV2Marker::INFO,
///         icu::list::provider::OrListV2Marker::INFO,
///     ]),
/// );
/// # Ok(())
/// # }
/// ```
pub fn markers_for_bin(path: &Path) -> Result<HashSet<DataMarkerInfo>, DataError> {
    markers_for_bin_inner(&std::fs::read(path)?)
}

#[test]
fn test_markers_for_bin() {
    assert_eq!(
        markers_for_bin_inner(include_bytes!("../tests/data/tutorial_buffer.wasm")),
        Ok(HashSet::from_iter([
            crate::datetime::provider::calendar::GregorianDateLengthsV1Marker::INFO,
            crate::datetime::provider::calendar::GregorianDateSymbolsV1Marker::INFO,
            crate::datetime::provider::calendar::TimeLengthsV1Marker::INFO,
            crate::datetime::provider::calendar::TimeSymbolsV1Marker::INFO,
            crate::calendar::provider::WeekDataV1Marker::INFO,
            crate::decimal::provider::DecimalSymbolsV1Marker::INFO,
            crate::plurals::provider::OrdinalV1Marker::INFO,
        ]))
    );
}

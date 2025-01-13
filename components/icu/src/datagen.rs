// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use icu_provider::marker::DataMarkerPath;
use std::collections::HashSet;
use std::path::Path;

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
        fn markers_for_bin_inner(bytes: &[u8]) -> Result<HashSet<DataMarkerInfo>, DataError> {
            use std::sync::OnceLock;
            use crate as icu;
            static LOOKUP: OnceLock<std::collections::HashMap<[u8; 4], Result<DataMarkerInfo, &'static str>>> = OnceLock::new();
            let lookup = LOOKUP.get_or_init(|| {
                [
                    (icu_provider::marker::data_marker_path!("core/helloworld@1").hashed().to_bytes(), Ok(icu_provider::hello_world::HelloWorldV1Marker::INFO)),
                    $(
                        (icu_provider::marker::data_marker_path!($path).hashed().to_bytes(), Ok(<$marker>::INFO)),
                    )+
                    $(
                        #[cfg(feature = "experimental")]
                        (icu_provider::marker::data_marker_path!($epath).hashed().to_bytes(), Ok(<$emarker>::INFO)),
                        #[cfg(not(feature = "experimental"))]
                        (icu_provider::marker::data_marker_path!($epath).hashed().to_bytes(), Err(stringify!($epath))),
                    )+

                ]
                .into_iter()
                .collect()
            });

            use memchr::memmem::*;


            find_iter(bytes, &DataMarkerPath::LEADING_TAG)
                .map(|tag_position| tag_position + DataMarkerPath::LEADING_TAG.len())
                .filter_map(|marker_start| bytes.get(marker_start..marker_start+4))
                .filter_map(|p| {
                    match lookup.get(p) {
                        Some(Ok(marker)) => Some(Ok(*marker)),
                        Some(Err(path)) => Some(Err(DataError::custom("This marker requires the `experimental` Cargo feature").with_display_context(path))),
                        None => None,
                    }
                })
                .collect::<Result<_, _>>()
        }
    }
}
icu_provider_registry::registry!(cb);

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
        markers_for_bin_inner(include_bytes!("../tests/data/tutorial_buffer.wasm")).unwrap(),
        [
            crate::datetime::provider::neo::DayPeriodNamesV1Marker::INFO,
            crate::datetime::provider::neo::GregorianMonthNamesV1Marker::INFO,
            crate::datetime::provider::neo::GregorianYearNamesV1Marker::INFO,
            crate::datetime::provider::neo::GluePatternV1Marker::INFO,
            crate::datetime::provider::GregorianDateNeoSkeletonPatternsV1Marker::INFO,
            crate::datetime::provider::TimeNeoSkeletonPatternsV1Marker::INFO,
            crate::decimal::provider::DecimalDigitsV1Marker::INFO,
            crate::decimal::provider::DecimalSymbolsV2Marker::INFO,
        ]
        .into_iter()
        .collect()
    );
}

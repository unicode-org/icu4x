// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use icu_provider::prelude::*;

macro_rules! cb {
    ($($marker:path = $path:literal,)+ #[experimental] $($emarker:path = $epath:literal,)+) => {
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
        ///     icu::markers_for_bin(&std::fs::read(Path::new("target/release/my-app"))?)?,
        ///     std::collections::BTreeSet::from_iter([
        ///         icu::list::provider::AndListV2Marker::INFO,
        ///         icu::list::provider::OrListV2Marker::INFO,
        ///     ]),
        /// );
        /// # Ok(())
        /// # }
        /// ```
        pub fn markers_for_bin(bytes: &[u8]) -> Result<BTreeSet<DataMarkerInfo>, DataError> {
            use crate as icu;
            let lookup =
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
                .collect::<BTreeMap<_,_>>();

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
                .map(core::str::from_utf8)
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
icu_provider_registry::registry!(cb);

#[test]
fn test_markers_for_bin() {
    let hashset = markers_for_bin(include_bytes!("../tests/data/tutorial_buffer.wasm")).unwrap();
    let mut sorted = hashset.into_iter().collect::<Vec<_>>();
    sorted.sort();
    assert_eq!(
        sorted,
        &[
            crate::datetime::provider::neo::DayPeriodNamesV1Marker::INFO,
            crate::datetime::provider::neo::GregorianMonthNamesV1Marker::INFO,
            crate::datetime::provider::neo::GregorianYearNamesV1Marker::INFO,
            crate::datetime::provider::neo::GluePatternV1Marker::INFO,
            crate::datetime::provider::GregorianDateNeoSkeletonPatternsV1Marker::INFO,
            crate::datetime::provider::TimeNeoSkeletonPatternsV1Marker::INFO,
            crate::decimal::provider::DecimalSymbolsV2Marker::INFO,
        ]
    );
}

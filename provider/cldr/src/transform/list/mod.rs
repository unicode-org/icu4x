// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, get_langid_subdirectory, open_reader};
use crate::CldrPaths;
use icu_list::provider::*;
use icu_locid_macros::langid;
use icu_provider::iter::IterableResourceProvider;
use icu_provider::prelude::*;
use std::path::PathBuf;

/// A data provider reading from CLDR JSON list rule files.
#[derive(Debug)]
pub struct ListProvider {
    cldr_misc: PathBuf,
    uprops_root: PathBuf,
}

impl ListProvider {
    pub fn try_from(cldr_paths: &dyn CldrPaths, uprops_root: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            cldr_misc: cldr_paths.cldr_misc()?,
            uprops_root,
        })
    }
}

impl<M: ResourceMarker<Yokeable = ListFormatterPatternsV1<'static>>> ResourceProvider<M>
    for ListProvider
{
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let langid = req
            .get_langid()
            .ok_or_else(|| DataErrorKind::NeedsLocale.with_req(M::KEY, req))?;

        let resource: cldr_serde::list_patterns::Resource = {
            let path = get_langid_subdirectory(&self.cldr_misc.join("main"), langid)?
                .ok_or_else(|| DataErrorKind::MissingLocale.with_req(M::KEY, req))?
                .join("listPatterns.json");
            serde_json::from_reader(open_reader(&path)?).map_err(|e| Error::Json(e, Some(path)))?
        };

        let data = &resource
            .main
            .0
            .get(langid)
            .expect("CLDR file contains the expected language")
            .list_patterns;

        let (wide, short, narrow) = match M::KEY {
            AndListV1Marker::KEY => (&data.standard, &data.standard_short, &data.standard_narrow),
            OrListV1Marker::KEY => (&data.or, &data.or_short, &data.or_narrow),
            UnitListV1Marker::KEY => (&data.unit, &data.unit_short, &data.unit_narrow),
            _ => {
                return Err(
                    DataError::custom("Unknown key for ListFormatterPatternsV1").with_key(M::KEY)
                )
            }
        };

        let mut patterns = ListFormatterPatternsV1::try_new([
            &wide.start,
            &wide.middle,
            &wide.end,
            &wide.pair,
            &short.start,
            &short.middle,
            &short.end,
            &short.pair,
            &narrow.start,
            &narrow.middle,
            &narrow.end,
            &narrow.pair,
        ])
        .map_err(|e| e.with_req(M::KEY, req))?;

        if langid.language == langid!("es").language {
            match M::KEY {
                // Replace " y " with " e " before /i/ sounds.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=important.%20For%20example%3A-,Spanish,AND,-Use%20%E2%80%98e%E2%80%99%20instead
                AndListV1Marker::KEY | UnitListV1Marker::KEY => patterns
                    .make_conditional(
                        "{0} y {1}",
                        // Starts with i or (hi but not hia/hie)
                        "i|hi([^ae]|$)",
                        "{0} e {1}",
                    )
                    .expect("Valid regex and pattern"),
                // Replace " o " with " u " before /o/ sound.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=agua%20e%20hielo-,OR,-Use%20%E2%80%98u%E2%80%99%20instead
                OrListV1Marker::KEY => patterns
                    .make_conditional(
                        "{0} o {1}",
                        // Starts with o, ho, 8 (including 80, 800, ...), or 11 either alone or followed
                        // by thousand groups and/or decimals (excluding e.g. 110, 1100, ...)
                        r"o|ho|8|(11(\.?\d\d\d)*(,\d*)?([^\.,\d]|$))",
                        "{0} u {1}",
                    )
                    .expect("Valid regex and pattern"),
                _ => unreachable!(),
            }
        }

        if langid.language == langid!("he").language {
            // Add dashes between ו and non-Hebrew characters
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=is%20not%20mute.-,Hebrew,AND,-Use%20%E2%80%98%2D%D7%95%E2%80%99%20instead
            patterns
                .make_conditional(
                    "{0} \u{05D5}{1}", // ״{0} ו {1}״
                    // Starts with a non-Hebrew letter
                    &format!(
                        "[^{}]",
                        icu_properties::sets::get_for_script(
                            &icu_provider_uprops::EnumeratedPropertyUnicodeSetDataProvider::try_new(
                                &self.uprops_root
                            )
                            .map_err(|e| DataError::custom("Properties data provider error")
                                .with_display_context(&e))?,
                            icu_properties::Script::Hebrew,
                        )
                        .map_err(|e| DataError::custom("Could not find Hebrew script set")
                            .with_display_context(&e))?
                        .get()
                        .inv_list
                        .iter_ranges()
                        .map(|range| format!(r#"\u{:04x}-\u{:04x}"#, range.start(), range.end()))
                        .fold(String::new(), |a, b| a + &b)
                    ),
                    "{0} \u{05D5}-{1}", // ״{0} ו- {1}״
                )
                .unwrap();
        }

        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(patterns)),
        })
    }
}

icu_provider::impl_dyn_provider!(
    ListProvider,
    [AndListV1Marker, OrListV1Marker, UnitListV1Marker,],
    SERDE_SE,
    impl ConvertData
);

impl<M: ResourceMarker<Yokeable = ListFormatterPatternsV1<'static>>> IterableResourceProvider<M>
    for ListProvider
{
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(
            get_langid_subdirectories(&self.cldr_misc.join("main"))?
                // ur-IN has a buggy pattern ("{1}, {0}") which violates
                // our invariant that {0} is at index 0 (and rotates the output).
                // ml has middle and start patterns with suffixes.
                // See https://github.com/unicode-org/icu4x/issues/1282
                .filter(|l| l != &langid!("ur-IN") && l != &langid!("ml"))
                .map(Into::<ResourceOptions>::into),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_list::{ListFormatter, ListStyle};
    use icu_locid_macros::langid;
    use writeable::assert_writeable_eq;

    macro_rules! test {
        ($langid:literal, $type:ident, $(($input:expr, $output:literal),)+) => {
            let cldr_paths = crate::cldr_paths::for_test();
            let provider = ListProvider::try_from(
                &cldr_paths as &dyn CldrPaths, icu_testdata::paths::uprops_toml_root()).unwrap();
            let f = ListFormatter::$type(langid!($langid), &provider, ListStyle::Wide).unwrap();
            $(
                assert_writeable_eq!(f.format($input.iter()), $output);
            )+
        };
    }

    #[test]
    fn test_basic() {
        test!("fr", try_new_or, (["A", "B"], "A ou B"),);
    }

    #[test]
    fn test_spanish() {
        test!(
            "es",
            try_new_and,
            (["x", "Mallorca"], "x y Mallorca"),
            (["x", "Ibiza"], "x e Ibiza"),
            (["x", "Hidalgo"], "x e Hidalgo"),
            (["x", "Hierva"], "x y Hierva"),
        );

        test!(
            "es",
            try_new_or,
            (["x", "Ibiza"], "x o Ibiza"),
            (["x", "Okinawa"], "x u Okinawa"),
            (["x", "8 más"], "x u 8 más"),
            (["x", "8"], "x u 8"),
            (["x", "87 más"], "x u 87 más"),
            (["x", "87"], "x u 87"),
            (["x", "11 más"], "x u 11 más"),
            (["x", "11"], "x u 11"),
            (["x", "110 más"], "x o 110 más"),
            (["x", "110"], "x o 110"),
            (["x", "11.000 más"], "x u 11.000 más"),
            (["x", "11.000"], "x u 11.000"),
            (["x", "11.000,92 más"], "x u 11.000,92 más"),
            (["x", "11.000,92"], "x u 11.000,92"),
        );

        test!("es-AR", try_new_and, (["x", "Ibiza"], "x e Ibiza"),);
    }

    #[test]
    fn test_hebrew() {
        test!(
            "he",
            try_new_and,
            (["x", "יפו"], "x ויפו"),
            (["x", "Ibiza"], "x ו-Ibiza"),
        );
    }
}

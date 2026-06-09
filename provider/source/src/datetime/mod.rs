// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde;
use icu::calendar::AnyCalendarKind;
use icu::datetime::provider::fields::{Field, components};
use icu::datetime::provider::packed_pattern::{
    GenericLengthElements, GenericPackedPatternsBuilder,
};
use icu::datetime::provider::pattern::CoarseHourCycle;
use icu::datetime::provider::skeleton::SkeletonError;
use icu::datetime::provider::skeleton::reference::Skeleton;
use icu_locale_core::preferences::extensions::unicode::keywords::HourCycle;

use icu::datetime::pattern::FixedCalendarDateTimeNames;
use icu::datetime::provider::packed_pattern::GenericPackedPatterns;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::ule::VarULE;

mod available_formats;
mod day_periods;
mod names;
mod semantic_skeletons;
use semantic_skeletons::Trio;
mod week_data;

/// These are the calendars that datetime needs names for. They are roughly the
/// CLDR calendars, with the Hijri calendars merged.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum DatagenCalendar {
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopic,
    Gregorian,
    Hebrew,
    Indian,
    Hijri,
    Japanese,
    Persian,
    Roc,
}

impl DatagenCalendar {
    pub(crate) fn cldr_name(self) -> &'static str {
        use DatagenCalendar::*;
        match self {
            Buddhist => "buddhist",
            Chinese => "chinese",
            Coptic => "coptic",
            Dangi => "dangi",
            Ethiopic => "ethiopic",
            Gregorian => "gregorian",
            Hebrew => "hebrew",
            Indian => "indian",
            Hijri => "islamic",
            Japanese => "japanese",
            Persian => "persian",
            Roc => "roc",
        }
    }

    pub(crate) fn from_cldr_name(s: &str) -> Self {
        use DatagenCalendar::*;
        match s {
            "buddhist" => Buddhist,
            "chinese" => Chinese,
            "coptic" => Coptic,
            "dangi" => Dangi,
            "ethiopic" | "ethiopic-amete-alem" => Ethiopic,
            "gregorian" => Gregorian,
            "hebrew" => Hebrew,
            "indian" => Indian,
            "islamic" | "islamic-civil" | "islamic-umalqura" | "islamic-rgsa" | "islamic-tbla" => {
                Hijri
            }
            "japanese" => Japanese,
            "persian" => Persian,
            "roc" => Roc,
            c => panic!("{c}"),
        }
    }

    pub(crate) fn canonical_any_calendar_kind(self) -> AnyCalendarKind {
        use DatagenCalendar::*;
        match self {
            Buddhist => AnyCalendarKind::Buddhist,
            Chinese => AnyCalendarKind::Chinese,
            Coptic => AnyCalendarKind::Coptic,
            Dangi => AnyCalendarKind::Dangi,
            Ethiopic => AnyCalendarKind::Ethiopian, // also covers EthiopianAmeteAlem
            Gregorian => AnyCalendarKind::Gregorian,
            Hebrew => AnyCalendarKind::Hebrew,
            Indian => AnyCalendarKind::Indian,
            Hijri => AnyCalendarKind::HijriUmmAlQura, // also covers HijriTabular*, HijriSimulatedMecca
            Japanese => AnyCalendarKind::Japanese,
            Persian => AnyCalendarKind::Persian,
            Roc => AnyCalendarKind::Roc,
        }
    }
}

impl SourceDataProvider {
    pub(crate) fn get_dates_resource(
        &self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
    ) -> Result<&cldr_serde::ca::Dates, DataError> {
        let cldr_cal = calendar
            .map(DatagenCalendar::cldr_name)
            .unwrap_or("generic");

        let resource = self
            .cldr()?
            .dates(cldr_cal)
            .read_and_parse::<cldr_serde::ca::Resource>(locale, &format!("ca-{cldr_cal}.json"))?
            .main
            .value
            .dates
            .calendars
            .get(cldr_cal)
            .expect("CLDR file contains the expected calendar");

        // load other ca-islamic-*.json files and verify that they match
        if calendar == Some(DatagenCalendar::Hijri) {
            for variant in &["civil", "rgsa", "tbla", "umalqura"] {
                let variant_resource = self
                    .cldr()?
                    .dates(cldr_cal)
                    .read_and_parse::<cldr_serde::ca::Resource>(
                        locale,
                        &format!("ca-islamic-{variant}.json"),
                    )?
                    .main
                    .value
                    .dates
                    .calendars
                    .get(&format!("islamic-{variant}"))
                    .expect("CLDR file contains the expected calendar");

                if variant_resource != resource {
                    log::warn!("islamic/islamic-{variant} data mismatch: {locale}");
                }
            }
        }

        // load ca-ethiopic-amete-alem.json and verify that it matches
        if calendar == Some(DatagenCalendar::Ethiopic) {
            let alem = self
                .cldr()?
                .dates(cldr_cal)
                .read_and_parse::<cldr_serde::ca::Resource>(locale, "ca-ethiopic-amete-alem.json")?
                .main
                .value
                .dates
                .calendars
                .get("ethiopic-amete-alem")
                .expect("CLDR file contains the expected calendar");

            if (
                &alem.cyclic_name_sets,
                &alem.date_formats,
                &alem.date_skeletons,
                &alem.datetime_formats,
                &alem.datetime_formats_at_time,
                &alem.day_periods,
                &alem.days,
                &alem
                    .eras
                    .as_ref()
                    .map(|e| (e.abbr.get("0"), e.names.get("0"), e.narrow.get("0"))),
                &alem.month_patterns,
                &alem.months,
                &alem.time_formats,
                &alem.time_skeletons,
            ) != (
                &resource.cyclic_name_sets,
                &resource.date_formats,
                &resource.date_skeletons,
                &resource.datetime_formats,
                &resource.datetime_formats_at_time,
                &resource.day_periods,
                &resource.days,
                &resource
                    .eras
                    .as_ref()
                    .map(|e| (e.abbr.get("0"), e.names.get("0"), e.narrow.get("0"))),
                &resource.month_patterns,
                &resource.months,
                &resource.time_formats,
                &resource.time_skeletons,
            ) {
                log::warn!("ethiopic/ethiopic-amete-alem data mismatch: {locale}");
            }
        }

        Ok(resource)
    }
}

/// Iterates over all supported locales for a given calendar and generates
/// `DataIdentifierCow` keys for all combinations of the provided fieldset attributes.
///
/// This is a shared helper used by both standard and range skeleton providers to
/// generate the set of supported locales they can serve.
///
/// # Arguments
/// * `provider` - The source data provider to load CLDR data from.
/// * `calendar` - The calendar to load locales for (e.g., Gregorian, Buddhist). If `None`, uses "generic".
/// * `fieldset_attributes` - A list of slices of data marker attributes to combine with each locale.
pub(crate) fn iter_skeleton_supported_locales(
    provider: &SourceDataProvider,
    calendar: Option<DatagenCalendar>,
    fieldset_attributes: &[&[&'static DataMarkerAttributes]],
) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
    let cldr_cal = calendar
        .map(DatagenCalendar::cldr_name)
        .unwrap_or("generic");
    Ok(provider
        .cldr()?
        .dates(cldr_cal)
        .list_locales()?
        .flat_map(|locale| {
            fieldset_attributes
                .iter()
                .flat_map(|list| list.iter())
                .map(move |attrs| DataIdentifierCow::from_borrowed_and_owned(attrs, locale))
        })
        .collect())
}

/// Parses a collection of raw CLDR skeleton strings and their associated values into a `BTreeMap`.
///
/// Skeletons that fail to parse via [`Skeleton::try_from`] are silently ignored.
/// If a duplicate skeleton is encountered after normalization (e.g. due to 'E' vs 'c' forms),
/// it will overwrite the previous value and log a warning.
///
/// # Example Input
/// This function is designed to parse maps like:
/// ```json
/// {
///   "yMd": "y/M/d",
///   "yMMMMd": "y MMMM d",
///   "invalid_skeleton": "pattern"
/// }
/// ```
/// For `"yMd"`, it parses it into a `Skeleton` and calls `map_fn` with it and `"y/M/d"`.
/// `"invalid_skeleton"` will be skipped.
///
/// # Arguments
/// * `raw_patterns` - An iterator over `(skeleton_string, value)` pairs.
/// * `map_fn` - A closure that maps the parsed `Skeleton` and the raw value into the desired result type `R`.
pub(crate) fn parse_cldr_skeletons<'a, K, V: 'a, R, I, F>(
    raw_patterns: I,
    mut map_fn: F,
) -> BTreeMap<Skeleton, R>
where
    K: AsRef<str> + 'a,
    I: IntoIterator<Item = (&'a K, &'a V)>,
    F: FnMut(&Skeleton, &'a V) -> Option<R>,
{
    let mut result = BTreeMap::new();
    for (skeleton_str, value) in raw_patterns {
        let skeleton = match Skeleton::try_from(skeleton_str.as_ref()) {
            Ok(s) => s,
            Err(SkeletonError::SymbolUnimplemented(_)) => continue,
            Err(SkeletonError::SkeletonHasVariant) => continue,
            Err(err) => panic!(
                "Unexpected skeleton error while parsing skeleton {} {err}",
                skeleton_str.as_ref()
            ),
        };
        if let Some(mapped) = map_fn(&skeleton, value) {
            // CLDR seems to be moving away from `c` in `availableFormats` skeleta.
            // We don't expect to see both `E` and `c` for the same skeleton, but if we do,
            // we warn and prefer the one that appeared later in the map (arbitrary).
            if let Some(_old) = result.insert(skeleton.clone(), mapped) {
                log::warn!(
                    "Duplicate skeleton found after normalization: {}. This might happen if CLDR has both 'E' and 'c' forms.",
                    skeleton
                );
            }
        }
    }
    result
}

/// Transposes a length-major structure of pattern trios into a variant-major builder structure,
/// performing fallback using `to_builder_item` to avoid cloning.
///
/// Converts `GenericLengthElements<Trio<T::FinalItem>>` (patterns grouped by length, containing standard/variants)
/// into `GenericPackedPatternsBuilder<T::BuilderItem<'b>>` (patterns grouped by standard/variants, containing lengths).
pub(crate) fn transpose_with_fallback<'b, T: PackedPatternItem>(
    group: &'b GenericLengthElements<Trio<T::FinalItem>>,
) -> GenericPackedPatternsBuilder<T::BuilderItem<'b>> {
    let variant0 = if group.long.variant0.is_some()
        || group.medium.variant0.is_some()
        || group.short.variant0.is_some()
    {
        Some(GenericLengthElements {
            long: T::to_builder_item(group.long.variant0.as_ref().unwrap_or(&group.long.standard)),
            medium: T::to_builder_item(
                group
                    .medium
                    .variant0
                    .as_ref()
                    .unwrap_or(&group.medium.standard),
            ),
            short: T::to_builder_item(
                group
                    .short
                    .variant0
                    .as_ref()
                    .unwrap_or(&group.short.standard),
            ),
        })
    } else {
        None
    };
    let variant1 = if group.long.variant1.is_some()
        || group.medium.variant1.is_some()
        || group.short.variant1.is_some()
    {
        Some(GenericLengthElements {
            long: T::to_builder_item(group.long.variant1.as_ref().unwrap_or(&group.long.standard)),
            medium: T::to_builder_item(
                group
                    .medium
                    .variant1
                    .as_ref()
                    .unwrap_or(&group.medium.standard),
            ),
            short: T::to_builder_item(
                group
                    .short
                    .variant1
                    .as_ref()
                    .unwrap_or(&group.short.standard),
            ),
        })
    } else {
        None
    };
    GenericPackedPatternsBuilder {
        standard: GenericLengthElements {
            long: T::to_builder_item(&group.long.standard),
            medium: T::to_builder_item(&group.medium.standard),
            short: T::to_builder_item(&group.short.standard),
        },
        variant0,
        variant1,
    }
}

pub(crate) trait PackedPatternItem: Sized {
    /// The context required to match fields for this pattern item.
    type MatchFieldsContext;
    /// The final item type after finalization (e.g. stripping distance).
    type FinalItem: PartialEq;
    /// The borrowed item type used for building the packed structure.
    type BuilderItem<'a>: PartialEq
    where
        Self: 'a;
    /// The ULE type for packing.
    type Ule: VarULE + ?Sized + 'static;
    /// The distance type used to sort patterns by match quality.
    type MatchQuality: Ord;

    /// Attempts to find a matching pattern for the given fields in the context.
    ///
    /// Generates a reasonable fallback if it can't find one.
    fn match_fields(
        context: &Self::MatchFieldsContext,
        components_bag: &components::Bag,
        hour_cycle: HourCycle,
        fields: &[Field],
    ) -> Self;

    /// Returns the match quality (distance) of this pattern item.
    fn match_quality(&self) -> Self::MatchQuality;

    /// Finalizes the item (e.g., converts from a internal representation to the provider one).
    fn finalize_item(self) -> Self::FinalItem;

    /// Converts a reference to the final item into the borrowed builder item.
    fn to_builder_item<'b>(item: &'b Self::FinalItem) -> Self::BuilderItem<'b>;

    /// Builds the packed structure from the builder.
    fn build_packed<'b>(
        builder: GenericPackedPatternsBuilder<Self::BuilderItem<'b>>,
    ) -> GenericPackedPatterns<'static, Self::Ule>
    where
        Self: 'b;

    /// Applies numeric overrides to the pattern items.
    fn apply_numeric_overrides(&mut self, lp: &cldr_serde::ca::LengthPattern);

    /// Enforces consistent field lengths in the patterns.
    ///
    /// This is only needed for date patterns which have some bugs in CLDR. It
    /// can be removed when CLDR bugs are fixed, or replaced with a pure non-mutating warning.
    fn enforce_consistency(
        &mut self,
        names: &mut FixedCalendarDateTimeNames<()>,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
        attributes: &DataMarkerAttributes,
    );
}

/// A generic helper to resolve a pattern from a components bag, handling hour cycle and fallback.
pub(crate) fn select_pattern<T: PackedPatternItem>(
    context: &T::MatchFieldsContext,
    components_bag: components::Bag,
    preferred_hour_cycle: CoarseHourCycle,
) -> T {
    let default_hour_cycle = match preferred_hour_cycle {
        CoarseHourCycle::H11H12 => HourCycle::H12,
        CoarseHourCycle::H23 => HourCycle::H23,
    };
    let fields = components_bag.to_vec_fields(default_hour_cycle);
    T::match_fields(context, &components_bag, default_hour_cycle, &fields)
}

#[cfg(test)]
mod test {
    use super::*;
    use icu::{
        datetime::provider::skeleton::reference::Skeleton, locale::langid, plurals::PluralElements,
    };

    #[test]
    #[ignore] // TODO(#5643)
    fn test_datetime_skeletons() {
        let skeletons = SourceDataProvider::new_testing()
            .get_dates_resource(&langid!("fil").into(), Some(DatagenCalendar::Gregorian))
            .unwrap()
            .datetime_formats
            .available_formats
            .parse_skeletons();

        assert_eq!(
            Some(&PluralElements::new(
                "L".parse().expect("Failed to create pattern")
            )),
            skeletons.get(&Skeleton::try_from("M").expect("Failed to create Skeleton"))
        );

        let expected = PluralElements::new(
            "'linggo' w 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        )
        .with_one_value(Some(
            "'ika'-w 'linggo' 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        ));
        assert_eq!(
            Some(&expected),
            skeletons.get(&Skeleton::try_from("yw").expect("Failed to create Skeleton"))
        );
    }
}

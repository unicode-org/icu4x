// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for semantic skeletons and datetime names.

/// Helpers involving the data marker attributes used for date names.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[allow(missing_docs)]
pub mod marker_attrs {
    use icu_provider::DataMarkerAttributes;

    pub const NUMERIC: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("1");
    pub const ABBR: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("3");
    pub const NARROW: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("4");
    pub const WIDE: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("5");
    pub const SHORT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("6");
    pub const ABBR_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("3s");
    pub const NARROW_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("4s");
    pub const WIDE_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("5s");
    pub const SHORT_STANDALONE: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("6s");

    pub const PATTERN_LONG: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("l");
    pub const PATTERN_MEDIUM: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("m");
    pub const PATTERN_SHORT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("s");

    // TODO: The 12-hour and 24-hour DataMarkerAttributes can probably be deleted

    pub const PATTERN_LONG12: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("l12");
    pub const PATTERN_MEDIUM12: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("m12");
    pub const PATTERN_SHORT12: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("s12");

    pub const PATTERN_LONG24: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("l24");
    pub const PATTERN_MEDIUM24: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("m24");
    pub const PATTERN_SHORT24: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("s24");

    pub const PATTERN_LONG_DT: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ldt");
    pub const PATTERN_MEDIUM_DT: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mdt");
    pub const PATTERN_SHORT_DT: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("sdt");

    pub const PATTERN_LONG_DZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ldz");
    pub const PATTERN_MEDIUM_DZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mdz");
    pub const PATTERN_SHORT_DZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("sdz");

    pub const PATTERN_LONG_TZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ltz");
    pub const PATTERN_MEDIUM_TZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mtz");
    pub const PATTERN_SHORT_TZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("stz");

    pub const PATTERN_LONG_DTZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("ldtz");
    pub const PATTERN_MEDIUM_DTZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("mdtz");
    pub const PATTERN_SHORT_DTZ: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("sdtz");

    pub const NUMERIC_STR: &str = NUMERIC.as_str();
    pub const ABBR_STR: &str = ABBR.as_str();
    pub const NARROW_STR: &str = NARROW.as_str();
    pub const WIDE_STR: &str = WIDE.as_str();
    pub const SHORT_STR: &str = SHORT.as_str();
    pub const ABBR_STANDALONE_STR: &str = ABBR_STANDALONE.as_str();
    pub const NARROW_STANDALONE_STR: &str = NARROW_STANDALONE.as_str();
    pub const WIDE_STANDALONE_STR: &str = WIDE_STANDALONE.as_str();
    pub const SHORT_STANDALONE_STR: &str = SHORT_STANDALONE.as_str();

    pub const PATTERN_LONG_STR: &str = PATTERN_LONG.as_str();
    pub const PATTERN_MEDIUM_STR: &str = PATTERN_MEDIUM.as_str();
    pub const PATTERN_SHORT_STR: &str = PATTERN_SHORT.as_str();

    pub const PATTERN_LONG12_STR: &str = PATTERN_LONG12.as_str();
    pub const PATTERN_MEDIUM12_STR: &str = PATTERN_MEDIUM12.as_str();
    pub const PATTERN_SHORT12_STR: &str = PATTERN_SHORT12.as_str();

    pub const PATTERN_LONG24_STR: &str = PATTERN_LONG24.as_str();
    pub const PATTERN_MEDIUM24_STR: &str = PATTERN_MEDIUM24.as_str();
    pub const PATTERN_SHORT24_STR: &str = PATTERN_SHORT24.as_str();

    pub const PATTERN_LONG_DT_STR: &str = PATTERN_LONG_DT.as_str();
    pub const PATTERN_MEDIUM_DT_STR: &str = PATTERN_MEDIUM_DT.as_str();
    pub const PATTERN_SHORT_DT_STR: &str = PATTERN_SHORT_DT.as_str();

    pub const PATTERN_LONG_DZ_STR: &str = PATTERN_LONG_DZ.as_str();
    pub const PATTERN_MEDIUM_DZ_STR: &str = PATTERN_MEDIUM_DZ.as_str();
    pub const PATTERN_SHORT_DZ_STR: &str = PATTERN_SHORT_DZ.as_str();

    pub const PATTERN_LONG_TZ_STR: &str = PATTERN_LONG_TZ.as_str();
    pub const PATTERN_MEDIUM_TZ_STR: &str = PATTERN_MEDIUM_TZ.as_str();
    pub const PATTERN_SHORT_TZ_STR: &str = PATTERN_SHORT_TZ.as_str();

    pub const PATTERN_LONG_DTZ_STR: &str = PATTERN_LONG_DTZ.as_str();
    pub const PATTERN_MEDIUM_DTZ_STR: &str = PATTERN_MEDIUM_DTZ.as_str();
    pub const PATTERN_SHORT_DTZ_STR: &str = PATTERN_SHORT_DTZ.as_str();

    /// Field lengths supported in data marker attribute.
    ///
    /// For a stable version of this enum, use one of the length enums in [`pattern`].
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`pattern`]: crate::pattern
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(clippy::exhaustive_enums)] // documented as unstable
    pub enum Length {
        Abbr,
        Narrow,
        Wide,
        Short,
        Numeric,
    }

    /// Pattern lengths supported in data marker attributes.
    ///
    /// For a stable version of this enum, use [`Length`].
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`Length`]: crate::options::Length
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum PatternLength {
        Long,
        Medium,
        Short,
    }

    /// Field contexts supported in data marker attributes.
    ///
    /// For a stable version of this enum, use one of the specific field symbol enums in [`fields`].
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`fields`]: crate::provider::fields
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(clippy::exhaustive_enums)] // documented as unstable
    pub enum Context {
        Format,
        Standalone,
    }

    /// Date, time, and time zone combinations supported in data marker attributes.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    ///
    /// [`fields`]: crate::provider::fields
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[allow(clippy::exhaustive_enums)] // documented as unstable
    pub enum GlueType {
        DateTime,
        DateZone,
        TimeZone,
        DateTimeZone,
    }

    /// Parses a name data marker attribute to enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn name_marker_attr_info(marker_attr: &DataMarkerAttributes) -> Option<(Context, Length)> {
        use {Context::*, Length::*};
        match &**marker_attr {
            NUMERIC_STR => Some((Format, Numeric)),
            ABBR_STR => Some((Format, Abbr)),
            NARROW_STR => Some((Format, Narrow)),
            WIDE_STR => Some((Format, Wide)),
            SHORT_STR => Some((Format, Short)),
            ABBR_STANDALONE_STR => Some((Standalone, Abbr)),
            NARROW_STANDALONE_STR => Some((Standalone, Narrow)),
            WIDE_STANDALONE_STR => Some((Standalone, Wide)),
            SHORT_STANDALONE_STR => Some((Standalone, Short)),
            _ => None,
        }
    }

    /// Parses a pattern data marker attribute to enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn pattern_marker_attr_info_for_glue(
        marker_attr: &DataMarkerAttributes,
    ) -> Option<(PatternLength, GlueType)> {
        use {GlueType::*, PatternLength::*};
        match &**marker_attr {
            PATTERN_LONG_DT_STR => Some((Long, DateTime)),
            PATTERN_MEDIUM_DT_STR => Some((Medium, DateTime)),
            PATTERN_SHORT_DT_STR => Some((Short, DateTime)),

            PATTERN_LONG_DZ_STR => Some((Long, DateZone)),
            PATTERN_MEDIUM_DZ_STR => Some((Medium, DateZone)),
            PATTERN_SHORT_DZ_STR => Some((Short, DateZone)),

            PATTERN_LONG_TZ_STR => Some((Long, TimeZone)),
            PATTERN_MEDIUM_TZ_STR => Some((Medium, TimeZone)),
            PATTERN_SHORT_TZ_STR => Some((Short, TimeZone)),

            PATTERN_LONG_DTZ_STR => Some((Long, DateTimeZone)),
            PATTERN_MEDIUM_DTZ_STR => Some((Medium, DateTimeZone)),
            PATTERN_SHORT_DTZ_STR => Some((Short, DateTimeZone)),

            _ => None,
        }
    }

    /// Creates a name data marker attribute from the enum values.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    pub fn name_attr_for(context: Context, length: Length) -> &'static DataMarkerAttributes {
        use {Context::*, Length::*};
        match (context, length) {
            (Format, Numeric) => NUMERIC,
            (Format, Abbr) => ABBR,
            (Format, Narrow) => NARROW,
            (Format, Wide) => WIDE,
            (Format, Short) => SHORT,
            (Standalone, Numeric) => NUMERIC,
            (Standalone, Abbr) => ABBR_STANDALONE,
            (Standalone, Narrow) => NARROW_STANDALONE,
            (Standalone, Wide) => WIDE_STANDALONE,
            (Standalone, Short) => SHORT_STANDALONE,
        }
    }

    pub fn pattern_marker_attr_for_glue(
        length: PatternLength,
        glue_type: GlueType,
    ) -> &'static DataMarkerAttributes {
        use {GlueType::*, PatternLength::*};
        match (length, glue_type) {
            (Long, DateTime) => PATTERN_LONG_DT,
            (Medium, DateTime) => PATTERN_MEDIUM_DT,
            (Short, DateTime) => PATTERN_SHORT_DT,

            (Long, DateZone) => PATTERN_LONG_DZ,
            (Medium, DateZone) => PATTERN_MEDIUM_DZ,
            (Short, DateZone) => PATTERN_SHORT_DZ,

            (Long, TimeZone) => PATTERN_LONG_TZ,
            (Medium, TimeZone) => PATTERN_MEDIUM_TZ,
            (Short, TimeZone) => PATTERN_SHORT_TZ,

            (Long, DateTimeZone) => PATTERN_LONG_DTZ,
            (Medium, DateTimeZone) => PATTERN_MEDIUM_DTZ,
            (Short, DateTimeZone) => PATTERN_SHORT_DTZ,
        }
    }
}

// TODO: We may need to support plural forms here. Something like
// pub enum NeoPatternPlurals<'data> {
//     SingleDate(runtime::Pattern<'data>),
//     WeekPlurals(ZeroMap<'data, PluralCategory, runtime::PatternULE>),
// }

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use diplomat_runtime::DiplomatOption;

    use crate::datetime_options::ffi::{
        DateTimeAlignment, DateTimeLength, TimePrecision, YearStyle,
    };

    #[diplomat::enum_convert(icu_datetime::fieldsets::builder::DateFields, needs_wildcard)]
    #[diplomat::rust_link(icu_datetime::fieldsets::builder::DateFields, Enum)]
    pub enum DateFields {
        D,
        MD,
        YMD,
        DE,
        MDE,
        YMDE,
        E,
        M,
        YM,
        Y,
    }

    #[diplomat::enum_convert(icu_datetime::fieldsets::builder::ZoneStyle, needs_wildcard)]
    #[diplomat::rust_link(icu_datetime::fieldsets::builder::ZoneStyle, Enum)]
    pub enum ZoneStyle {
        SpecificLong,
        SpecificShort,
        LocalizedOffsetLong,
        LocalizedOffsetShort,
        GenericLong,
        GenericShort,
        Location,
        ExemplarCity,
    }

    #[diplomat::rust_link(icu_datetime::fieldsets::builder::FieldSetBuilder, Struct)]
    pub struct DateTimeFieldSetBuilder {
        pub length: DiplomatOption<DateTimeLength>,
        pub date_fields: DiplomatOption<DateFields>,
        pub time_precision: DiplomatOption<TimePrecision>,
        pub zone_style: DiplomatOption<ZoneStyle>,
        pub alignment: DiplomatOption<DateTimeAlignment>,
        pub year_style: DiplomatOption<YearStyle>,
    }
}

impl From<icu_datetime::fieldsets::builder::FieldSetBuilder> for ffi::DateTimeFieldSetBuilder {
    fn from(value: icu_datetime::fieldsets::builder::FieldSetBuilder) -> Self {
        Self {
            length: value.length.map(Into::into).into(),
            date_fields: value.date_fields.map(Into::into).into(),
            time_precision: value.time_precision.map(Into::into).into(),
            zone_style: value.zone_style.map(Into::into).into(),
            alignment: value.alignment.map(Into::into).into(),
            year_style: value.year_style.map(Into::into).into(),
        }
    }
}

impl From<&ffi::DateTimeFieldSetBuilder> for icu_datetime::fieldsets::builder::FieldSetBuilder {
    fn from(value: &ffi::DateTimeFieldSetBuilder) -> Self {
        let mut res = icu_datetime::fieldsets::builder::FieldSetBuilder::default();
        res.length = value.length.as_ref().ok().map(|x| (*x).into());
        res.date_fields = value.date_fields.as_ref().ok().map(|x| (*x).into());
        res.time_precision = value.time_precision.as_ref().ok().map(|x| (*x).into());
        res.zone_style = value.zone_style.as_ref().ok().map(|x| (*x).into());
        res.alignment = value.alignment.as_ref().ok().map(|x| (*x).into());
        res.year_style = value.year_style.as_ref().ok().map(|x| (*x).into());
        res
    }
}

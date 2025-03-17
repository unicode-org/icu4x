// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{fs::File, path::PathBuf};

use askama::Template;
use icu::datetime::fieldsets::builder::*;

#[derive(Debug)]
struct ConsumedOptions {
    length: bool,
    alignment: bool,
    year_style: bool,
}

impl ConsumedOptions {
    fn from_builder(builder: FieldSetBuilder) -> Option<Self> {
        match builder.build_composite() {
            Ok(_) => Some(ConsumedOptions {
                length: true,
                alignment: true,
                year_style: true,
            }),
            Err(BuilderError::SuperfluousOptions(options)) => Some(ConsumedOptions {
                length: options.length.is_none(),
                alignment: options.alignment.is_none(),
                year_style: options.year_style.is_none(),
            }),
            Err(BuilderError::InvalidDateFields) => None,
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}

struct DateFieldsWrap(DateFields);

impl DateFieldsWrap {
    pub fn name_upper(&self) -> &str {
        match self.0 {
            DateFields::D => "D",
            DateFields::MD => "MD",
            DateFields::YMD => "YMD",
            DateFields::DE => "DE",
            DateFields::MDE => "MDE",
            DateFields::YMDE => "YMDE",
            DateFields::E => "E",
            DateFields::M => "M",
            DateFields::YM => "YM",
            DateFields::Y => "Y",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn name_lower(&self) -> &str {
        match self.0 {
            DateFields::D => "d",
            DateFields::MD => "md",
            DateFields::YMD => "ymd",
            DateFields::DE => "de",
            DateFields::MDE => "mde",
            DateFields::YMDE => "ymde",
            DateFields::E => "e",
            DateFields::M => "m",
            DateFields::YM => "ym",
            DateFields::Y => "y",
            _ => unreachable!("unknown variant"),
        }
    }
    pub fn is_default_constructor(&self) -> bool {
        return matches!(self.0, DateFields::YMD);
    }
}

#[derive(Template, Default)]
#[template(path = "date_formatter.rs.jinja")]
struct DateFormatterTemplate {
    variants: Vec<DateFormatterVariant>,
}

struct DateFormatterVariant {
    date_fields: DateFieldsWrap,
    consumed_options: ConsumedOptions,
}

pub fn main() {
    let mut date_formatter_template = DateFormatterTemplate::default();

    for date_fields in DateFields::VALUES.iter() {
        // Determine the options for these date fields
        let mut builder = FieldSetBuilder::new();
        builder.date_fields = Some(*date_fields);
        builder.length = Some(Default::default());
        builder.alignment = Some(Default::default());
        builder.year_style = Some(Default::default());

        let consumed_options = ConsumedOptions::from_builder(builder.clone()).unwrap();
        println!("{date_fields:?} as Date => {consumed_options:?}");
        assert!(consumed_options.length); // all constructors accept a length
        date_formatter_template.variants.push(DateFormatterVariant {
            date_fields: DateFieldsWrap(*date_fields),
            consumed_options,
        });

        builder.time_precision = Some(Default::default());
        let consumed_options = ConsumedOptions::from_builder(builder.clone());
        println!("{date_fields:?} as DateTime => {consumed_options:?}");

        builder.zone_style = Some(ZoneStyle::LocalizedOffsetShort);
        let consumed_options = ConsumedOptions::from_builder(builder.clone());
        println!("{date_fields:?} as DateTimeZone => {consumed_options:?}");

        builder.time_precision = None;
        let consumed_options = ConsumedOptions::from_builder(builder.clone());
        println!("{date_fields:?} as DateZone => {consumed_options:?}");
    }

    let mut path_buf = PathBuf::new();
    path_buf.push(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("../../../ffi/capi/src");

    {
        let mut path_buf = path_buf.clone();
        path_buf.push("date_formatter.rs");
        let mut file = File::create(&path_buf).unwrap();
        use std::io::Write;
        write!(&mut file, "{}", date_formatter_template).unwrap();
    }
}

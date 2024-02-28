// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example program making use of a number of ICU components
// in a pseudo-real-world application of Textual User Interface.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use icu::calendar::{DateTime, Gregorian};
use icu::datetime::time_zone::TimeZoneFormatterOptions;
use icu::datetime::{DateTimeFormatterOptions, TypedZonedDateTimeFormatter};
use icu::locid::{locale, Locale};
use icu::plurals::{PluralCategory, PluralRules};
use icu::timezone::CustomTimeZone;
use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use std::env;
use std::str::FromStr;

fn print<T: AsRef<str>>(_input: T) {
    #[cfg(debug_assertions)]
    println!("{}", _input.as_ref());
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let args: Vec<String> = env::args().collect();

    let locale: Locale = args
        .get(1)
        .map(|s| s.parse().expect("Failed to parse locale"))
        .unwrap_or_else(|| locale!("en"));

    let user_name = args.as_slice().get(2).map(String::as_str).unwrap_or("John");

    let email_count: usize = args
        .get(3)
        .map(|s| {
            s.parse()
                .expect("Could not parse unread email count as unsigned integer.")
        })
        .unwrap_or(5);

    print(format!("\nTextual User Interface Example ({locale})"));
    print("===================================");
    print(format!("User: {user_name}"));

    {
        let dtf = TypedZonedDateTimeFormatter::<Gregorian>::try_new(
            &locale.into(),
            DateTimeFormatterOptions::default(),
            TimeZoneFormatterOptions::default(),
        )
        .expect("Failed to create TypedDateTimeFormatter.");
        let today_date = DateTime::try_new_gregorian_datetime(2020, 10, 10, 18, 56, 0).unwrap();
        let today_tz = CustomTimeZone::from_str("Z").unwrap(); // Z refers to the utc timezone

        let formatted_dt = dtf.format(&today_date, &today_tz);

        print(format!("Today is: {formatted_dt}"));
    }

    {
        let mut builder = CodePointInversionListBuilder::new();
        // See http://ftp.unicode.org/Public/MAPPINGS/ISO8859/8859-1.TXT
        builder.add_range(&('\u{0000}'..='\u{00FF}'));
        let latin1_set = builder.build();

        let only_latin1 = user_name.chars().all(|ch| latin1_set.contains(ch));

        if only_latin1 {
            print("User name latin1 only: true");
        } else {
            print("User name latin1 only: false");
        }
    }

    {
        let pr = PluralRules::try_new_cardinal(&locale!("en").into())
            .expect("Failed to create PluralRules.");

        match pr.category_for(email_count) {
            PluralCategory::One => print("Note: You have one unread email."),
            _ => print(format!("Note: You have {email_count} unread emails.")),
        }
    }

    0
}

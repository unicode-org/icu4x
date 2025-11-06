// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example program making use of a number of ICU components
// in a pseudo-real-world application of Textual User Interface.

use icu::calendar::{Date, Gregorian};
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::datetime::fieldsets::{self, YMDT};
use icu::datetime::FixedCalendarDateTimeFormatter;
use icu::locale::locale;
use icu::plurals::{PluralCategory, PluralRules};
use icu::time::TimeZoneInfo;
use icu::time::{Time, ZonedDateTime};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let locale = args
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

    println!("\nTextual User Interface Example ({locale})");
    println!("===================================");
    println!("User: {user_name}");

    {
        let dtf = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
            locale.into(),
            YMDT::medium().with_zone(fieldsets::zone::LocalizedOffsetShort),
        )
        .expect("Failed to create zoned datetime formatter.");
        let date = Date::try_new_gregorian(2020, 10, 10).unwrap();
        let time = Time::try_new(18, 56, 0, 0).unwrap();
        let zone = TimeZoneInfo::utc();

        let formatted_dt = dtf.format(&ZonedDateTime { date, time, zone });

        println!("Today is: {formatted_dt}");
    }

    {
        let mut builder = CodePointInversionListBuilder::new();
        // See http://ftp.unicode.org/Public/MAPPINGS/ISO8859/8859-1.TXT
        builder.add_range('\u{0000}'..='\u{00FF}');
        let latin1_set = builder.build();

        let only_latin1 = user_name.chars().all(|ch| latin1_set.contains(ch));

        if only_latin1 {
            println!("User name latin1 only: true");
        } else {
            println!("User name latin1 only: false");
        }
    }

    {
        let pr = PluralRules::try_new_cardinal(locale!("en").into())
            .expect("Failed to create PluralRules.");

        match pr.category_for(email_count) {
            PluralCategory::One => println!("Note: You have one unread email."),
            _ => println!("Note: You have {email_count} unread emails."),
        }
    }
}

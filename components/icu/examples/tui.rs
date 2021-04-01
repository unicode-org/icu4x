// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example program making use of a number of ICU components
// in a pseudo-real-world application of Textual User Interface.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use icu::datetime::DateTimeFormatOptions;
use icu::locid::{macros::langid, Locale};
use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
use icu::uniset::UnicodeSetBuilder;
use icu_datetime::{mock::zoned_datetime::MockZonedDateTime, ZonedDateTimeFormat};
use std::env;

fn print<T: AsRef<str>>(_input: T) {
    #[cfg(debug_assertions)]
    println!("{}", _input.as_ref());
}

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let provider = icu_testdata::get_provider();

    let args: Vec<String> = env::args().collect();

    let locale: Locale = args
        .get(1)
        .map(|s| s.parse().expect("Failed to parse language identifier"))
        .unwrap_or_else(|| langid!("en").into());

    let user_name = args.get(2).cloned().unwrap_or_else(|| "John".to_string());

    let email_count: usize = args
        .get(3)
        .unwrap_or(&"5".to_string())
        .parse()
        .expect("Could not parse unread email count as unsigned integer.");

    print(format!("\nTextual User Interface Example ({})", locale));
    print("===================================");
    print(format!("User: {}", user_name));

    {
        let dtf = ZonedDateTimeFormat::try_new(
            locale,
            &provider,
            &provider,
            &DateTimeFormatOptions::default(),
        )
        .expect("Failed to create DateTimeFormat.");
        let today: MockZonedDateTime = "2020-10-10T18:56:00Z"
            .parse()
            .expect("Failed to parse date");

        let formatted_dt = dtf.format(&today);

        print(format!("Today is: {}", formatted_dt));
    }

    {
        let mut builder = UnicodeSetBuilder::new();
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
        let en = langid!("en");
        let pr = PluralRules::try_new(en, &provider, PluralRuleType::Cardinal)
            .expect("Failed to create PluralRules.");

        match pr.select(email_count) {
            PluralCategory::One => print("Note: You have one unread email."),
            _ => print(format!("Note: You have {} unread emails.", email_count)),
        }
    }

    0
}

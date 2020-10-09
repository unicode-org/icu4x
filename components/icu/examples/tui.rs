// An example program making use of a number of ICU components
// in a pseudo-real-world application of Textual User Interface.
use icu::datetime::{date::MockDateTime, DateTimeFormat, DateTimeFormatOptions};
use icu::locale::LanguageIdentifier;
use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
use icu::uniset::UnicodeSetBuilder;
use icu_fs_data_provider::FsDataProvider;
use std::env;

fn print<T: AsRef<str>>(_input: T) {
    #[cfg(debug_assertions)]
    println!("{}", _input.as_ref());
}

fn main() {
    let provider = FsDataProvider::try_new("../../resources/testdata/data/json")
        .expect("Loading file from testdata directory");

    let args: Vec<String> = env::args().collect();

    let langid: LanguageIdentifier = args
        .get(1)
        .unwrap_or(&"en".to_string())
        .parse()
        .expect("Failed to parse language identifier.");

    let user_name = args.get(2).cloned().unwrap_or_else(|| "John".to_string());

    let email_count: usize = args
        .get(3)
        .unwrap_or(&"5".to_string())
        .parse()
        .expect("Could not parse unread email count as unsigned integer.");

    print(format!("\nTextual User Interface Example ({})", langid));
    print("===================================");
    print(format!("User: {}", user_name));

    {
        let dtf = DateTimeFormat::try_new(langid, &provider, &DateTimeFormatOptions::default())
            .expect("Failed to create DateTimeFormat.");
        let today: MockDateTime = "2020-10-10T18:56:00".parse().expect("Failed to parse date");

        let formatted_dt = dtf.format(&today);

        print(format!("Today is: {}", formatted_dt));
    }

    {
        let mut builder = UnicodeSetBuilder::new();
        builder.add_range(&('\u{0000}'..='\u{007F}'));
        let latin1_set = builder.build();

        let only_latin1 = user_name.chars().all(|ch| latin1_set.contains(ch));

        if only_latin1 {
            print("User name latin1 only: true");
        } else {
            print("User name latin1 only: false");
        }
    }

    {
        let en: LanguageIdentifier = "en".parse().expect("Failed to parse Language Identifier.");
        let pr = PluralRules::try_new(en, &provider, PluralRuleType::Cardinal)
            .expect("Failed to create PluralRules.");

        match pr.select(email_count) {
            PluralCategory::One => print("Note: You have one unread email."),
            _ => print(format!("Note: You have {} unread emails.", email_count)),
        }
    }
}

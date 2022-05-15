mod dtf;
mod preferences;

pub use dtf::{DTFPreferencesBag, DateTimeFormat};
pub use preferences::Preferences;

#[cfg(test)]
mod tests {
    use super::*;
    use icu_datetime::options::preferences::HourCycle;
    use icu_locid::{language, locale, Locale};

    #[test]
    fn it_works() {
        {
            // Default locale, no prefs
            let loc = locale!("en-US");
            let dtf = DateTimeFormat::try_new(&loc);
            assert_eq!(dtf.format().as_str(), "05/13/2022 3:00pm");
        }

        {
            // Prefs, no locale
            let p = DTFPreferencesBag {
                hour_cycle: Some(HourCycle::H12),
                ..Default::default()
            };
            let dtf = DateTimeFormat::try_new(&p);
            assert_eq!(dtf.format().as_str(), "ISO DATE 3:00pm");
        }

        {
            // Prefs and locale
            let mut p = DTFPreferencesBag {
                hour_cycle: Some(HourCycle::H24),
                ..Default::default()
            };
            p.merge_locale(&locale!("fr-CA")).unwrap();

            let dtf = DateTimeFormat::try_new(&p);
            assert_eq!(dtf.format().as_str(), "13/05/2022 15:00");
        }

        {
            // Prefs and locale, override language
            let mut p = DTFPreferencesBag {
                language: Some(language!("en")),
                ..Default::default()
            };
            p.merge_locale(&locale!("fr-CA")).unwrap();

            let dtf = DateTimeFormat::try_new(&p);
            assert_eq!(dtf.format().as_str(), "05/13/2022 3:00pm");
        }

        {
            // Prefs and locale, override language and set hour_cycle
            let mut p = DTFPreferencesBag {
                language: Some(language!("en")),
                hour_cycle: Some(HourCycle::H24),
                ..Default::default()
            };
            p.merge_locale(&locale!("fr-CA")).unwrap();

            let dtf = DateTimeFormat::try_new(&p);
            assert_eq!(dtf.format().as_str(), "05/13/2022 15:00");
        }

        {
            let mut p = DTFPreferencesBag {
                ..Default::default()
            };
            let loc: Locale = "fr-CA-u-hc-h12".parse().unwrap();
            p.merge_locale(&loc).unwrap();

            let dtf = DateTimeFormat::try_new(&p);
            assert_eq!(dtf.format().as_str(), "13/05/2022 3:00pm");
        }
    }
}

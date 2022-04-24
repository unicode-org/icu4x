#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use super::*;
    use icu_datetime::options::preferences::HourCycle;
    use icu_locid::{unicode_ext_key, Locale};

    preferences!(
        Preferences,
        ResolvedPreferences,
        {
            hour_cycle => Option<HourCycle>, HourCycle, Some(unicode_ext_key!("hc"))
        }
    );

    #[test]
    fn it_works() {
        let l: Locale = "en-US-u-hc-h23".parse().unwrap();

        let prefs = Preferences { hour_cycle: None };

        let defaults = ResolvedPreferences {
            hour_cycle: HourCycle::H12,
        };

        let resolved_prefs = ResolvedPreferences::try_from((&prefs, &l, &defaults)).unwrap();

        assert_eq!(resolved_prefs.hour_cycle, HourCycle::H23);
    }
}

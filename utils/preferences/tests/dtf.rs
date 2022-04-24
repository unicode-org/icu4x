use icu_datetime::options::preferences::HourCycle;
use icu_locid::{unicode_ext_key, Locale};
use icu_preferences::preferences;

#[derive(Clone, Copy)]
enum Calendar {
    Gregory,
}

impl TryFrom<&icu_locid::extensions::unicode::Value> for Calendar {
    type Error = ();

    fn try_from(_i: &icu_locid::extensions::unicode::Value) -> Result<Self, Self::Error> {
        Ok(Calendar::Gregory)
    }
}

preferences!(
    DTFPreferences,
    ResolvedDTFPreferences,
    {
        hour_cycle => Option<HourCycle>, HourCycle, Some(unicode_ext_key!("hc")),
        calendar => Option<Calendar>, Calendar, Some(unicode_ext_key!("ca"))
    }
);

struct DateTimeFormat {
    prefs: ResolvedDTFPreferences,
}

impl DateTimeFormat {
    pub fn new(locale: &Locale, prefs: &DTFPreferences) -> Self {
        let en_us_defaults = ResolvedDTFPreferences {
            hour_cycle: HourCycle::H12,
            calendar: Calendar::Gregory,
        };

        let prefs = ResolvedDTFPreferences::try_from((prefs, locale, &en_us_defaults)).unwrap();

        Self { prefs }
    }

    pub fn format(&self, _input: u64) -> String {
        match self.prefs.hour_cycle {
            HourCycle::H11 => "00:13 am",
            HourCycle::H12 => "12:13 am",
            HourCycle::H23 => "00:13",
            HourCycle::H24 => "24:13",
        }
        .to_string()
    }
}

#[test]
fn dtf_default() {
    let loc: Locale = "en-US".parse().unwrap();
    let prefs = DTFPreferences {
        hour_cycle: None,
        ..Default::default()
    };
    let dtf = DateTimeFormat::new(&loc, &prefs);
    assert_eq!(dtf.format(0), String::from("12:13 am"));
}

#[test]
fn dtf_uext() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let prefs = DTFPreferences {
        hour_cycle: None,
        ..Default::default()
    };
    let dtf = DateTimeFormat::new(&loc, &prefs);
    assert_eq!(dtf.format(0), String::from("00:13 am"));
}

#[test]
fn dtf_prefs() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let prefs = DTFPreferences {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    let dtf = DateTimeFormat::new(&loc, &prefs);
    assert_eq!(dtf.format(0), String::from("24:13"));
}

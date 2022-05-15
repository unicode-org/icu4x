use icu_datetime::options::preferences::HourCycle;
use icu_locid::{langid, language, locale, unicode_ext_key, unicode_ext_value, Locale};
use icu_preferences::{preferences, Preferences};

#[derive(Clone, Copy)]
pub enum Calendar {
    Gregory,
    Buddhist,
}

impl TryFrom<&icu_locid::extensions::unicode::Value> for Calendar {
    type Error = ();

    fn try_from(i: &icu_locid::extensions::unicode::Value) -> Result<Self, Self::Error> {
        match i {
            _ if *i == unicode_ext_value!("gregory") => Ok(Calendar::Gregory),
            _ if *i == unicode_ext_value!("buddhist") => Ok(Calendar::Buddhist),
            _ => Err(()),
        }
    }
}

preferences!(
    DTFPreferences,
    DTFPreferencesBag,
    ResolvedDTFPreferencesBag,
    {
        hour_cycle => Option<HourCycle>, HourCycle, Some(unicode_ext_key!("hc")),
        calendar => Option<Calendar>, Calendar, Some(unicode_ext_key!("ca"))
    }
);

fn get_defaults(preferences: &impl DTFPreferences) -> ResolvedDTFPreferencesBag {
    let defaults = ResolvedDTFPreferencesBag {
        lid: langid!("und"),

        hour_cycle: if *preferences.language() == language!("en") {
            HourCycle::H12
        } else {
            HourCycle::H24
        },
        calendar: Calendar::Gregory,
    };
    defaults
}

struct DateTimeFormat {
    prefs: ResolvedDTFPreferencesBag,
}

impl DateTimeFormat {
    pub fn new(prefs: &impl DTFPreferences) -> Self {
        let defaults = get_defaults(prefs);

        let prefs = defaults.resolve(prefs);

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
    let loc = locale!("en-US");
    let dtf = DateTimeFormat::new(&loc);
    assert_eq!(dtf.format(0), String::from("12:13 am"));
}

#[test]
fn dtf_uext() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let dtf = DateTimeFormat::new(&loc);
    assert_eq!(dtf.format(0), String::from("00:13 am"));
}

#[test]
fn dtf_prefs() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let mut prefs = DTFPreferencesBag {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    prefs.merge_locale(&loc).unwrap();
    let dtf = DateTimeFormat::new(&prefs);
    assert_eq!(dtf.format(0), String::from("24:13"));
}

#[test]
fn dtf_prefs_with_ca() {
    let loc: Locale = "en-US-u-hc-h11-ca-buddhist".parse().unwrap();
    let mut prefs = DTFPreferencesBag {
        hour_cycle: Some(HourCycle::H24),
        ..Default::default()
    };
    prefs.merge_locale(&loc).unwrap();
    let dtf = DateTimeFormat::new(&prefs);
    assert_eq!(dtf.format(0), String::from("24:13"));
}

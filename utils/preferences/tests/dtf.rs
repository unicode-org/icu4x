use icu_datetime::options::preferences::HourCycle;
use icu_locid::{langid, language, locale, region, unicode_ext_key, unicode_ext_value, Locale};
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
    DTFPreferencesBag,
    ResolvedDTFPreferencesBag,
    {
        hour_cycle => Option<HourCycle>, HourCycle, Some(unicode_ext_key!("hc")),
        calendar => Option<Calendar>, Calendar, Some(unicode_ext_key!("ca"))
    }
);

fn get_defaults(prefs: &impl Preferences) -> ResolvedDTFPreferencesBag {
    match (prefs.language().as_str(), prefs.script(), prefs.region()) {
        ("en", _, _) => ResolvedDTFPreferencesBag {
            lid: langid!("en-US"),
            hour_cycle: HourCycle::H12,
            calendar: Calendar::Gregory,
        },
        _ => ResolvedDTFPreferencesBag {
            lid: langid!("und"),
            hour_cycle: HourCycle::H24,
            calendar: Calendar::Gregory,
        },
    }
}

pub struct DateTimeFormat {
    prefs: ResolvedDTFPreferencesBag,
}

impl DateTimeFormat {
    pub fn new(prefs: &DTFPreferencesBag) -> Self {
        let mut resolved = get_defaults(prefs);

        resolved.resolve(prefs);

        Self { prefs: resolved }
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

    pub fn resolved_options(&self) -> &ResolvedDTFPreferencesBag {
        &self.prefs
    }
}

#[test]
fn dtf_default() {
    let loc = locale!("en-US");
    let dtf_bag: DTFPreferencesBag = loc.try_into().unwrap();
    let dtf = DateTimeFormat::new(&dtf_bag);
    assert_eq!(dtf.format(0), String::from("12:13 am"));
}

#[test]
fn dtf_uext() {
    let loc: Locale = "en-US-u-hc-h11".parse().unwrap();
    let dtf_bag: DTFPreferencesBag = loc.try_into().unwrap();
    let dtf = DateTimeFormat::new(&dtf_bag);
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

#[test]
fn dtf_prefs_default_region() {
    let loc: Locale = "en-u-hc-h12".parse().unwrap();
    let dtf_bag: DTFPreferencesBag = loc.try_into().unwrap();
    let dtf = DateTimeFormat::new(&dtf_bag);
    assert_eq!(dtf.format(0), String::from("12:13 am"));
    assert_eq!(dtf.resolved_options().lid.language, language!("en"));
    assert_eq!(dtf.resolved_options().lid.region, Some(region!("US")));
}

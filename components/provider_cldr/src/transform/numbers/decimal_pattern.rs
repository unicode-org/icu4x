use std::str::FromStr;
type SmallString8 = smallstr::SmallString<[u8; 8]>;
use icu_decimal::provider::AffixesV1;

pub struct DecimalPatternParseResult {
    pub unsigned_affixes: (SmallString8, SmallString8),
    pub signed_affixes: Option<(SmallString8, SmallString8)>,
    pub primary_grouping: u8,
    pub secondary_grouping: u8,
    pub min_fraction_digits: u8,
    pub max_fraction_digits: u8,
}

impl FromStr for DecimalPatternParseResult {
    type Err = &'static str;

    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
        // Example patterns:
        // #,##,##0.###
        // #,##0.00;#,##0.00-
        // TODO
        Ok(Self {
            unsigned_affixes: ("".into(), "".into()),
            signed_affixes: None,
            primary_grouping: 3,
            secondary_grouping: 3,
            min_fraction_digits: 0,
            max_fraction_digits: 3,
        })
    }
}

impl DecimalPatternParseResult {
    pub fn localize_sign(&self, sign_str: &str) -> AffixesV1 {
        // UTS 35: the absence of a negative pattern means a single prefixed sign
        let signed_affixes = self
            .signed_affixes
            .clone()
            .unwrap_or_else(|| ("-".into(), "".into()));
        AffixesV1 {
            prefix: signed_affixes.0.replace("-", sign_str).into(),
            suffix: signed_affixes.1.replace("-", sign_str).into(),
        }
    }
}

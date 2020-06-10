use crate::data::cldr_resource::Resource;
use crate::data::provider::{DataProviderError, DataProviderType};
use crate::data::{PluralRuleList, RulesSelector};
use crate::rules::parse;
use crate::PluralCategory;
use crate::PluralRuleType;
use icu_locale::LanguageIdentifier;
use std::fs::File;
use std::io::Read;

static mut ORDINALS_STRING: Vec<u8> = vec![];
static mut CARDINALS_STRING: Vec<u8> = vec![];
static mut ORDINALS: Option<Resource> = None;
static mut CARDINALS: Option<Resource> = None;

#[derive(Default)]
pub struct DataProvider {}

impl DataProviderType for DataProvider {
    fn get_selector(
        &self,
        locale: &LanguageIdentifier,
        type_: PluralRuleType,
    ) -> Result<Option<RulesSelector>, DataProviderError> {
        Ok(get_rules(locale, type_)?.map(RulesSelector::Conditions))
    }
}

pub fn get_resource(
    type_: PluralRuleType,
) -> Result<&'static Resource<'static>, DataProviderError> {
    let rules = match type_ {
        PluralRuleType::Cardinal => unsafe { &mut ORDINALS },
        PluralRuleType::Ordinal => unsafe { &mut CARDINALS },
    };

    let s = match type_ {
        PluralRuleType::Cardinal => unsafe { &mut ORDINALS_STRING },
        PluralRuleType::Ordinal => unsafe { &mut CARDINALS_STRING },
    };

    if rules.is_none() {
        let path = match type_ {
            PluralRuleType::Cardinal => "./data/plurals.dat",
            PluralRuleType::Ordinal => "./data/ordinals.dat",
        };

        let mut fh = File::open(path).map_err(|_| DataProviderError::IO)?;

        fh.read_to_end(s).map_err(|_| DataProviderError::IO)?;

        let res: Resource =
            bincode::deserialize(s).map_err(|_| DataProviderError::Deserialization)?;

        *rules = Some(res);
    }

    Ok(rules.as_ref().unwrap())
}

pub fn get_rules(
    locale: &LanguageIdentifier,
    type_: PluralRuleType,
) -> Result<Option<PluralRuleList>, DataProviderError> {
    let res = get_resource(type_)?;

    let rules = match type_ {
        PluralRuleType::Cardinal => res.supplemental.plurals_type_cardinal.as_ref().unwrap(),
        PluralRuleType::Ordinal => res.supplemental.plurals_type_ordinal.as_ref().unwrap(),
    };

    let lstr = locale.to_string();

    let lang_idx = if let Ok(idx) = rules.0.binary_search_by_key(&lstr.as_str(), |(l, _)| &l) {
        idx
    } else {
        return Ok(None);
    };
    let lang_rules = &rules.0[lang_idx].1;

    let result = PluralCategory::all()
        .filter_map(|pc| {
            let input = lang_rules.get(pc)?;
            Some(parse(input.as_bytes()).map(|ast| (*pc, ast)))
        })
        .collect::<Result<_, _>>()?;

    Ok(Some(result))
}

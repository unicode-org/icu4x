use crate::data::cldr_resource::Resource;
use crate::data::provider::{DataProviderError, DataProviderType};
use crate::data::{PluralRuleList, RulesSelector};
use crate::rules::parse;
use crate::PluralCategory;
use crate::PluralRuleType;
use icu_locale::LanguageIdentifier;
use std::fs::File;
use std::io::Read;

static mut ORDINALS_STRING: String = String::new();
static mut CARDINALS_STRING: String = String::new();
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

/// # Safety
///
/// This method operates on a static mutable data and is
/// inherently single-threaded.
///
/// This is a temporary solution until we swap the whole data management
/// to use DataProvider.
pub unsafe fn get_resource(
    type_: PluralRuleType,
) -> Result<&'static Resource<'static>, DataProviderError> {
    let rules = match type_ {
        PluralRuleType::Cardinal => &mut ORDINALS,
        PluralRuleType::Ordinal => &mut CARDINALS,
    };

    let s = match type_ {
        PluralRuleType::Cardinal => &mut ORDINALS_STRING,
        PluralRuleType::Ordinal => &mut CARDINALS_STRING,
    };

    if rules.is_none() {
        let path = match type_ {
            PluralRuleType::Cardinal => "./data/plurals.json",
            PluralRuleType::Ordinal => "./data/ordinals.json",
        };
        File::open(path)
            .map_err(|_| DataProviderError::IO)?
            .read_to_string(s)
            .map_err(|_| DataProviderError::IO)?;

        let res: Resource =
            serde_json::from_str(s).map_err(|_| DataProviderError::Deserialization)?;
        *rules = Some(res);
    }

    Ok(rules.as_ref().expect("Rules were just populated."))
}

pub fn get_rules(
    locale: &LanguageIdentifier,
    type_: PluralRuleType,
) -> Result<Option<PluralRuleList>, DataProviderError> {
    let res = unsafe { get_resource(type_)? };

    let rules = match type_ {
        PluralRuleType::Cardinal => res.supplemental.plurals_type_cardinal.as_ref(),
        PluralRuleType::Ordinal => res.supplemental.plurals_type_ordinal.as_ref(),
    };

    if let Some(rules) = rules {
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
    } else {
        Ok(None)
    }
}

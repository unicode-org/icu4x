use std::borrow::Cow;
use std::convert::TryFrom;

use crate::cldr_langid::CldrLanguage;
use icu_data_provider::plurals::PluralRuleStringsV1;
use icu_data_provider::*;
use icu_locale::LanguageIdentifier;
use serde::Deserialize;
use serde_json;

// TODO: Make this non-pub
#[derive(PartialEq, Debug)]
pub struct LanguagePluralsPair {
    langid: LanguageIdentifier,
    rules: PluralRuleStringsV1,
}

#[derive(PartialEq, Debug)]
pub struct CldrPluralsDataProvider<'d> {
    resource: Resource<'d>,
}

impl<'d> TryFrom<&'d str> for CldrPluralsDataProvider<'d> {
    type Error = serde_json::error::Error;
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        Ok(CldrPluralsDataProvider {
            resource: serde_json::from_str(s)?
        })
    }
}

impl<'d> CldrPluralsDataProvider<'d> {
    fn get_rules_for(&self, data_key: &DataKey) -> Result<&Rules<'d>, ResponseError> {
        if data_key.category != Category::Plurals {
            return Err((&data_key.category).into());
        }
        if data_key.version != 1 {
            return Err(data_key.into());
        }
        match data_key.sub_category.as_str() {
            "cardinal" => self.resource.supplemental.plurals_type_cardinal.as_ref(),
            "ordinal" => self.resource.supplemental.plurals_type_ordinal.as_ref(),
            _ => return Err(ResponseError::UnsupportedDataKeyError(*data_key)),
        }
        .ok_or(ResponseError::UnavailableEntryError)
    }
}

impl<'a, 'd> DataProvider<'a, 'd> for CldrPluralsDataProvider<'d> {
    fn load(&'a self, req: &Request) -> Result<Response<'d>, ResponseError> {
        let list: &Rules<'d> = self.get_rules_for(&req.data_key)?;
        // TODO: Implement language fallback
        // TODO: Avoid the clone.
        let cldr_langid = CldrLanguage(req.data_entry.langid.clone());
        let (_, r) = match list.0.binary_search_by_key(&&cldr_langid, |(l, _)| l) {
            Ok(idx) => &list.0[idx],
            Err(_) => return Err(ResponseError::UnavailableEntryError),
        };
        Ok(ResponseBuilder {
            data_langid: req.data_entry.langid.clone(),
        }
        .with_owned_payload(PluralRuleStringsV1::from(r)))
    }
}

impl<'d> IterableDataProvider<'d> for CldrPluralsDataProvider<'d> {
    // TODO: The following works in nightly with type_alias_impl_trait:
    // type Iter = impl Iterator<Item = DataEntry>;
    type Iter = std::iter::Map<
        std::slice::Iter<'d, (CldrLanguage, LocalePluralRules<'d>)>,
        fn(&(CldrLanguage, LocalePluralRules<'d>)) -> DataEntry,
    >;

    fn iter_for_key(&'d self, data_key: &DataKey) -> Result<Self::Iter, ResponseError> {
        let list: &Rules<'d> = self.get_rules_for(data_key)?;
        Ok(list.0.iter().map(|(l, _)| DataEntry {
            variant: None,
            // TODO: Avoid the clone
            langid: l.0.clone(),
        }))
    }
}

impl<'d> From<&LocalePluralRules<'d>> for PluralRuleStringsV1 {
    fn from(other: &LocalePluralRules) -> PluralRuleStringsV1 {
        fn convert(s: &str) -> Cow<'static, str> {
            Cow::Owned(s.to_string())
        }
        PluralRuleStringsV1 {
            zero: other.zero.map(convert),
            one: other.one.map(convert),
            two: other.two.map(convert),
            few: other.few.map(convert),
            many: other.many.map(convert),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
// TODO: Make this non-pub
pub struct LocalePluralRules<'s> {
    #[serde(rename = "pluralRule-count-zero")]
    pub zero: Option<&'s str>,
    #[serde(rename = "pluralRule-count-one")]
    pub one: Option<&'s str>,
    #[serde(rename = "pluralRule-count-two")]
    pub two: Option<&'s str>,
    #[serde(rename = "pluralRule-count-few")]
    pub few: Option<&'s str>,
    #[serde(rename = "pluralRule-count-many")]
    pub many: Option<&'s str>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct Rules<'s>(
    #[serde(with = "tuple_vec_map")]
    #[serde(borrow)]
    pub Vec<(CldrLanguage, LocalePluralRules<'s>)>,
);

#[derive(PartialEq, Debug, Deserialize)]
struct Supplemental<'s> {
    #[serde(rename = "plurals-type-cardinal")]
    #[serde(borrow)]
    pub plurals_type_cardinal: Option<Rules<'s>>,
    #[serde(rename = "plurals-type-ordinal")]
    #[serde(borrow)]
    pub plurals_type_ordinal: Option<Rules<'s>>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct Resource<'s> {
    #[serde(borrow)]
    pub supplemental: Supplemental<'s>,
}

use std::borrow::Cow;
use std::str::FromStr;

use json;

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
pub struct CldrPluralsDataProvider {
    cardinal: Vec<LanguagePluralsPair>,
    ordinal: Vec<LanguagePluralsPair>,
}

impl FromStr for CldrPluralsDataProvider {
    // type Err = json::Error;
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let obj: Resource = serde_json::from_str(s)?;
        let mut result = CldrPluralsDataProvider {
            cardinal: Vec::new(),
            ordinal: Vec::new(),
        };
        result.cardinal = obj.supplemental.plurals_type_cardinal.unwrap().0.iter().map(|(l, r)| {
            let l = if l == &"root" { "und" } else { l };
            LanguagePluralsPair {
                langid: l.parse().unwrap(),
                rules: r.into()
            }
        }).collect();
        // TODO: Ordinal
        Ok(result)
    }

    /*
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let obj = json::parse(s)?;
        let obj = &obj["supplemental"];
        let mut result = CldrPluralsDataProvider {
            cardinal: Vec::new(),
            ordinal: Vec::new(),
        };
        fn convert<'a>(
            (key, value): (&'a str, &'a json::JsonValue),
        ) -> Option<LanguagePluralsPair> {
            if let Ok(langid) = key.parse() {
                if let json::JsonValue::Object(obj) = value {
                    fn convert(value: &json::JsonValue) -> Option<Cow<'static, str>> {
                        value.as_str().map(|s| Cow::Owned(s.to_string()))
                    }
                    Some(LanguagePluralsPair {
                        langid: langid,
                        rules: PluralRuleStringsV1 {
                            zero: obj.get("pluralRule-count-zero").and_then(convert),
                            one: obj.get("pluralRule-count-one").and_then(convert),
                            two: obj.get("pluralRule-count-two").and_then(convert),
                            few: obj.get("pluralRule-count-few").and_then(convert),
                            many: obj.get("pluralRule-count-many").and_then(convert),
                        },
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
        result.cardinal = obj["plurals-type-cardinal"]
            .entries()
            .filter_map(convert)
            .collect();
        result.ordinal = obj["plurals-type-ordinal"]
            .entries()
            .filter_map(convert)
            .collect();
        return Ok(result);
    }
    */
}

impl<'a> DataProvider<'a, 'a> for CldrPluralsDataProvider {
    fn load(&'a self, req: &Request) -> Result<Response<'a>, ResponseError> {
        if req.data_key.category != Category::Plurals {
            return Err(ResponseError::UnsupportedCategoryError(
                req.data_key.category,
            ));
        }
        if req.data_key.version != 1 {
            return Err(ResponseError::UnsupportedDataKeyError(req.data_key));
        }
        let list = match req.data_key.sub_category.as_str() {
            "cardinal" => &self.cardinal,
            "ordinal" => &self.ordinal,
            _ => return Err(ResponseError::UnsupportedDataKeyError(req.data_key)),
        };
        // TODO: Implement language fallback
        let pair = match list.binary_search_by_key(&&req.data_entry.langid, |pair| &pair.langid) {
            Ok(idx) => &list[idx],
            Err(_) => return Err(ResponseError::UnavailableEntryError),
        };
        Ok(ResponseBuilder {
            data_langid: pair.langid.clone(),
        }
        .with_borrowed_payload(&pair.rules))
    }
}

impl<'a> IterableDataProvider<'a> for CldrPluralsDataProvider {
    // TODO: The following works in nightly:
    // type Iter = impl Iterator<Item=DataEntry>;
    type Iter = std::iter::Map<
        std::slice::Iter<'a, LanguagePluralsPair>,
        fn(&LanguagePluralsPair) -> DataEntry,
    >;

    fn iter_for_key(&'a self, data_key: &DataKey) -> Result<Self::Iter, ResponseError> {
        if data_key.category != Category::Plurals {
            return Err(ResponseError::UnsupportedCategoryError(data_key.category));
        }
        fn convert(pair: &LanguagePluralsPair) -> DataEntry {
            DataEntry {
                variant: None,
                langid: pair.langid.clone(),
            }
        }
        match data_key.sub_category.as_str() {
            "cardinal" => Ok(self.cardinal.iter().map(convert)),
            "ordinal" => Ok(self.ordinal.iter().map(convert)),
            _ => Err(ResponseError::UnsupportedDataKeyError(*data_key)),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
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

impl<'a> From<&LocalePluralRules<'a>> for PluralRuleStringsV1 {
    fn from(other: &LocalePluralRules) -> PluralRuleStringsV1 {
        PluralRuleStringsV1 {
            zero: other.zero.map(|s| Cow::Owned(s.to_string())),
            one: other.one.map(|s| Cow::Owned(s.to_string())),
            two: other.two.map(|s| Cow::Owned(s.to_string())),
            few: other.few.map(|s| Cow::Owned(s.to_string())),
            many: other.many.map(|s| Cow::Owned(s.to_string())),
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Rules<'s>(
    #[serde(with = "tuple_vec_map")]
    #[serde(borrow)]
    pub Vec<(&'s str, LocalePluralRules<'s>)>,
);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental<'s> {
    #[serde(rename = "plurals-type-cardinal")]
    #[serde(borrow)]
    pub plurals_type_cardinal: Option<Rules<'s>>,
    #[serde(rename = "plurals-type-ordinal")]
    #[serde(borrow)]
    pub plurals_type_ordinal: Option<Rules<'s>>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource<'s> {
    #[serde(borrow)]
    pub supplemental: Supplemental<'s>,
}

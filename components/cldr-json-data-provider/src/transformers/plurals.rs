use std::borrow::Cow;
use std::str::FromStr;

use json;

use icu_data_provider::plurals::PluralRuleStringsV1;
use icu_data_provider::*;
use icu_locale::LanguageIdentifier;

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
    type Err = json::Error;

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

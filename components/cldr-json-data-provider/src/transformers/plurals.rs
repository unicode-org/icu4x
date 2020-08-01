use std::borrow::Cow;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;

use crate::cldr_langid::CldrLanguage;
use crate::transformers::DataKeySupport;
use crate::CldrPaths;
use crate::Error;
use icu_data_provider::iter::DataEntryCollection;
use icu_data_provider::prelude::*;
use icu_data_provider::structs::plurals::*;
use serde::Deserialize;

#[derive(PartialEq, Debug)]
pub struct CldrPluralsDataProvider<'d> {
    resource: Resource,
    // owned_json: Option<Pin<String>>,
    _phantom: PhantomData<&'d ()>,
}

impl TryFrom<&CldrPaths> for CldrPluralsDataProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &CldrPaths) -> Result<Self, Self::Error> {
        let path = cldr_paths.plurals_json()?;
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(Error::IoError(err, path)),
        };
        let reader = BufReader::new(file);
        let resource: Resource = serde_json::from_reader(reader)?;
        Ok(CldrPluralsDataProvider {
            resource: resource.normalize(),
            _phantom: PhantomData,
        })
    }
}

impl<'d> TryFrom<&'d str> for CldrPluralsDataProvider<'d> {
    type Error = serde_json::error::Error;
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let resource: Resource = serde_json::from_str(s)?;
        Ok(CldrPluralsDataProvider {
            resource: resource.normalize(),
            _phantom: PhantomData,
        })
    }
}

impl<'d> DataKeySupport for CldrPluralsDataProvider<'d> {
    fn supports_key(data_key: &DataKey) -> Result<(), data_provider::Error> {
        if data_key.category != data_key::Category::Plurals {
            return Err((&data_key.category).into());
        }
        if data_key.version != 1 {
            return Err(data_key.into());
        }
        Ok(())
    }
}

impl<'d> CldrPluralsDataProvider<'d> {
    fn get_rules_for(&self, data_key: &DataKey) -> Result<&Rules, data_provider::Error> {
        CldrPluralsDataProvider::supports_key(data_key)?;
        match data_key.sub_category.as_str() {
            "cardinal" => self.resource.supplemental.plurals_type_cardinal.as_ref(),
            "ordinal" => self.resource.supplemental.plurals_type_ordinal.as_ref(),
            _ => return Err(data_key.into()),
        }
        .ok_or_else(|| data_key.into())
    }
}

impl<'d> DataProvider<'d> for CldrPluralsDataProvider<'d> {
    fn load(
        &self,
        req: &data_provider::Request,
    ) -> Result<data_provider::Response<'d>, data_provider::Error> {
        let cldr_rules = self.get_rules_for(&req.data_key)?;
        // TODO: Implement language fallback?
        // TODO: Avoid the clone
        let cldr_langid = CldrLanguage(req.data_entry.langid.clone());
        let (_, r) = match cldr_rules.0.binary_search_by_key(&&cldr_langid, |(l, _)| l) {
            Ok(idx) => &cldr_rules.0[idx],
            Err(_) => return Err(req.data_entry.clone().into()),
        };
        Ok(data_provider::ResponseBuilder {
            data_langid: req.data_entry.langid.clone(),
        }
        .with_owned_payload(PluralRuleStringsV1::from(r)))
    }
}

impl<'d> DataEntryCollection for CldrPluralsDataProvider<'d> {
    fn iter_for_key(
        &self,
        data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, data_provider::Error> {
        let cldr_rules = self.get_rules_for(data_key)?;
        let list: Vec<DataEntry> = cldr_rules
            .0
            .iter()
            .map(|(l, _)| DataEntry {
                variant: None,
                // TODO: Avoid the clone
                langid: l.0.clone(),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&LocalePluralRules> for PluralRuleStringsV1 {
    fn from(other: &LocalePluralRules) -> PluralRuleStringsV1 {
        fn convert<'s>(s: &Cow<'s, str>) -> Cow<'static, str> {
            Cow::Owned(s.to_string())
        }
        PluralRuleStringsV1 {
            zero: other.zero.as_ref().map(convert),
            one: other.one.as_ref().map(convert),
            two: other.two.as_ref().map(convert),
            few: other.few.as_ref().map(convert),
            many: other.many.as_ref().map(convert),
        }
    }
}

// TODO: Use Serde Borrow throughout these structs. Blocked by:
// https://stackoverflow.com/q/63201624/1407170

#[derive(PartialEq, Debug, Deserialize)]
struct LocalePluralRules {
    #[serde(rename = "pluralRule-count-zero")]
    pub zero: Option<Cow<'static, str>>,
    #[serde(rename = "pluralRule-count-one")]
    pub one: Option<Cow<'static, str>>,
    #[serde(rename = "pluralRule-count-two")]
    pub two: Option<Cow<'static, str>>,
    #[serde(rename = "pluralRule-count-few")]
    pub few: Option<Cow<'static, str>>,
    #[serde(rename = "pluralRule-count-many")]
    pub many: Option<Cow<'static, str>>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct Rules(#[serde(with = "tuple_vec_map")] pub Vec<(CldrLanguage, LocalePluralRules)>);

#[derive(PartialEq, Debug, Deserialize)]
struct Supplemental {
    #[serde(rename = "plurals-type-cardinal")]
    pub plurals_type_cardinal: Option<Rules>,
    #[serde(rename = "plurals-type-ordinal")]
    pub plurals_type_ordinal: Option<Rules>,
}

#[derive(PartialEq, Debug, Deserialize)]
struct Resource {
    pub supplemental: Supplemental,
}

impl Resource {
    pub fn normalize(mut self) -> Self {
        // NOTE: We need to sort in order to put "root" -> "und" into place.
        fn sort(rules: &mut Rules) {
            // TODO: Avoid the clone
            rules.0.sort_by_key(|(l, _)| l.0.clone())
        }
        self.supplemental.plurals_type_cardinal.as_mut().map(sort);
        self.supplemental.plurals_type_ordinal.as_mut().map(sort);
        self
    }
}

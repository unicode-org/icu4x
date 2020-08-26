use crate::PluralCategory;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resource<'s> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub supplemental: Supplemental<'s>,
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocalePluralRules<'s> {
    #[cfg_attr(feature = "serde", serde(rename = "pluralRule-count-zero"))]
    pub zero: Option<&'s str>,
    #[cfg_attr(feature = "serde", serde(rename = "pluralRule-count-one"))]
    pub one: Option<&'s str>,
    #[cfg_attr(feature = "serde", serde(rename = "pluralRule-count-two"))]
    pub two: Option<&'s str>,
    #[cfg_attr(feature = "serde", serde(rename = "pluralRule-count-few"))]
    pub few: Option<&'s str>,
    #[cfg_attr(feature = "serde", serde(rename = "pluralRule-count-many"))]
    pub many: Option<&'s str>,
}

impl<'s> LocalePluralRules<'s> {
    #[allow(dead_code)]
    pub fn get(&self, category: &PluralCategory) -> Option<&'s str> {
        match category {
            PluralCategory::Zero => self.zero,
            PluralCategory::One => self.one,
            PluralCategory::Two => self.two,
            PluralCategory::Few => self.few,
            PluralCategory::Many => self.many,
            PluralCategory::Other => None,
        }
    }
}

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rules<'s>(
    #[cfg_attr(feature = "serde", serde(with = "tuple_vec_map"))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub Vec<(&'s str, LocalePluralRules<'s>)>,
);

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Supplemental<'s> {
    #[cfg_attr(feature = "serde", serde(rename = "plurals-type-cardinal"))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub plurals_type_cardinal: Option<Rules<'s>>,
    #[cfg_attr(feature = "serde", serde(rename = "plurals-type-ordinal"))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub plurals_type_ordinal: Option<Rules<'s>>,
}

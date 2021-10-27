#[cfg(any(test, feature = "provider_transform_internals"))]
use crate::error::Error;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Deserialize, serde::Serialize)
)]
pub(crate) struct StringMatcher<'data>(
    #[cfg_attr(feature = "provider_serde", serde(borrow))] Cow<'data, str>,
);

impl<'data> StringMatcher<'data> {
    #[cfg(any(test, feature = "provider_transform_internals"))]
    pub(crate) fn new(pattern: &str) -> Result<Self, Error> {
        let regex = alloc::format!("(?i)^({})", pattern);
        Regex::new(&regex).map_err(Error::IllegalCondition)?;
        Ok(StringMatcher(Cow::Owned(regex)))
    }

    pub(crate) fn test(&self, string: &str) -> bool {
        Regex::new(&self.0).unwrap().is_match(string)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_matcher() {
        let matcher = StringMatcher::new("abc.*").unwrap();
        assert!(!matcher.test("ab"));
        assert!(matcher.test("abc"));
        assert!(matcher.test("abcde"));
    }
}

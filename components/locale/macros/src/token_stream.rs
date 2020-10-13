// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale::subtags;

extern crate proc_macro;

use super::get_crate_name;
use proc_macro::TokenStream;

pub(crate) trait IntoTokenStream {
    fn into_token_stream_string(self) -> String;
    fn into_token_stream(self) -> Result<TokenStream, proc_macro::LexError>
    where
        Self: Sized,
    {
        self.into_token_stream_string().parse()
    }
}

impl IntoTokenStream for u32 {
    fn into_token_stream_string(self) -> String {
        format!("{}", self)
    }
}

impl IntoTokenStream for u64 {
    fn into_token_stream_string(self) -> String {
        format!("{}", self)
    }
}

impl IntoTokenStream for subtags::Language {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ {}::subtags::Language::from_raw_unchecked({}) }}",
            get_crate_name(),
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Script {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ {}::subtags::Script::from_raw_unchecked({}) }}",
            get_crate_name(),
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Region {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ {}::subtags::Region::from_raw_unchecked({}) }}",
            get_crate_name(),
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Variant {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ {}::subtags::Variant::from_raw_unchecked({}) }}",
            get_crate_name(),
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Variants {
    fn into_token_stream_string(self) -> String {
        let variants = if let Some(variants) = self.into_raw() {
            let v: Vec<_> = variants
                .iter()
                .map(|v| v.into_token_stream_string())
                .collect();
            format!("Some(Box::new([{}]))", v.join(", "))
        } else {
            "None".to_string()
        };
        format!(
            "unsafe {{ {}::subtags::Variants::from_raw_unchecked({}) }}",
            get_crate_name(),
            variants
        )
    }
}

impl<T> IntoTokenStream for Option<T>
where
    T: IntoTokenStream,
{
    fn into_token_stream_string(self) -> String {
        if let Some(v) = self {
            format!("Some({})", v.into_token_stream_string())
        } else {
            "None".to_string()
        }
    }
}

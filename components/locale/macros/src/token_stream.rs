use icu_locale::subtags;
use tinystr::{TinyStr4, TinyStr8};

extern crate proc_macro;

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

impl IntoTokenStream for TinyStr8 {
    fn into_token_stream_string(self) -> String {
        format!(
            "tinystr::TinyStr8::new_unchecked({})",
            Into::<u64>::into(self)
        )
    }
}

impl IntoTokenStream for TinyStr4 {
    fn into_token_stream_string(self) -> String {
        format!(
            "tinystr::TinyStr4::new_unchecked({})",
            Into::<u32>::into(self)
        )
    }
}

impl IntoTokenStream for subtags::Language {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ icu_locale::subtags::Language::from_raw_unchecked({}) }}",
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Script {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ icu_locale::subtags::Script::from_raw_unchecked({}) }}",
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Region {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ icu_locale::subtags::Region::from_raw_unchecked({}) }}",
            self.into_raw().into_token_stream_string()
        )
    }
}

impl IntoTokenStream for subtags::Variant {
    fn into_token_stream_string(self) -> String {
        format!(
            "unsafe {{ icu_locale::subtags::Variant::from_raw_unchecked({}) }}",
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
            "unsafe {{ icu_locale::subtags::Variants::from_raw_unchecked({}) }}",
            variants
        )
    }
}

impl<T> IntoTokenStream for Option<T>
where T: IntoTokenStream {
    fn into_token_stream_string(self) -> String {
         if let Some(v) = self {
            format!("Some({})", v.into_token_stream_string())
        } else {
            "None".to_string()
        }
    }
}

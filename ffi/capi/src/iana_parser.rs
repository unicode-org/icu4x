// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::timezone::ffi::TimeZoneInfo;
    #[cfg(feature = "buffer_provider")]
    use crate::{errors::ffi::DataError, provider::ffi::DataProvider};

    /// A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
    ///
    /// This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
    /// It also supports normalizing and canonicalizing the IANA strings.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParser, Struct)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParser::as_borrowed, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserBorrowed, Struct, hidden)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserBorrowed::new, FnInStruct, hidden)]
    pub struct IanaParser(pub icu_time::zone::iana::IanaParser);

    impl IanaParser {
        /// Create a new [`IanaParser`] using compiled data
        #[diplomat::rust_link(icu::time::zone::iana::IanaParser::new, FnInStruct)]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<IanaParser> {
            Box::new(IanaParser(
                icu_time::zone::iana::IanaParser::new().static_to_owned(),
            ))
        }

        /// Create a new [`IanaParser`] using a particular data source
        #[diplomat::rust_link(icu::time::zone::iana::IanaParser::new, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(provider: &DataProvider) -> Result<Box<IanaParser>, DataError> {
            Ok(Box::new(IanaParser(
                icu_time::zone::iana::IanaParser::try_new_with_buffer_provider(provider.get()?)?,
            )))
        }

        #[diplomat::rust_link(icu::time::zone::iana::IanaParserBorrowed::parse, FnInStruct)]
        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserBorrowed::parse_from_utf8,
            FnInStruct,
            hidden
        )]
        pub fn parse(&self, value: &DiplomatStr) -> Box<TimeZoneInfo> {
            Box::new(TimeZoneInfo {
                time_zone_id: self.0.as_borrowed().parse_from_utf8(value),
                local_time: None,
                offset: None,
                zone_variant: None,
            })
        }
    }
}

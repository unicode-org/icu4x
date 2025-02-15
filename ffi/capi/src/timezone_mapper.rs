// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use writeable::Writeable;

    #[cfg(feature = "buffer_provider")]
    use crate::{errors::ffi::DataError, provider::ffi::DataProvider};

    use tinystr::TinyAsciiStr;

    /// A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
    ///
    /// This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
    /// It also supports normalizing and canonicalizing the IANA strings.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParser, Struct)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParser::as_borrowed, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserBorrowed, Struct, hidden)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserBorrowed::new, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::time::NormalizedIana, Struct, hidden)]
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

        #[diplomat::rust_link(icu::time::zone::iana::IanaParserBorrowed::iana_to_bcp47, FnInStruct)]
        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserBorrowed::iana_bytes_to_bcp47,
            FnInStruct,
            hidden
        )]
        pub fn iana_to_bcp47(
            &self,
            value: &DiplomatStr,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let handle = self.0.as_borrowed();
            let bcp47 = handle.iana_bytes_to_bcp47(value);
            let _infallible = bcp47.0.write_to(write);
        }

        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserBorrowed::normalize_iana,
            FnInStruct
        )]
        pub fn normalize_iana(
            &self,
            value: &str,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Option<()> {
            let handle = self.0.as_borrowed();
            let iana = handle.normalize_iana(value)?;
            let _infallible = iana.0.write_to(write);
            Some(())
        }

        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserBorrowed::canonicalize_iana,
            FnInStruct
        )]
        pub fn canonicalize_iana(
            &self,
            value: &str,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Option<()> {
            let handle = self.0.as_borrowed();
            let iana = handle.canonicalize_iana(value)?;
            let _infallible = iana.0.write_to(write);
            Some(())
        }

        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserBorrowed::find_canonical_iana_from_bcp47,
            FnInStruct
        )]
        pub fn find_canonical_iana_from_bcp47(
            &self,
            value: &DiplomatStr,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Option<()> {
            let handle = self.0.as_borrowed();
            let iana = TinyAsciiStr::try_from_utf8(value)
                .ok()
                .and_then(|s| handle.find_canonical_iana_from_bcp47(icu_time::TimeZone(s)))?;
            let _infallible = iana.write_to(write);
            Some(())
        }
    }

    /// A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
    ///
    /// This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
    /// It also supports normalizing and canonicalizing the IANA strings.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtended, Struct)]
    #[diplomat::rust_link(
        icu::time::zone::iana::IanaParserExtended::as_borrowed,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtended::inner, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtendedBorrowed, Struct, hidden)]
    #[diplomat::rust_link(
        icu::time::zone::iana::IanaParserExtendedBorrowed::inner,
        FnInStruct,
        hidden
    )]
    pub struct IanaParserExtended(
        pub icu_time::zone::iana::IanaParserExtended<icu_time::zone::iana::IanaParser>,
    );

    impl IanaParserExtended {
        /// Create a new [`IanaParserExtended`] using compiled data
        #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtended::new, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtendedBorrowed::new, FnInStruct)]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<IanaParserExtended> {
            Box::new(IanaParserExtended(
                icu_time::zone::iana::IanaParserExtended::new().static_to_owned(),
            ))
        }
        /// Create a new [`IanaParserExtended`] using a particular data source
        #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtended::new, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::iana::IanaParserExtendedBorrowed::new, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<IanaParserExtended>, DataError> {
            Ok(Box::new(IanaParserExtended(
                icu_time::zone::iana::IanaParserExtended::try_new_with_buffer_provider(
                    provider.get()?,
                )?,
            )))
        }

        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserExtendedBorrowed::canonicalize_iana,
            FnInStruct
        )]
        pub fn canonicalize_iana(
            &self,
            value: &str,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Option<()> {
            let handle = self.0.as_borrowed();
            let iana = handle.canonicalize_iana(value)?;
            let _infallible = iana.0.write_to(write);
            Some(())
        }

        #[diplomat::rust_link(
            icu::time::zone::iana::IanaParserExtendedBorrowed::canonical_iana_from_bcp47,
            FnInStruct
        )]
        pub fn canonical_iana_from_bcp47(
            &self,
            value: &DiplomatStr,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Option<()> {
            let handle = self.0.as_borrowed();
            let iana = TinyAsciiStr::try_from_utf8(value)
                .ok()
                .map(icu_time::TimeZone)
                .and_then(|t| handle.canonical_iana_from_bcp47(t))?;
            let _infallible = iana.write_to(write);
            Some(())
        }
    }
}

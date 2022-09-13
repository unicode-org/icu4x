// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_properties::{provider, script, Script};
    use icu_provider::DataPayload;

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    /// An ICU4X ScriptWithExtensions map object, capable of holding a map of codepoints to scriptextensions values
    #[diplomat::rust_link(icu::properties::script::ScriptWithExtensions, Struct)]
    pub struct ICU4XScriptWithExtensionsSet(
        pub DataPayload<provider::ScriptWithExtensionsPropertyV1Marker>,
    );

    impl ICU4XScriptWithExtensionsSet {
        #[diplomat::rust_link(icu::properties::script::load_script_with_extensions_unstable, Fn)]
        pub fn load(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XScriptWithExtensionsSet>, ICU4XError> {
            script::load_script_with_extensions_unstable(&provider.0)
                .map(|data| Box::new(ICU4XScriptWithExtensionsSet(data)))
                .map_err(Into::into)
                .into()
        }

        /// Get the Script property value for a code point
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensions::get_script_val,
            FnInStruct
        )]
        pub fn get_script_val(&self, code_point: u32) -> u16 {
            self.0.get().data.get_script_val(code_point).0
        }

        /// Check if the Script_Extensions property of the given code point covers the given script
        #[diplomat::rust_link(
            icu::properties::script::ScriptWithExtensions::has_script,
            FnInStruct
        )]
        pub fn has_script(&self, code_point: u32, script: u16) -> bool {
            self.0.get().data.has_script(code_point, Script(script))
        }
    }
}

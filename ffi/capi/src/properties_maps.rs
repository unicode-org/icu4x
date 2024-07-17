// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "ICU4X{0}"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::properties_iter::ffi::CodePointRangeIterator;
    use crate::properties_sets::ffi::CodePointSetData;
    use crate::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
    ///
    /// For properties whose values fit into 8 bits.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapData, Struct)]
    #[diplomat::rust_link(
        icu::properties::maps::CodePointMapData::try_into_converted,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed, Struct)]
    pub struct CodePointMapData8(icu_properties::maps::CodePointMapData<u8>);

    fn convert_8<P: icu_collections::codepointtrie::TrieValue>(
        data: icu_properties::maps::CodePointMapData<P>,
    ) -> Box<CodePointMapData8> {
        #[allow(clippy::unwrap_used)] // infallible for the chosen properties
        Box::new(CodePointMapData8(
            data.try_into_converted().map_err(|_| ()).unwrap(),
        ))
    }

    impl CodePointMapData8 {
        /// Gets the value for a code point.
        #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed::get, FnInStruct)]
        #[diplomat::attr(supports = indexing, indexer)]
        pub fn get(&self, cp: DiplomatChar) -> u8 {
            self.0.as_borrowed().get32(cp)
        }

        /// Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::get32,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(any(dart, js), disable)]
        pub fn get32(&self, cp: u32) -> u8 {
            self.get(cp)
        }

        /// Converts a general category to its corresponding mask value
        ///
        /// Nonexistent general categories will map to the empty mask
        #[diplomat::rust_link(icu::properties::GeneralCategoryGroup, Struct)]
        pub fn general_category_to_mask(gc: u8) -> u32 {
            if let Ok(gc) = icu_properties::GeneralCategory::try_from(gc) {
                let group: icu_properties::GeneralCategoryGroup = gc.into();
                group.into()
            } else {
                0
            }
        }

        /// Produces an iterator over ranges of code points that map to `value`
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::iter_ranges_for_value,
            FnInStruct
        )]
        pub fn iter_ranges_for_value<'a>(&'a self, value: u8) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0.as_borrowed().iter_ranges_for_value(value),
            )))
        }

        /// Produces an iterator over ranges of code points that do not map to `value`
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::iter_ranges_for_value_complemented,
            FnInStruct
        )]
        pub fn iter_ranges_for_value_complemented<'a>(
            &'a self,
            value: u8,
        ) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0
                    .as_borrowed()
                    .iter_ranges_for_value_complemented(value),
            )))
        }

        /// Given a mask value (the nth bit marks property value = n), produce an iterator over ranges of code points
        /// whose property values are contained in the mask.
        ///
        /// The main mask property supported is that for General_Category, which can be obtained via `general_category_to_mask()` or
        /// by using `GeneralCategoryNameToMaskMapper`
        ///
        /// Should only be used on maps for properties with values less than 32 (like Generak_Category),
        /// other maps will have unpredictable results
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::iter_ranges_for_group,
            FnInStruct
        )]
        pub fn iter_ranges_for_mask<'a>(&'a self, mask: u32) -> Box<CodePointRangeIterator<'a>> {
            let ranges = self
                .0
                .as_borrowed()
                .iter_ranges_mapped(move |v| {
                    let val_mask = 1_u32.checked_shl(v.into()).unwrap_or(0);
                    val_mask & mask != 0
                })
                .filter(|v| v.value)
                .map(|v| v.range);
            Box::new(CodePointRangeIterator(Box::new(ranges)))
        }

        /// Gets a [`CodePointSetData`] representing all entries in this map that map to the given value
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::get_set_for_value,
            FnInStruct
        )]
        pub fn get_set_for_value(&self, value: u8) -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                self.0.as_borrowed().get_set_for_value(value),
            ))
        }

        #[diplomat::rust_link(icu::properties::maps::general_category, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_general_category, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "general_category")]
        pub fn load_general_category(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::general_category [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_general_category,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::bidi_class, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_bidi_class, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "bidi_class")]
        pub fn load_bidi_class(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::bidi_class [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_bidi_class,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::east_asian_width, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_east_asian_width, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "east_asian_width")]
        pub fn load_east_asian_width(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::east_asian_width [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_east_asian_width,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::hangul_syllable_type, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_hangul_syllable_type, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "hangul_syllable_type")]
        pub fn load_hangul_syllable_type(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::hangul_syllable_type [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_hangul_syllable_type,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::indic_syllabic_category, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_indic_syllabic_category, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "indic_syllabic_category")]
        pub fn load_indic_syllabic_category(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::indic_syllabic_category [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_indic_syllabic_category,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::line_break, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_line_break, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "line_break")]
        pub fn load_line_break(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::line_break [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_line_break,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::grapheme_cluster_break, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_grapheme_cluster_break, Fn, hidden)]
        pub fn try_grapheme_cluster_break(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::grapheme_cluster_break [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_grapheme_cluster_break,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::word_break, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_word_break, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "word_break")]
        pub fn load_word_break(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::word_break [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_word_break,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::sentence_break, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_sentence_break, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "sentence_break")]
        pub fn load_sentence_break(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::sentence_break [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_sentence_break,
                provider,
            )?))
        }

        #[diplomat::rust_link(icu::properties::maps::joining_type, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_joining_type, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "joining_type")]
        pub fn load_joining_type(
            provider: &DataProvider,
        ) -> Result<Box<CodePointMapData8>, DataError> {
            Ok(convert_8(call_constructor_unstable!(
                icu_properties::maps::joining_type [r => Ok(r.static_to_owned())],
                icu_properties::maps::load_joining_type,
                provider,
            )?))
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
    ///
    /// For properties whose values fit into 16 bits.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapData, Struct)]
    #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed, Struct)]
    pub struct CodePointMapData16(icu_properties::maps::CodePointMapData<u16>);

    impl CodePointMapData16 {
        /// Gets the value for a code point.
        #[diplomat::rust_link(icu::properties::maps::CodePointMapDataBorrowed::get, FnInStruct)]
        #[diplomat::attr(supports = indexing, indexer)]
        pub fn get(&self, cp: DiplomatChar) -> u16 {
            self.0.as_borrowed().get32(cp)
        }

        /// Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::get32,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(any(dart, js), disable)]
        pub fn get32(&self, cp: u32) -> u16 {
            self.get(cp)
        }

        /// Produces an iterator over ranges of code points that map to `value`
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::iter_ranges_for_value,
            FnInStruct
        )]
        pub fn iter_ranges_for_value<'a>(&'a self, value: u16) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0.as_borrowed().iter_ranges_for_value(value),
            )))
        }

        /// Produces an iterator over ranges of code points that do not map to `value`
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::iter_ranges_for_value_complemented,
            FnInStruct
        )]
        pub fn iter_ranges_for_value_complemented<'a>(
            &'a self,
            value: u16,
        ) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0
                    .as_borrowed()
                    .iter_ranges_for_value_complemented(value),
            )))
        }

        /// Gets a [`CodePointSetData`] representing all entries in this map that map to the given value
        #[diplomat::rust_link(
            icu::properties::maps::CodePointMapDataBorrowed::get_set_for_value,
            FnInStruct
        )]
        pub fn get_set_for_value(&self, value: u16) -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                self.0.as_borrowed().get_set_for_value(value),
            ))
        }

        #[diplomat::rust_link(icu::properties::maps::script, Fn)]
        #[diplomat::rust_link(icu::properties::maps::load_script, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "script")]
        pub fn load_script(provider: &DataProvider) -> Result<Box<CodePointMapData16>, DataError> {
            #[allow(clippy::unwrap_used)] // script is a 16-bit property
            Ok(Box::new(CodePointMapData16(
                call_constructor_unstable!(
                    icu_properties::maps::script [r => Ok(r.static_to_owned())],
                    icu_properties::maps::load_script,
                    provider,
                )?
                .try_into_converted()
                .map_err(|_| ())
                .unwrap(),
            )))
        }
    }
}

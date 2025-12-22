package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CodePointSetDataLib: Library {
    fun icu4x_CodePointSetData_destroy_mv1(handle: Pointer)
    fun icu4x_CodePointSetData_contains_mv1(handle: Pointer, cp: Int): Byte
    fun icu4x_CodePointSetData_iter_ranges_mv1(handle: Pointer): Pointer
    fun icu4x_CodePointSetData_iter_ranges_complemented_mv1(handle: Pointer): Pointer
    fun icu4x_CodePointSetData_create_general_category_group_mv1(group: GeneralCategoryGroupNative): Pointer
    fun icu4x_CodePointSetData_create_general_category_group_with_provider_mv1(provider: Pointer, group: FFIUint32): ResultPointerInt
    fun icu4x_CodePointSetData_ascii_hex_digit_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_ascii_hex_digit_mv1(): Pointer
    fun icu4x_CodePointSetData_create_ascii_hex_digit_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_alnum_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_alnum_mv1(): Pointer
    fun icu4x_CodePointSetData_create_alnum_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_alphabetic_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_alphabetic_mv1(): Pointer
    fun icu4x_CodePointSetData_create_alphabetic_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_bidi_control_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_bidi_control_mv1(): Pointer
    fun icu4x_CodePointSetData_create_bidi_control_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_bidi_mirrored_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_bidi_mirrored_mv1(): Pointer
    fun icu4x_CodePointSetData_create_bidi_mirrored_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_blank_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_blank_mv1(): Pointer
    fun icu4x_CodePointSetData_create_blank_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_cased_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_cased_mv1(): Pointer
    fun icu4x_CodePointSetData_create_cased_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_case_ignorable_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_case_ignorable_mv1(): Pointer
    fun icu4x_CodePointSetData_create_case_ignorable_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_full_composition_exclusion_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_full_composition_exclusion_mv1(): Pointer
    fun icu4x_CodePointSetData_create_full_composition_exclusion_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_changes_when_casefolded_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_changes_when_casefolded_mv1(): Pointer
    fun icu4x_CodePointSetData_create_changes_when_casefolded_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_changes_when_casemapped_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_changes_when_casemapped_mv1(): Pointer
    fun icu4x_CodePointSetData_create_changes_when_casemapped_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_changes_when_nfkc_casefolded_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_changes_when_nfkc_casefolded_mv1(): Pointer
    fun icu4x_CodePointSetData_create_changes_when_nfkc_casefolded_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_changes_when_lowercased_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_changes_when_lowercased_mv1(): Pointer
    fun icu4x_CodePointSetData_create_changes_when_lowercased_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_changes_when_titlecased_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_changes_when_titlecased_mv1(): Pointer
    fun icu4x_CodePointSetData_create_changes_when_titlecased_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_changes_when_uppercased_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_changes_when_uppercased_mv1(): Pointer
    fun icu4x_CodePointSetData_create_changes_when_uppercased_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_dash_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_dash_mv1(): Pointer
    fun icu4x_CodePointSetData_create_dash_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_deprecated_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_deprecated_mv1(): Pointer
    fun icu4x_CodePointSetData_create_deprecated_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_default_ignorable_code_point_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_default_ignorable_code_point_mv1(): Pointer
    fun icu4x_CodePointSetData_create_default_ignorable_code_point_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_diacritic_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_diacritic_mv1(): Pointer
    fun icu4x_CodePointSetData_create_diacritic_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_emoji_modifier_base_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_emoji_modifier_base_mv1(): Pointer
    fun icu4x_CodePointSetData_create_emoji_modifier_base_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_emoji_component_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_emoji_component_mv1(): Pointer
    fun icu4x_CodePointSetData_create_emoji_component_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_emoji_modifier_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_emoji_modifier_mv1(): Pointer
    fun icu4x_CodePointSetData_create_emoji_modifier_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_emoji_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_emoji_mv1(): Pointer
    fun icu4x_CodePointSetData_create_emoji_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_emoji_presentation_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_emoji_presentation_mv1(): Pointer
    fun icu4x_CodePointSetData_create_emoji_presentation_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_extender_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_extender_mv1(): Pointer
    fun icu4x_CodePointSetData_create_extender_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_extended_pictographic_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_extended_pictographic_mv1(): Pointer
    fun icu4x_CodePointSetData_create_extended_pictographic_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_graph_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_graph_mv1(): Pointer
    fun icu4x_CodePointSetData_create_graph_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_grapheme_base_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_grapheme_base_mv1(): Pointer
    fun icu4x_CodePointSetData_create_grapheme_base_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_grapheme_extend_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_grapheme_extend_mv1(): Pointer
    fun icu4x_CodePointSetData_create_grapheme_extend_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_grapheme_link_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_grapheme_link_mv1(): Pointer
    fun icu4x_CodePointSetData_create_grapheme_link_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_hex_digit_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_hex_digit_mv1(): Pointer
    fun icu4x_CodePointSetData_create_hex_digit_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_hyphen_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_hyphen_mv1(): Pointer
    fun icu4x_CodePointSetData_create_hyphen_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_id_compat_math_continue_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_id_compat_math_continue_mv1(): Pointer
    fun icu4x_CodePointSetData_create_id_compat_math_continue_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_id_compat_math_start_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_id_compat_math_start_mv1(): Pointer
    fun icu4x_CodePointSetData_create_id_compat_math_start_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_id_continue_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_id_continue_mv1(): Pointer
    fun icu4x_CodePointSetData_create_id_continue_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_ideographic_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_ideographic_mv1(): Pointer
    fun icu4x_CodePointSetData_create_ideographic_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_id_start_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_id_start_mv1(): Pointer
    fun icu4x_CodePointSetData_create_id_start_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_ids_binary_operator_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_ids_binary_operator_mv1(): Pointer
    fun icu4x_CodePointSetData_create_ids_binary_operator_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_ids_trinary_operator_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_ids_trinary_operator_mv1(): Pointer
    fun icu4x_CodePointSetData_create_ids_trinary_operator_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_ids_unary_operator_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_ids_unary_operator_mv1(): Pointer
    fun icu4x_CodePointSetData_create_ids_unary_operator_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_join_control_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_join_control_mv1(): Pointer
    fun icu4x_CodePointSetData_create_join_control_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_logical_order_exception_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_logical_order_exception_mv1(): Pointer
    fun icu4x_CodePointSetData_create_logical_order_exception_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_lowercase_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_lowercase_mv1(): Pointer
    fun icu4x_CodePointSetData_create_lowercase_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_math_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_math_mv1(): Pointer
    fun icu4x_CodePointSetData_create_math_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_modifier_combining_mark_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_modifier_combining_mark_mv1(): Pointer
    fun icu4x_CodePointSetData_create_modifier_combining_mark_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_noncharacter_code_point_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_noncharacter_code_point_mv1(): Pointer
    fun icu4x_CodePointSetData_create_noncharacter_code_point_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_nfc_inert_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_nfc_inert_mv1(): Pointer
    fun icu4x_CodePointSetData_create_nfc_inert_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_nfd_inert_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_nfd_inert_mv1(): Pointer
    fun icu4x_CodePointSetData_create_nfd_inert_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_nfkc_inert_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_nfkc_inert_mv1(): Pointer
    fun icu4x_CodePointSetData_create_nfkc_inert_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_nfkd_inert_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_nfkd_inert_mv1(): Pointer
    fun icu4x_CodePointSetData_create_nfkd_inert_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_pattern_syntax_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_pattern_syntax_mv1(): Pointer
    fun icu4x_CodePointSetData_create_pattern_syntax_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_pattern_white_space_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_pattern_white_space_mv1(): Pointer
    fun icu4x_CodePointSetData_create_pattern_white_space_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_prepended_concatenation_mark_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_prepended_concatenation_mark_mv1(): Pointer
    fun icu4x_CodePointSetData_create_prepended_concatenation_mark_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_print_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_print_mv1(): Pointer
    fun icu4x_CodePointSetData_create_print_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_quotation_mark_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_quotation_mark_mv1(): Pointer
    fun icu4x_CodePointSetData_create_quotation_mark_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_radical_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_radical_mv1(): Pointer
    fun icu4x_CodePointSetData_create_radical_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_regional_indicator_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_regional_indicator_mv1(): Pointer
    fun icu4x_CodePointSetData_create_regional_indicator_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_soft_dotted_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_soft_dotted_mv1(): Pointer
    fun icu4x_CodePointSetData_create_soft_dotted_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_segment_starter_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_segment_starter_mv1(): Pointer
    fun icu4x_CodePointSetData_create_segment_starter_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_case_sensitive_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_case_sensitive_mv1(): Pointer
    fun icu4x_CodePointSetData_create_case_sensitive_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_sentence_terminal_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_sentence_terminal_mv1(): Pointer
    fun icu4x_CodePointSetData_create_sentence_terminal_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_terminal_punctuation_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_terminal_punctuation_mv1(): Pointer
    fun icu4x_CodePointSetData_create_terminal_punctuation_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_unified_ideograph_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_unified_ideograph_mv1(): Pointer
    fun icu4x_CodePointSetData_create_unified_ideograph_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_uppercase_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_uppercase_mv1(): Pointer
    fun icu4x_CodePointSetData_create_uppercase_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_variation_selector_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_variation_selector_mv1(): Pointer
    fun icu4x_CodePointSetData_create_variation_selector_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_white_space_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_white_space_mv1(): Pointer
    fun icu4x_CodePointSetData_create_white_space_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_xdigit_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_xdigit_mv1(): Pointer
    fun icu4x_CodePointSetData_create_xdigit_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_xid_continue_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_xid_continue_mv1(): Pointer
    fun icu4x_CodePointSetData_create_xid_continue_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_xid_start_for_char_mv1(ch: Int): Byte
    fun icu4x_CodePointSetData_create_xid_start_mv1(): Pointer
    fun icu4x_CodePointSetData_create_xid_start_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointSetData_create_for_ecma262_mv1(propertyName: Slice): ResultPointerInt
    fun icu4x_CodePointSetData_create_for_ecma262_with_provider_mv1(provider: Pointer, propertyName: Slice): ResultPointerInt
}
/** An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
*
*See the [Rust documentation for `properties`](https://docs.rs/icu/2.1.1/icu/properties/index.html) for more information.
*
*See the [Rust documentation for `CodePointSetData`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetData.html) for more information.
*
*See the [Rust documentation for `CodePointSetDataBorrowed`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetDataBorrowed.html) for more information.
*/
class CodePointSetData internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CodePointSetDataCleaner(val handle: Pointer, val lib: CodePointSetDataLib) : Runnable {
        override fun run() {
            lib.icu4x_CodePointSetData_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CodePointSetDataLib> = CodePointSetDataLib::class.java
        internal val lib: CodePointSetDataLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Produces a set for obtaining General Category Group values
        *which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C, using compiled data.
        *
        *See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
        *
        *See the [Rust documentation for `get_set_for_value_group`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.get_set_for_value_group) for more information.
        */
        fun createGeneralCategoryGroup(group: GeneralCategoryGroup): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_general_category_group_mv1(group.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Produces a set for obtaining General Category Group values
        *which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C, using a provided data source.
        *
        *See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
        *
        *See the [Rust documentation for `get_set_for_value_group`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.get_set_for_value_group) for more information.
        */
        fun createGeneralCategoryGroupWithProvider(provider: DataProvider, group: UInt): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_general_category_group_with_provider_mv1(provider.handle, FFIUint32(group));
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Ascii_Hex_Digit` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun asciiHexDigitForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_ascii_hex_digit_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Ascii_Hex_Digit` property, using compiled data.
        *
        *See the [Rust documentation for `AsciiHexDigit`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.AsciiHexDigit.html) for more information.
        */
        fun createAsciiHexDigit(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ascii_hex_digit_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Ascii_Hex_Digit` property, using a particular data source.
        *
        *See the [Rust documentation for `AsciiHexDigit`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.AsciiHexDigit.html) for more information.
        */
        fun createAsciiHexDigitWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ascii_hex_digit_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Alnum` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun alnumForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_alnum_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Alnum` property, using compiled data.
        *
        *See the [Rust documentation for `Alnum`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Alnum.html) for more information.
        */
        fun createAlnum(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_alnum_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Alnum` property, using a particular data source.
        *
        *See the [Rust documentation for `Alnum`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Alnum.html) for more information.
        */
        fun createAlnumWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_alnum_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Alphabetic` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun alphabeticForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_alphabetic_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Alphabetic` property, using compiled data.
        *
        *See the [Rust documentation for `Alphabetic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Alphabetic.html) for more information.
        */
        fun createAlphabetic(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_alphabetic_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Alphabetic` property, using a particular data source.
        *
        *See the [Rust documentation for `Alphabetic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Alphabetic.html) for more information.
        */
        fun createAlphabeticWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_alphabetic_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Bidi_Control` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun bidiControlForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_bidi_control_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Bidi_Control` property, using compiled data.
        *
        *See the [Rust documentation for `BidiControl`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiControl.html) for more information.
        */
        fun createBidiControl(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_bidi_control_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Bidi_Control` property, using a particular data source.
        *
        *See the [Rust documentation for `BidiControl`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiControl.html) for more information.
        */
        fun createBidiControlWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_bidi_control_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Bidi_Mirrored` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun bidiMirroredForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_bidi_mirrored_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Bidi_Mirrored` property, using compiled data.
        *
        *See the [Rust documentation for `BidiMirrored`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiMirrored.html) for more information.
        */
        fun createBidiMirrored(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_bidi_mirrored_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Bidi_Mirrored` property, using a particular data source.
        *
        *See the [Rust documentation for `BidiMirrored`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiMirrored.html) for more information.
        */
        fun createBidiMirroredWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_bidi_mirrored_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Blank` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun blankForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_blank_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Blank` property, using compiled data.
        *
        *See the [Rust documentation for `Blank`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Blank.html) for more information.
        */
        fun createBlank(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_blank_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Blank` property, using a particular data source.
        *
        *See the [Rust documentation for `Blank`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Blank.html) for more information.
        */
        fun createBlankWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_blank_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Cased` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun casedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_cased_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Cased` property, using compiled data.
        *
        *See the [Rust documentation for `Cased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Cased.html) for more information.
        */
        fun createCased(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_cased_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Cased` property, using a particular data source.
        *
        *See the [Rust documentation for `Cased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Cased.html) for more information.
        */
        fun createCasedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_cased_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Case_Ignorable` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun caseIgnorableForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_case_ignorable_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Case_Ignorable` property, using compiled data.
        *
        *See the [Rust documentation for `CaseIgnorable`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CaseIgnorable.html) for more information.
        */
        fun createCaseIgnorable(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_case_ignorable_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Case_Ignorable` property, using a particular data source.
        *
        *See the [Rust documentation for `CaseIgnorable`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CaseIgnorable.html) for more information.
        */
        fun createCaseIgnorableWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_case_ignorable_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Full_Composition_Exclusion` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun fullCompositionExclusionForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_full_composition_exclusion_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Full_Composition_Exclusion` property, using compiled data.
        *
        *See the [Rust documentation for `FullCompositionExclusion`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.FullCompositionExclusion.html) for more information.
        */
        fun createFullCompositionExclusion(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_full_composition_exclusion_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Full_Composition_Exclusion` property, using a particular data source.
        *
        *See the [Rust documentation for `FullCompositionExclusion`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.FullCompositionExclusion.html) for more information.
        */
        fun createFullCompositionExclusionWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_full_composition_exclusion_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Changes_When_Casefolded` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun changesWhenCasefoldedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_changes_when_casefolded_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Casefolded` property, using compiled data.
        *
        *See the [Rust documentation for `ChangesWhenCasefolded`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenCasefolded.html) for more information.
        */
        fun createChangesWhenCasefolded(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_casefolded_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Casefolded` property, using a particular data source.
        *
        *See the [Rust documentation for `ChangesWhenCasefolded`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenCasefolded.html) for more information.
        */
        fun createChangesWhenCasefoldedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_casefolded_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Changes_When_Casemapped` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun changesWhenCasemappedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_changes_when_casemapped_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Casemapped` property, using compiled data.
        *
        *See the [Rust documentation for `ChangesWhenCasemapped`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenCasemapped.html) for more information.
        */
        fun createChangesWhenCasemapped(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_casemapped_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Casemapped` property, using a particular data source.
        *
        *See the [Rust documentation for `ChangesWhenCasemapped`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenCasemapped.html) for more information.
        */
        fun createChangesWhenCasemappedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_casemapped_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Changes_When_Nfkc_Casefolded` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun changesWhenNfkcCasefoldedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_changes_when_nfkc_casefolded_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Nfkc_Casefolded` property, using compiled data.
        *
        *See the [Rust documentation for `ChangesWhenNfkcCasefolded`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenNfkcCasefolded.html) for more information.
        */
        fun createChangesWhenNfkcCasefolded(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_nfkc_casefolded_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Nfkc_Casefolded` property, using a particular data source.
        *
        *See the [Rust documentation for `ChangesWhenNfkcCasefolded`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenNfkcCasefolded.html) for more information.
        */
        fun createChangesWhenNfkcCasefoldedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_nfkc_casefolded_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Changes_When_Lowercased` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun changesWhenLowercasedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_changes_when_lowercased_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Lowercased` property, using compiled data.
        *
        *See the [Rust documentation for `ChangesWhenLowercased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenLowercased.html) for more information.
        */
        fun createChangesWhenLowercased(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_lowercased_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Lowercased` property, using a particular data source.
        *
        *See the [Rust documentation for `ChangesWhenLowercased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenLowercased.html) for more information.
        */
        fun createChangesWhenLowercasedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_lowercased_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Changes_When_Titlecased` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun changesWhenTitlecasedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_changes_when_titlecased_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Titlecased` property, using compiled data.
        *
        *See the [Rust documentation for `ChangesWhenTitlecased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenTitlecased.html) for more information.
        */
        fun createChangesWhenTitlecased(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_titlecased_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Titlecased` property, using a particular data source.
        *
        *See the [Rust documentation for `ChangesWhenTitlecased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenTitlecased.html) for more information.
        */
        fun createChangesWhenTitlecasedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_titlecased_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Changes_When_Uppercased` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun changesWhenUppercasedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_changes_when_uppercased_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Uppercased` property, using compiled data.
        *
        *See the [Rust documentation for `ChangesWhenUppercased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenUppercased.html) for more information.
        */
        fun createChangesWhenUppercased(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_uppercased_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Changes_When_Uppercased` property, using a particular data source.
        *
        *See the [Rust documentation for `ChangesWhenUppercased`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ChangesWhenUppercased.html) for more information.
        */
        fun createChangesWhenUppercasedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_changes_when_uppercased_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Dash` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun dashForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_dash_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Dash` property, using compiled data.
        *
        *See the [Rust documentation for `Dash`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Dash.html) for more information.
        */
        fun createDash(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_dash_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Dash` property, using a particular data source.
        *
        *See the [Rust documentation for `Dash`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Dash.html) for more information.
        */
        fun createDashWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_dash_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Deprecated` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun deprecatedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_deprecated_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Deprecated` property, using compiled data.
        *
        *See the [Rust documentation for `Deprecated`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Deprecated.html) for more information.
        */
        fun createDeprecated(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_deprecated_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Deprecated` property, using a particular data source.
        *
        *See the [Rust documentation for `Deprecated`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Deprecated.html) for more information.
        */
        fun createDeprecatedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_deprecated_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Default_Ignorable_Code_Point` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun defaultIgnorableCodePointForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_default_ignorable_code_point_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Default_Ignorable_Code_Point` property, using compiled data.
        *
        *See the [Rust documentation for `DefaultIgnorableCodePoint`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.DefaultIgnorableCodePoint.html) for more information.
        */
        fun createDefaultIgnorableCodePoint(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_default_ignorable_code_point_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Default_Ignorable_Code_Point` property, using a particular data source.
        *
        *See the [Rust documentation for `DefaultIgnorableCodePoint`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.DefaultIgnorableCodePoint.html) for more information.
        */
        fun createDefaultIgnorableCodePointWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_default_ignorable_code_point_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Diacritic` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun diacriticForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_diacritic_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Diacritic` property, using compiled data.
        *
        *See the [Rust documentation for `Diacritic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Diacritic.html) for more information.
        */
        fun createDiacritic(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_diacritic_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Diacritic` property, using a particular data source.
        *
        *See the [Rust documentation for `Diacritic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Diacritic.html) for more information.
        */
        fun createDiacriticWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_diacritic_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Emoji_Modifier_Base` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun emojiModifierBaseForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_emoji_modifier_base_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Modifier_Base` property, using compiled data.
        *
        *See the [Rust documentation for `EmojiModifierBase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiModifierBase.html) for more information.
        */
        fun createEmojiModifierBase(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_modifier_base_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Modifier_Base` property, using a particular data source.
        *
        *See the [Rust documentation for `EmojiModifierBase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiModifierBase.html) for more information.
        */
        fun createEmojiModifierBaseWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_modifier_base_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Emoji_Component` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun emojiComponentForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_emoji_component_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Component` property, using compiled data.
        *
        *See the [Rust documentation for `EmojiComponent`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiComponent.html) for more information.
        */
        fun createEmojiComponent(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_component_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Component` property, using a particular data source.
        *
        *See the [Rust documentation for `EmojiComponent`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiComponent.html) for more information.
        */
        fun createEmojiComponentWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_component_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Emoji_Modifier` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun emojiModifierForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_emoji_modifier_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Modifier` property, using compiled data.
        *
        *See the [Rust documentation for `EmojiModifier`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiModifier.html) for more information.
        */
        fun createEmojiModifier(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_modifier_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Modifier` property, using a particular data source.
        *
        *See the [Rust documentation for `EmojiModifier`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiModifier.html) for more information.
        */
        fun createEmojiModifierWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_modifier_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Emoji` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun emojiForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_emoji_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Emoji` property, using compiled data.
        *
        *See the [Rust documentation for `Emoji`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Emoji.html) for more information.
        */
        fun createEmoji(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Emoji` property, using a particular data source.
        *
        *See the [Rust documentation for `Emoji`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Emoji.html) for more information.
        */
        fun createEmojiWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Emoji_Presentation` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun emojiPresentationForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_emoji_presentation_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Presentation` property, using compiled data.
        *
        *See the [Rust documentation for `EmojiPresentation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiPresentation.html) for more information.
        */
        fun createEmojiPresentation(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_presentation_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Emoji_Presentation` property, using a particular data source.
        *
        *See the [Rust documentation for `EmojiPresentation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EmojiPresentation.html) for more information.
        */
        fun createEmojiPresentationWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_emoji_presentation_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Extender` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun extenderForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_extender_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Extender` property, using compiled data.
        *
        *See the [Rust documentation for `Extender`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Extender.html) for more information.
        */
        fun createExtender(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_extender_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Extender` property, using a particular data source.
        *
        *See the [Rust documentation for `Extender`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Extender.html) for more information.
        */
        fun createExtenderWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_extender_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Extended_Pictographic` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun extendedPictographicForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_extended_pictographic_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Extended_Pictographic` property, using compiled data.
        *
        *See the [Rust documentation for `ExtendedPictographic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ExtendedPictographic.html) for more information.
        */
        fun createExtendedPictographic(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_extended_pictographic_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Extended_Pictographic` property, using a particular data source.
        *
        *See the [Rust documentation for `ExtendedPictographic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ExtendedPictographic.html) for more information.
        */
        fun createExtendedPictographicWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_extended_pictographic_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Graph` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun graphForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_graph_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Graph` property, using compiled data.
        *
        *See the [Rust documentation for `Graph`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Graph.html) for more information.
        */
        fun createGraph(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_graph_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Graph` property, using a particular data source.
        *
        *See the [Rust documentation for `Graph`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Graph.html) for more information.
        */
        fun createGraphWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_graph_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Grapheme_Base` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun graphemeBaseForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_grapheme_base_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Grapheme_Base` property, using compiled data.
        *
        *See the [Rust documentation for `GraphemeBase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeBase.html) for more information.
        */
        fun createGraphemeBase(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_grapheme_base_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Grapheme_Base` property, using a particular data source.
        *
        *See the [Rust documentation for `GraphemeBase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeBase.html) for more information.
        */
        fun createGraphemeBaseWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_grapheme_base_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Grapheme_Extend` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun graphemeExtendForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_grapheme_extend_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Grapheme_Extend` property, using compiled data.
        *
        *See the [Rust documentation for `GraphemeExtend`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeExtend.html) for more information.
        */
        fun createGraphemeExtend(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_grapheme_extend_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Grapheme_Extend` property, using a particular data source.
        *
        *See the [Rust documentation for `GraphemeExtend`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeExtend.html) for more information.
        */
        fun createGraphemeExtendWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_grapheme_extend_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Grapheme_Link` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun graphemeLinkForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_grapheme_link_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Grapheme_Link` property, using compiled data.
        *
        *See the [Rust documentation for `GraphemeLink`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeLink.html) for more information.
        */
        fun createGraphemeLink(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_grapheme_link_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Grapheme_Link` property, using a particular data source.
        *
        *See the [Rust documentation for `GraphemeLink`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeLink.html) for more information.
        */
        fun createGraphemeLinkWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_grapheme_link_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Hex_Digit` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun hexDigitForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_hex_digit_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Hex_Digit` property, using compiled data.
        *
        *See the [Rust documentation for `HexDigit`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HexDigit.html) for more information.
        */
        fun createHexDigit(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_hex_digit_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Hex_Digit` property, using a particular data source.
        *
        *See the [Rust documentation for `HexDigit`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HexDigit.html) for more information.
        */
        fun createHexDigitWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_hex_digit_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Hyphen` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun hyphenForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_hyphen_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Hyphen` property, using compiled data.
        *
        *See the [Rust documentation for `Hyphen`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Hyphen.html) for more information.
        */
        fun createHyphen(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_hyphen_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Hyphen` property, using a particular data source.
        *
        *See the [Rust documentation for `Hyphen`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Hyphen.html) for more information.
        */
        fun createHyphenWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_hyphen_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `ID_Compat_Math_Continue` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idCompatMathContinueForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_id_compat_math_continue_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `ID_Compat_Math_Continue` property, using compiled data.
        *
        *See the [Rust documentation for `IdCompatMathContinue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdCompatMathContinue.html) for more information.
        */
        fun createIdCompatMathContinue(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_compat_math_continue_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `ID_Compat_Math_Continue` property, using a particular data source.
        *
        *See the [Rust documentation for `IdCompatMathContinue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdCompatMathContinue.html) for more information.
        */
        fun createIdCompatMathContinueWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_compat_math_continue_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `ID_Compat_Math_Start` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idCompatMathStartForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_id_compat_math_start_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `ID_Compat_Math_Start` property, using compiled data.
        *
        *See the [Rust documentation for `IdCompatMathStart`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdCompatMathStart.html) for more information.
        */
        fun createIdCompatMathStart(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_compat_math_start_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `ID_Compat_Math_Start` property, using a particular data source.
        *
        *See the [Rust documentation for `IdCompatMathStart`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdCompatMathStart.html) for more information.
        */
        fun createIdCompatMathStartWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_compat_math_start_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Id_Continue` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idContinueForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_id_continue_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Id_Continue` property, using compiled data.
        *
        *See the [Rust documentation for `IdContinue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdContinue.html) for more information.
        */
        fun createIdContinue(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_continue_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Id_Continue` property, using a particular data source.
        *
        *See the [Rust documentation for `IdContinue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdContinue.html) for more information.
        */
        fun createIdContinueWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_continue_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Ideographic` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun ideographicForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_ideographic_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Ideographic` property, using compiled data.
        *
        *See the [Rust documentation for `Ideographic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Ideographic.html) for more information.
        */
        fun createIdeographic(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ideographic_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Ideographic` property, using a particular data source.
        *
        *See the [Rust documentation for `Ideographic`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Ideographic.html) for more information.
        */
        fun createIdeographicWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ideographic_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Id_Start` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idStartForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_id_start_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Id_Start` property, using compiled data.
        *
        *See the [Rust documentation for `IdStart`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdStart.html) for more information.
        */
        fun createIdStart(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_start_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Id_Start` property, using a particular data source.
        *
        *See the [Rust documentation for `IdStart`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdStart.html) for more information.
        */
        fun createIdStartWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_id_start_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Ids_Binary_Operator` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idsBinaryOperatorForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_ids_binary_operator_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Ids_Binary_Operator` property, using compiled data.
        *
        *See the [Rust documentation for `IdsBinaryOperator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdsBinaryOperator.html) for more information.
        */
        fun createIdsBinaryOperator(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ids_binary_operator_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Ids_Binary_Operator` property, using a particular data source.
        *
        *See the [Rust documentation for `IdsBinaryOperator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdsBinaryOperator.html) for more information.
        */
        fun createIdsBinaryOperatorWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ids_binary_operator_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Ids_Trinary_Operator` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idsTrinaryOperatorForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_ids_trinary_operator_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Ids_Trinary_Operator` property, using compiled data.
        *
        *See the [Rust documentation for `IdsTrinaryOperator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdsTrinaryOperator.html) for more information.
        */
        fun createIdsTrinaryOperator(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ids_trinary_operator_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Ids_Trinary_Operator` property, using a particular data source.
        *
        *See the [Rust documentation for `IdsTrinaryOperator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdsTrinaryOperator.html) for more information.
        */
        fun createIdsTrinaryOperatorWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ids_trinary_operator_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Ids_Unary_Operator` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun idsUnaryOperatorForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_ids_unary_operator_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Ids_Unary_Operator` property, using compiled data.
        *
        *See the [Rust documentation for `IdsUnaryOperator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdsUnaryOperator.html) for more information.
        */
        fun createIdsUnaryOperator(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ids_unary_operator_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Ids_Unary_Operator` property, using a particular data source.
        *
        *See the [Rust documentation for `IdsUnaryOperator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IdsUnaryOperator.html) for more information.
        */
        fun createIdsUnaryOperatorWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_ids_unary_operator_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Join_Control` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun joinControlForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_join_control_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Join_Control` property, using compiled data.
        *
        *See the [Rust documentation for `JoinControl`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoinControl.html) for more information.
        */
        fun createJoinControl(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_join_control_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Join_Control` property, using a particular data source.
        *
        *See the [Rust documentation for `JoinControl`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoinControl.html) for more information.
        */
        fun createJoinControlWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_join_control_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Logical_Order_Exception` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun logicalOrderExceptionForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_logical_order_exception_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Logical_Order_Exception` property, using compiled data.
        *
        *See the [Rust documentation for `LogicalOrderException`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LogicalOrderException.html) for more information.
        */
        fun createLogicalOrderException(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_logical_order_exception_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Logical_Order_Exception` property, using a particular data source.
        *
        *See the [Rust documentation for `LogicalOrderException`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LogicalOrderException.html) for more information.
        */
        fun createLogicalOrderExceptionWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_logical_order_exception_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Lowercase` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun lowercaseForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_lowercase_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Lowercase` property, using compiled data.
        *
        *See the [Rust documentation for `Lowercase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Lowercase.html) for more information.
        */
        fun createLowercase(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_lowercase_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Lowercase` property, using a particular data source.
        *
        *See the [Rust documentation for `Lowercase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Lowercase.html) for more information.
        */
        fun createLowercaseWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_lowercase_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Math` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun mathForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_math_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Math` property, using compiled data.
        *
        *See the [Rust documentation for `Math`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Math.html) for more information.
        */
        fun createMath(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_math_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Math` property, using a particular data source.
        *
        *See the [Rust documentation for `Math`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Math.html) for more information.
        */
        fun createMathWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_math_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Modifier_Combining_mark` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun modifierCombiningMarkForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_modifier_combining_mark_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Modifier_Combining_mark` property, using compiled data.
        *
        *See the [Rust documentation for `ModifierCombiningMark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ModifierCombiningMark.html) for more information.
        */
        fun createModifierCombiningMark(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_modifier_combining_mark_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Modifier_Combining_mark` property, using a particular data source.
        *
        *See the [Rust documentation for `ModifierCombiningMark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.ModifierCombiningMark.html) for more information.
        */
        fun createModifierCombiningMarkWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_modifier_combining_mark_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Noncharacter_Code_Point` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun noncharacterCodePointForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_noncharacter_code_point_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Noncharacter_Code_Point` property, using compiled data.
        *
        *See the [Rust documentation for `NoncharacterCodePoint`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NoncharacterCodePoint.html) for more information.
        */
        fun createNoncharacterCodePoint(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_noncharacter_code_point_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Noncharacter_Code_Point` property, using a particular data source.
        *
        *See the [Rust documentation for `NoncharacterCodePoint`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NoncharacterCodePoint.html) for more information.
        */
        fun createNoncharacterCodePointWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_noncharacter_code_point_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Nfc_Inert` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun nfcInertForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_nfc_inert_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Nfc_Inert` property, using compiled data.
        *
        *See the [Rust documentation for `NfcInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfcInert.html) for more information.
        */
        fun createNfcInert(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfc_inert_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Nfc_Inert` property, using a particular data source.
        *
        *See the [Rust documentation for `NfcInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfcInert.html) for more information.
        */
        fun createNfcInertWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfc_inert_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Nfd_Inert` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun nfdInertForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_nfd_inert_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Nfd_Inert` property, using compiled data.
        *
        *See the [Rust documentation for `NfdInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfdInert.html) for more information.
        */
        fun createNfdInert(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfd_inert_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Nfd_Inert` property, using a particular data source.
        *
        *See the [Rust documentation for `NfdInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfdInert.html) for more information.
        */
        fun createNfdInertWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfd_inert_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Nfkc_Inert` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun nfkcInertForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_nfkc_inert_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Nfkc_Inert` property, using compiled data.
        *
        *See the [Rust documentation for `NfkcInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfkcInert.html) for more information.
        */
        fun createNfkcInert(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfkc_inert_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Nfkc_Inert` property, using a particular data source.
        *
        *See the [Rust documentation for `NfkcInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfkcInert.html) for more information.
        */
        fun createNfkcInertWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfkc_inert_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Nfkd_Inert` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun nfkdInertForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_nfkd_inert_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Nfkd_Inert` property, using compiled data.
        *
        *See the [Rust documentation for `NfkdInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfkdInert.html) for more information.
        */
        fun createNfkdInert(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfkd_inert_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Nfkd_Inert` property, using a particular data source.
        *
        *See the [Rust documentation for `NfkdInert`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NfkdInert.html) for more information.
        */
        fun createNfkdInertWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_nfkd_inert_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Pattern_Syntax` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun patternSyntaxForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_pattern_syntax_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Pattern_Syntax` property, using compiled data.
        *
        *See the [Rust documentation for `PatternSyntax`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.PatternSyntax.html) for more information.
        */
        fun createPatternSyntax(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_pattern_syntax_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Pattern_Syntax` property, using a particular data source.
        *
        *See the [Rust documentation for `PatternSyntax`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.PatternSyntax.html) for more information.
        */
        fun createPatternSyntaxWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_pattern_syntax_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Pattern_White_Space` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun patternWhiteSpaceForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_pattern_white_space_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Pattern_White_Space` property, using compiled data.
        *
        *See the [Rust documentation for `PatternWhiteSpace`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.PatternWhiteSpace.html) for more information.
        */
        fun createPatternWhiteSpace(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_pattern_white_space_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Pattern_White_Space` property, using a particular data source.
        *
        *See the [Rust documentation for `PatternWhiteSpace`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.PatternWhiteSpace.html) for more information.
        */
        fun createPatternWhiteSpaceWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_pattern_white_space_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Prepended_Concatenation_Mark` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun prependedConcatenationMarkForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_prepended_concatenation_mark_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Prepended_Concatenation_Mark` property, using compiled data.
        *
        *See the [Rust documentation for `PrependedConcatenationMark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.PrependedConcatenationMark.html) for more information.
        */
        fun createPrependedConcatenationMark(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_prepended_concatenation_mark_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Prepended_Concatenation_Mark` property, using a particular data source.
        *
        *See the [Rust documentation for `PrependedConcatenationMark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.PrependedConcatenationMark.html) for more information.
        */
        fun createPrependedConcatenationMarkWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_prepended_concatenation_mark_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Print` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun printForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_print_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Print` property, using compiled data.
        *
        *See the [Rust documentation for `Print`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Print.html) for more information.
        */
        fun createPrint(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_print_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Print` property, using a particular data source.
        *
        *See the [Rust documentation for `Print`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Print.html) for more information.
        */
        fun createPrintWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_print_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Quotation_Mark` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun quotationMarkForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_quotation_mark_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Quotation_Mark` property, using compiled data.
        *
        *See the [Rust documentation for `QuotationMark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.QuotationMark.html) for more information.
        */
        fun createQuotationMark(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_quotation_mark_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Quotation_Mark` property, using a particular data source.
        *
        *See the [Rust documentation for `QuotationMark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.QuotationMark.html) for more information.
        */
        fun createQuotationMarkWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_quotation_mark_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Radical` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun radicalForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_radical_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Radical` property, using compiled data.
        *
        *See the [Rust documentation for `Radical`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Radical.html) for more information.
        */
        fun createRadical(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_radical_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Radical` property, using a particular data source.
        *
        *See the [Rust documentation for `Radical`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Radical.html) for more information.
        */
        fun createRadicalWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_radical_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Regional_Indicator` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun regionalIndicatorForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_regional_indicator_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Regional_Indicator` property, using compiled data.
        *
        *See the [Rust documentation for `RegionalIndicator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.RegionalIndicator.html) for more information.
        */
        fun createRegionalIndicator(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_regional_indicator_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Regional_Indicator` property, using a particular data source.
        *
        *See the [Rust documentation for `RegionalIndicator`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.RegionalIndicator.html) for more information.
        */
        fun createRegionalIndicatorWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_regional_indicator_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Soft_Dotted` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun softDottedForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_soft_dotted_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Soft_Dotted` property, using compiled data.
        *
        *See the [Rust documentation for `SoftDotted`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SoftDotted.html) for more information.
        */
        fun createSoftDotted(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_soft_dotted_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Soft_Dotted` property, using a particular data source.
        *
        *See the [Rust documentation for `SoftDotted`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SoftDotted.html) for more information.
        */
        fun createSoftDottedWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_soft_dotted_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Segment_Starter` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun segmentStarterForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_segment_starter_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Segment_Starter` property, using compiled data.
        *
        *See the [Rust documentation for `SegmentStarter`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SegmentStarter.html) for more information.
        */
        fun createSegmentStarter(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_segment_starter_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Segment_Starter` property, using a particular data source.
        *
        *See the [Rust documentation for `SegmentStarter`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SegmentStarter.html) for more information.
        */
        fun createSegmentStarterWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_segment_starter_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Case_Sensitive` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun caseSensitiveForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_case_sensitive_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Case_Sensitive` property, using compiled data.
        *
        *See the [Rust documentation for `CaseSensitive`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CaseSensitive.html) for more information.
        */
        fun createCaseSensitive(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_case_sensitive_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Case_Sensitive` property, using a particular data source.
        *
        *See the [Rust documentation for `CaseSensitive`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CaseSensitive.html) for more information.
        */
        fun createCaseSensitiveWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_case_sensitive_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Sentence_Terminal` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun sentenceTerminalForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_sentence_terminal_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Sentence_Terminal` property, using compiled data.
        *
        *See the [Rust documentation for `SentenceTerminal`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SentenceTerminal.html) for more information.
        */
        fun createSentenceTerminal(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_sentence_terminal_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Sentence_Terminal` property, using a particular data source.
        *
        *See the [Rust documentation for `SentenceTerminal`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SentenceTerminal.html) for more information.
        */
        fun createSentenceTerminalWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_sentence_terminal_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Terminal_Punctuation` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun terminalPunctuationForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_terminal_punctuation_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Terminal_Punctuation` property, using compiled data.
        *
        *See the [Rust documentation for `TerminalPunctuation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.TerminalPunctuation.html) for more information.
        */
        fun createTerminalPunctuation(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_terminal_punctuation_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Terminal_Punctuation` property, using a particular data source.
        *
        *See the [Rust documentation for `TerminalPunctuation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.TerminalPunctuation.html) for more information.
        */
        fun createTerminalPunctuationWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_terminal_punctuation_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Unified_Ideograph` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun unifiedIdeographForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_unified_ideograph_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Unified_Ideograph` property, using compiled data.
        *
        *See the [Rust documentation for `UnifiedIdeograph`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.UnifiedIdeograph.html) for more information.
        */
        fun createUnifiedIdeograph(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_unified_ideograph_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Unified_Ideograph` property, using a particular data source.
        *
        *See the [Rust documentation for `UnifiedIdeograph`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.UnifiedIdeograph.html) for more information.
        */
        fun createUnifiedIdeographWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_unified_ideograph_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Uppercase` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun uppercaseForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_uppercase_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Uppercase` property, using compiled data.
        *
        *See the [Rust documentation for `Uppercase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Uppercase.html) for more information.
        */
        fun createUppercase(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_uppercase_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Uppercase` property, using a particular data source.
        *
        *See the [Rust documentation for `Uppercase`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Uppercase.html) for more information.
        */
        fun createUppercaseWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_uppercase_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Variation_Selector` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun variationSelectorForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_variation_selector_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Variation_Selector` property, using compiled data.
        *
        *See the [Rust documentation for `VariationSelector`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.VariationSelector.html) for more information.
        */
        fun createVariationSelector(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_variation_selector_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Variation_Selector` property, using a particular data source.
        *
        *See the [Rust documentation for `VariationSelector`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.VariationSelector.html) for more information.
        */
        fun createVariationSelectorWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_variation_selector_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `White_Space` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun whiteSpaceForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_white_space_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `White_Space` property, using compiled data.
        *
        *See the [Rust documentation for `WhiteSpace`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.WhiteSpace.html) for more information.
        */
        fun createWhiteSpace(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_white_space_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `White_Space` property, using a particular data source.
        *
        *See the [Rust documentation for `WhiteSpace`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.WhiteSpace.html) for more information.
        */
        fun createWhiteSpaceWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_white_space_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Xdigit` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun xdigitForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_xdigit_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Xdigit` property, using compiled data.
        *
        *See the [Rust documentation for `Xdigit`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Xdigit.html) for more information.
        */
        fun createXdigit(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_xdigit_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Xdigit` property, using a particular data source.
        *
        *See the [Rust documentation for `Xdigit`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Xdigit.html) for more information.
        */
        fun createXdigitWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_xdigit_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Xid_Continue` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun xidContinueForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_xid_continue_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Xid_Continue` property, using compiled data.
        *
        *See the [Rust documentation for `XidContinue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.XidContinue.html) for more information.
        */
        fun createXidContinue(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_xid_continue_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Xid_Continue` property, using a particular data source.
        *
        *See the [Rust documentation for `XidContinue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.XidContinue.html) for more information.
        */
        fun createXidContinueWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_xid_continue_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Xid_Start` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.BinaryProperty.html#tymethod.for_char) for more information.
        */
        fun xidStartForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_CodePointSetData_xid_start_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Create a set for the `Xid_Start` property, using compiled data.
        *
        *See the [Rust documentation for `XidStart`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.XidStart.html) for more information.
        */
        fun createXidStart(): CodePointSetData {
            
            val returnVal = lib.icu4x_CodePointSetData_create_xid_start_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a set for the `Xid_Start` property, using a particular data source.
        *
        *See the [Rust documentation for `XidStart`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.XidStart.html) for more information.
        */
        fun createXidStartWithProvider(provider: DataProvider): Result<CodePointSetData> {
            
            val returnVal = lib.icu4x_CodePointSetData_create_xid_start_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
        *
        *See the [Rust documentation for `new_for_ecma262`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetData.html#method.new_for_ecma262) for more information.
        */
        fun createForEcma262(propertyName: String): Result<CodePointSetData> {
            val (propertyNameMem, propertyNameSlice) = PrimitiveArrayTools.borrowUtf8(propertyName)
            
            val returnVal = lib.icu4x_CodePointSetData_create_for_ecma262_mv1(propertyNameSlice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                if (propertyNameMem != null) propertyNameMem.close()
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
        *
        *See the [Rust documentation for `new_for_ecma262`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetData.html#method.new_for_ecma262) for more information.
        */
        fun createForEcma262WithProvider(provider: DataProvider, propertyName: String): Result<CodePointSetData> {
            val (propertyNameMem, propertyNameSlice) = PrimitiveArrayTools.borrowUtf8(propertyName)
            
            val returnVal = lib.icu4x_CodePointSetData_create_for_ecma262_with_provider_mv1(provider.handle, propertyNameSlice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
                if (propertyNameMem != null) propertyNameMem.close()
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Checks whether the code point is in the set.
    *
    *See the [Rust documentation for `contains`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetDataBorrowed.html#method.contains) for more information.
    */
    fun contains(cp: Int): Boolean {
        
        val returnVal = lib.icu4x_CodePointSetData_contains_mv1(handle, cp);
        return (returnVal > 0)
    }
    
    /** Produces an iterator over ranges of code points contained in this set
    *
    *See the [Rust documentation for `iter_ranges`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetDataBorrowed.html#method.iter_ranges) for more information.
    */
    fun iterRanges(): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointSetData_iter_ranges_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }
    
    /** Produces an iterator over ranges of code points not contained in this set
    *
    *See the [Rust documentation for `iter_ranges_complemented`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointSetDataBorrowed.html#method.iter_ranges_complemented) for more information.
    */
    fun iterRangesComplemented(): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointSetData_iter_ranges_complemented_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CodePointMapData8Lib: Library {
    fun icu4x_CodePointMapData8_destroy_mv1(handle: Pointer)
    fun icu4x_CodePointMapData8_get_mv1(handle: Pointer, cp: Int): FFIUint8
    fun icu4x_CodePointMapData8_iter_ranges_for_value_mv1(handle: Pointer, value: FFIUint8): Pointer
    fun icu4x_CodePointMapData8_iter_ranges_for_value_complemented_mv1(handle: Pointer, value: FFIUint8): Pointer
    fun icu4x_CodePointMapData8_iter_ranges_for_group_mv1(handle: Pointer, group: GeneralCategoryGroupNative): Pointer
    fun icu4x_CodePointMapData8_get_set_for_value_mv1(handle: Pointer, value: FFIUint8): Pointer
    fun icu4x_CodePointMapData8_create_bidi_class_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_bidi_class_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_numeric_type_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_numeric_type_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_hangul_syllable_type_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_hangul_syllable_type_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_east_asian_width_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_east_asian_width_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_line_break_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_line_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_grapheme_cluster_break_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_grapheme_cluster_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_word_break_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_word_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_sentence_break_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_sentence_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_canonical_combining_class_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_canonical_combining_class_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_indic_syllabic_category_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_indic_syllabic_category_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_indic_conjunct_break_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_indic_conjunct_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_joining_group_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_joining_group_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_joining_type_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_joining_type_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_general_category_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_general_category_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_vertical_orientation_mv1(): Pointer
    fun icu4x_CodePointMapData8_create_vertical_orientation_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CodePointMapData8_create_planes_mv1(): Pointer
}
/** An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
*
*For properties whose values fit into 8 bits.
*
*See the [Rust documentation for `properties`](https://docs.rs/icu/2.1.1/icu/properties/index.html) for more information.
*
*See the [Rust documentation for `CodePointMapData`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapData.html) for more information.
*
*See the [Rust documentation for `CodePointMapDataBorrowed`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html) for more information.
*/
class CodePointMapData8 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CodePointMapData8Cleaner(val handle: Pointer, val lib: CodePointMapData8Lib) : Runnable {
        override fun run() {
            lib.icu4x_CodePointMapData8_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CodePointMapData8Lib> = CodePointMapData8Lib::class.java
        internal val lib: CodePointMapData8Lib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a map for the `BidiClass` property, using compiled data.
        *
        *See the [Rust documentation for `BidiClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiClass.html) for more information.
        */
        fun createBidiClass(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_bidi_class_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `BidiClass` property, using a particular data source.
        *
        *See the [Rust documentation for `BidiClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiClass.html) for more information.
        */
        fun createBidiClassWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_bidi_class_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `NumericType` property, using compiled data.
        *
        *See the [Rust documentation for `NumericType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericType.html) for more information.
        */
        fun createNumericType(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_numeric_type_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `NumericType` property, using a particular data source.
        *
        *See the [Rust documentation for `NumericType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericType.html) for more information.
        */
        fun createNumericTypeWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_numeric_type_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `HangulSyllableType` property, using compiled data.
        *
        *See the [Rust documentation for `HangulSyllableType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html) for more information.
        */
        fun createHangulSyllableType(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_hangul_syllable_type_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `HangulSyllableType` property, using a particular data source.
        *
        *See the [Rust documentation for `HangulSyllableType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html) for more information.
        */
        fun createHangulSyllableTypeWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_hangul_syllable_type_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `EastAsianWidth` property, using compiled data.
        *
        *See the [Rust documentation for `EastAsianWidth`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html) for more information.
        */
        fun createEastAsianWidth(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_east_asian_width_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `EastAsianWidth` property, using a particular data source.
        *
        *See the [Rust documentation for `EastAsianWidth`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html) for more information.
        */
        fun createEastAsianWidthWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_east_asian_width_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `LineBreak` property, using compiled data.
        *
        *See the [Rust documentation for `LineBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html) for more information.
        */
        fun createLineBreak(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_line_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `LineBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `LineBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html) for more information.
        */
        fun createLineBreakWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_line_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `GraphemeClusterBreak` property, using compiled data.
        *
        *See the [Rust documentation for `GraphemeClusterBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeClusterBreak.html) for more information.
        */
        fun createGraphemeClusterBreak(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_grapheme_cluster_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `GraphemeClusterBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `GraphemeClusterBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeClusterBreak.html) for more information.
        */
        fun createGraphemeClusterBreakWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_grapheme_cluster_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `WordBreak` property, using compiled data.
        *
        *See the [Rust documentation for `WordBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.WordBreak.html) for more information.
        */
        fun createWordBreak(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_word_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `WordBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `WordBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.WordBreak.html) for more information.
        */
        fun createWordBreakWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_word_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `SentenceBreak` property, using compiled data.
        *
        *See the [Rust documentation for `SentenceBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SentenceBreak.html) for more information.
        */
        fun createSentenceBreak(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_sentence_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `SentenceBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `SentenceBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SentenceBreak.html) for more information.
        */
        fun createSentenceBreakWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_sentence_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `CanonicalCombiningClass` property, using compiled data.
        *
        *See the [Rust documentation for `CanonicalCombiningClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html) for more information.
        */
        fun createCanonicalCombiningClass(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_canonical_combining_class_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `CanonicalCombiningClass` property, using a particular data source.
        *
        *See the [Rust documentation for `CanonicalCombiningClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html) for more information.
        */
        fun createCanonicalCombiningClassWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_canonical_combining_class_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `IndicSyllabicCategory` property, using compiled data.
        *
        *See the [Rust documentation for `IndicSyllabicCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicSyllabicCategory.html) for more information.
        */
        fun createIndicSyllabicCategory(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_indic_syllabic_category_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `IndicSyllabicCategory` property, using a particular data source.
        *
        *See the [Rust documentation for `IndicSyllabicCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicSyllabicCategory.html) for more information.
        */
        fun createIndicSyllabicCategoryWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_indic_syllabic_category_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `IndicConjunctBreak` property, using compiled data.
        *
        *See the [Rust documentation for `IndicConjunctBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html) for more information.
        */
        fun createIndicConjunctBreak(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_indic_conjunct_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `IndicConjunctBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `IndicConjunctBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html) for more information.
        */
        fun createIndicConjunctBreakWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_indic_conjunct_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `JoiningGroup` property, using compiled data.
        *
        *See the [Rust documentation for `JoiningGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html) for more information.
        */
        fun createJoiningGroup(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_joining_group_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `JoiningGroup` property, using a particular data source.
        *
        *See the [Rust documentation for `JoiningGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html) for more information.
        */
        fun createJoiningGroupWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_joining_group_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `JoiningType` property, using compiled data.
        *
        *See the [Rust documentation for `JoiningType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningType.html) for more information.
        */
        fun createJoiningType(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_joining_type_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `JoiningType` property, using a particular data source.
        *
        *See the [Rust documentation for `JoiningType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningType.html) for more information.
        */
        fun createJoiningTypeWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_joining_type_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `GeneralCategory` property, using compiled data.
        *
        *See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.GeneralCategory.html) for more information.
        */
        fun createGeneralCategory(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_general_category_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `GeneralCategory` property, using a particular data source.
        *
        *See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.GeneralCategory.html) for more information.
        */
        fun createGeneralCategoryWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_general_category_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the `VerticalOrientation` property, using compiled data.
        *
        *See the [Rust documentation for `VerticalOrientation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.VerticalOrientation.html) for more information.
        */
        fun createVerticalOrientation(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_vertical_orientation_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `VerticalOrientation` property, using a particular data source.
        *
        *See the [Rust documentation for `VerticalOrientation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.VerticalOrientation.html) for more information.
        */
        fun createVerticalOrientationWithProvider(provider: DataProvider): Result<CodePointMapData8> {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_vertical_orientation_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData8(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a map for the associated planes.
        */
        fun createPlanes(): CodePointMapData8 {
            
            val returnVal = lib.icu4x_CodePointMapData8_create_planes_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData8(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData8.CodePointMapData8Cleaner(handle, CodePointMapData8.lib));
            return returnOpaque
        }
    }
    
    /** Gets the value for a code point.
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.get) for more information.
    */
    fun get(cp: Int): UByte {
        
        val returnVal = lib.icu4x_CodePointMapData8_get_mv1(handle, cp);
        return (returnVal.toUByte())
    }
    
    /** Produces an iterator over ranges of code points that map to `value`
    *
    *See the [Rust documentation for `iter_ranges_for_value`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value) for more information.
    */
    fun iterRangesForValue(value: UByte): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointMapData8_iter_ranges_for_value_mv1(handle, FFIUint8(value));
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }
    
    /** Produces an iterator over ranges of code points that do not map to `value`
    *
    *See the [Rust documentation for `iter_ranges_for_value_complemented`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented) for more information.
    */
    fun iterRangesForValueComplemented(value: UByte): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointMapData8_iter_ranges_for_value_complemented_mv1(handle, FFIUint8(value));
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }
    
    /** Given a mask value (the nth bit marks property value = n), produce an iterator over ranges of code points
    *whose property values are contained in the mask.
    *
    *The main mask property supported is that for General_Category, which can be obtained via `general_category_to_mask()` or
    *by using `GeneralCategoryNameToMaskMapper`
    *
    *Should only be used on maps for properties with values less than 32 (like Generak_Category),
    *other maps will have unpredictable results
    *
    *See the [Rust documentation for `iter_ranges_for_group`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_group) for more information.
    */
    fun iterRangesForGroup(group: GeneralCategoryGroup): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointMapData8_iter_ranges_for_group_mv1(handle, group.toNative());
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }
    
    /** Gets a [CodePointSetData] representing all entries in this map that map to the given value
    *
    *See the [Rust documentation for `get_set_for_value`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.get_set_for_value) for more information.
    */
    fun getSetForValue(value: UByte): CodePointSetData {
        
        val returnVal = lib.icu4x_CodePointMapData8_get_set_for_value_mv1(handle, FFIUint8(value));
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = CodePointSetData(handle, selfEdges)
        CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
        return returnOpaque
    }

}
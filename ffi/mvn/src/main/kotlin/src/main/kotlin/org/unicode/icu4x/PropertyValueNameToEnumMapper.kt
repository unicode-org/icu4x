package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PropertyValueNameToEnumMapperLib: Library {
    fun icu4x_PropertyValueNameToEnumMapper_destroy_mv1(handle: Pointer)
    fun icu4x_PropertyValueNameToEnumMapper_get_strict_mv1(handle: Pointer, name: Slice): Short
    fun icu4x_PropertyValueNameToEnumMapper_get_loose_mv1(handle: Pointer, name: Slice): Short
    fun icu4x_PropertyValueNameToEnumMapper_create_bidi_class_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_bidi_class_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_numeric_type_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_numeric_type_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_script_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_script_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_hangul_syllable_type_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_hangul_syllable_type_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_east_asian_width_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_east_asian_width_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_line_break_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_line_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_grapheme_cluster_break_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_grapheme_cluster_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_word_break_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_word_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_sentence_break_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_sentence_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_canonical_combining_class_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_canonical_combining_class_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_indic_syllabic_category_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_indic_syllabic_category_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_indic_conjunct_break_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_indic_conjunct_break_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_joining_group_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_joining_group_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_joining_type_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_joining_type_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_general_category_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_general_category_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_PropertyValueNameToEnumMapper_create_vertical_orientation_mv1(): Pointer
    fun icu4x_PropertyValueNameToEnumMapper_create_vertical_orientation_with_provider_mv1(provider: Pointer): ResultPointerInt
}
/** A type capable of looking up a property value from a string name.
*
*See the [Rust documentation for `PropertyParser`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParser.html) for more information.
*
*See the [Rust documentation for `PropertyParserBorrowed`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParserBorrowed.html) for more information.
*
*See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParser.html#method.new) for more information.
*/
class PropertyValueNameToEnumMapper internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class PropertyValueNameToEnumMapperCleaner(val handle: Pointer, val lib: PropertyValueNameToEnumMapperLib) : Runnable {
        override fun run() {
            lib.icu4x_PropertyValueNameToEnumMapper_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<PropertyValueNameToEnumMapperLib> = PropertyValueNameToEnumMapperLib::class.java
        internal val lib: PropertyValueNameToEnumMapperLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `BidiClass` property, using compiled data.
        *
        *See the [Rust documentation for `BidiClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiClass.html) for more information.
        */
        fun createBidiClass(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_bidi_class_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `BidiClass` property, using a particular data source.
        *
        *See the [Rust documentation for `BidiClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiClass.html) for more information.
        */
        fun createBidiClassWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_bidi_class_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `NumericType` property, using compiled data.
        *
        *See the [Rust documentation for `NumericType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericType.html) for more information.
        */
        fun createNumericType(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_numeric_type_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `NumericType` property, using a particular data source.
        *
        *See the [Rust documentation for `NumericType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericType.html) for more information.
        */
        fun createNumericTypeWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_numeric_type_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `Script` property, using compiled data.
        *
        *See the [Rust documentation for `Script`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html) for more information.
        */
        fun createScript(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_script_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `Script` property, using a particular data source.
        *
        *See the [Rust documentation for `Script`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html) for more information.
        */
        fun createScriptWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_script_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `HangulSyllableType` property, using compiled data.
        *
        *See the [Rust documentation for `HangulSyllableType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html) for more information.
        */
        fun createHangulSyllableType(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_hangul_syllable_type_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `HangulSyllableType` property, using a particular data source.
        *
        *See the [Rust documentation for `HangulSyllableType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html) for more information.
        */
        fun createHangulSyllableTypeWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_hangul_syllable_type_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `EastAsianWidth` property, using compiled data.
        *
        *See the [Rust documentation for `EastAsianWidth`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html) for more information.
        */
        fun createEastAsianWidth(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_east_asian_width_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `EastAsianWidth` property, using a particular data source.
        *
        *See the [Rust documentation for `EastAsianWidth`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html) for more information.
        */
        fun createEastAsianWidthWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_east_asian_width_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `LineBreak` property, using compiled data.
        *
        *See the [Rust documentation for `LineBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html) for more information.
        */
        fun createLineBreak(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_line_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `LineBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `LineBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html) for more information.
        */
        fun createLineBreakWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_line_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `GraphemeClusterBreak` property, using compiled data.
        *
        *See the [Rust documentation for `GraphemeClusterBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeClusterBreak.html) for more information.
        */
        fun createGraphemeClusterBreak(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_grapheme_cluster_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `GraphemeClusterBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `GraphemeClusterBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GraphemeClusterBreak.html) for more information.
        */
        fun createGraphemeClusterBreakWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_grapheme_cluster_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `WordBreak` property, using compiled data.
        *
        *See the [Rust documentation for `WordBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.WordBreak.html) for more information.
        */
        fun createWordBreak(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_word_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `WordBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `WordBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.WordBreak.html) for more information.
        */
        fun createWordBreakWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_word_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `SentenceBreak` property, using compiled data.
        *
        *See the [Rust documentation for `SentenceBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SentenceBreak.html) for more information.
        */
        fun createSentenceBreak(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_sentence_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `SentenceBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `SentenceBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.SentenceBreak.html) for more information.
        */
        fun createSentenceBreakWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_sentence_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `CanonicalCombiningClass` property, using compiled data.
        *
        *See the [Rust documentation for `CanonicalCombiningClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html) for more information.
        */
        fun createCanonicalCombiningClass(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_canonical_combining_class_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `CanonicalCombiningClass` property, using a particular data source.
        *
        *See the [Rust documentation for `CanonicalCombiningClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html) for more information.
        */
        fun createCanonicalCombiningClassWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_canonical_combining_class_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `IndicSyllabicCategory` property, using compiled data.
        *
        *See the [Rust documentation for `IndicSyllabicCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicSyllabicCategory.html) for more information.
        */
        fun createIndicSyllabicCategory(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_indic_syllabic_category_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `IndicSyllabicCategory` property, using a particular data source.
        *
        *See the [Rust documentation for `IndicSyllabicCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicSyllabicCategory.html) for more information.
        */
        fun createIndicSyllabicCategoryWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_indic_syllabic_category_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `IndicConjunctBreak` property, using compiled data.
        *
        *See the [Rust documentation for `IndicConjunctBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html) for more information.
        */
        fun createIndicConjunctBreak(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_indic_conjunct_break_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `IndicConjunctBreak` property, using a particular data source.
        *
        *See the [Rust documentation for `IndicConjunctBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html) for more information.
        */
        fun createIndicConjunctBreakWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_indic_conjunct_break_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `JoiningGroup` property, using compiled data.
        *
        *See the [Rust documentation for `JoiningGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html) for more information.
        */
        fun createJoiningGroup(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_joining_group_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `JoiningGroup` property, using a particular data source.
        *
        *See the [Rust documentation for `JoiningGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html) for more information.
        */
        fun createJoiningGroupWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_joining_group_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `JoiningType` property, using compiled data.
        *
        *See the [Rust documentation for `JoiningType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningType.html) for more information.
        */
        fun createJoiningType(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_joining_type_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `JoiningType` property, using a particular data source.
        *
        *See the [Rust documentation for `JoiningType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningType.html) for more information.
        */
        fun createJoiningTypeWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_joining_type_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `GeneralCategory` property, using compiled data.
        *
        *See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.GeneralCategory.html) for more information.
        */
        fun createGeneralCategory(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_general_category_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `GeneralCategory` property, using a particular data source.
        *
        *See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.GeneralCategory.html) for more information.
        */
        fun createGeneralCategoryWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_general_category_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `VerticalOrientation` property, using compiled data.
        *
        *See the [Rust documentation for `VerticalOrientation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.VerticalOrientation.html) for more information.
        */
        fun createVerticalOrientation(): PropertyValueNameToEnumMapper {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_vertical_orientation_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-enum mapper for the `VerticalOrientation` property, using a particular data source.
        *
        *See the [Rust documentation for `VerticalOrientation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.VerticalOrientation.html) for more information.
        */
        fun createVerticalOrientationWithProvider(provider: DataProvider): Result<PropertyValueNameToEnumMapper> {
            
            val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_create_vertical_orientation_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PropertyValueNameToEnumMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, PropertyValueNameToEnumMapper.PropertyValueNameToEnumMapperCleaner(handle, PropertyValueNameToEnumMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Get the property value matching the given name, using strict matching
    *
    *Returns -1 if the name is unknown for this property
    *
    *See the [Rust documentation for `get_strict`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParserBorrowed.html#method.get_strict) for more information.
    */
    fun getStrict(name: String): Short {
        val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
        
        val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_get_strict_mv1(handle, nameSliceMemory.slice);
        return (returnVal)
    }
    
    /** Get the property value matching the given name, using loose matching
    *
    *Returns -1 if the name is unknown for this property
    *
    *See the [Rust documentation for `get_loose`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParserBorrowed.html#method.get_loose) for more information.
    */
    fun getLoose(name: String): Short {
        val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
        
        val returnVal = lib.icu4x_PropertyValueNameToEnumMapper_get_loose_mv1(handle, nameSliceMemory.slice);
        return (returnVal)
    }

}
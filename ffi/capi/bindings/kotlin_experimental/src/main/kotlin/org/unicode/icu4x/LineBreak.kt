package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LineBreakLib: Library {
    fun icu4x_LineBreak_for_char_mv1(ch: Int): Int
    fun icu4x_LineBreak_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_LineBreak_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_LineBreak_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_LineBreak_from_integer_value_mv1(other: FFIUint8): OptionInt
}
/** See the [Rust documentation for `LineBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html) for more information.
*/
enum class LineBreak {
    Unknown,
    Ambiguous,
    Alphabetic,
    BreakBoth,
    BreakAfter,
    BreakBefore,
    MandatoryBreak,
    ContingentBreak,
    ClosePunctuation,
    CombiningMark,
    CarriageReturn,
    Exclamation,
    Glue,
    Hyphen,
    Ideographic,
    Inseparable,
    InfixNumeric,
    LineFeed,
    Nonstarter,
    Numeric,
    OpenPunctuation,
    PostfixNumeric,
    PrefixNumeric,
    Quotation,
    ComplexContext,
    Surrogate,
    Space,
    BreakSymbols,
    ZWSpace,
    NextLine,
    WordJoiner,
    H2,
    H3,
    JL,
    JT,
    JV,
    CloseParenthesis,
    ConditionalJapaneseStarter,
    HebrewLetter,
    RegionalIndicator,
    EBase,
    EModifier,
    ZWJ,
    Aksara,
    AksaraPrebase,
    AksaraStart,
    ViramaFinal,
    Virama,
    UnambiguousHyphen;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LineBreakLib> = LineBreakLib::class.java
        internal val lib: LineBreakLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LineBreak {
            return LineBreak.entries[native]
        }

        fun default(): LineBreak {
            return Unknown
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): LineBreak {
            
            val returnVal = lib.icu4x_LineBreak_for_char_mv1(ch);
            return (LineBreak.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): LineBreak? {
            
            val returnVal = lib.icu4x_LineBreak_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return LineBreak.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_LineBreak_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_LineBreak_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.LineBreak.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_LineBreak_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
}
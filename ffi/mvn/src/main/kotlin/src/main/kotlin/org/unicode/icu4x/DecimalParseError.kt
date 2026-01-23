package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalParseErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.ParseError.html)
*/
enum class DecimalParseError {
    Unknown,
    Limit,
    Syntax;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DecimalParseErrorLib> = DecimalParseErrorLib::class.java
        internal val lib: DecimalParseErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DecimalParseError {
            return DecimalParseError.entries[native]
        }

        fun default(): DecimalParseError {
            return Unknown
        }
    }
}
class DecimalParseErrorError internal constructor(internal val value: DecimalParseError): Exception("Rust error result for DecimalParseError") {
    override fun toString(): String {
        return "DecimalParseError error with value " + value
    }

    fun getValue(): DecimalParseError = value
}

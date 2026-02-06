package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleParseErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/locale/enum.ParseError.html)
*/
enum class LocaleParseError {
    Unknown,
    Language,
    Subtag,
    Extension;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LocaleParseErrorLib> = LocaleParseErrorLib::class.java
        internal val lib: LocaleParseErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LocaleParseError {
            return LocaleParseError.entries[native]
        }

        fun default(): LocaleParseError {
            return Unknown
        }
    }
}
class LocaleParseErrorError internal constructor(internal val value: LocaleParseError): Exception("Rust error result for LocaleParseError") {
    override fun toString(): String {
        return "LocaleParseError error with value " + value
    }

    fun getValue(): LocaleParseError = value
}

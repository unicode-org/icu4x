package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface Rfc9557ParseErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/enum.ParseError.html), [2](https://docs.rs/icu/2.1.1/icu/time/enum.ParseError.html)
*/
enum class Rfc9557ParseError {
    Unknown,
    InvalidSyntax,
    OutOfRange,
    MissingFields,
    UnknownCalendar;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<Rfc9557ParseErrorLib> = Rfc9557ParseErrorLib::class.java
        internal val lib: Rfc9557ParseErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): Rfc9557ParseError {
            return Rfc9557ParseError.entries[native]
        }

        fun default(): Rfc9557ParseError {
            return Unknown
        }
    }
}
class Rfc9557ParseErrorError internal constructor(internal val value: Rfc9557ParseError): Exception("Rust error result for Rfc9557ParseError") {
    override fun toString(): String {
        return "Rfc9557ParseError error with value " + value
    }

    fun getValue(): Rfc9557ParseError = value
}
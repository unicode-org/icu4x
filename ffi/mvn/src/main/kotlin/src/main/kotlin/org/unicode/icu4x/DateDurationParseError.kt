package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateDurationParseErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/error/enum.DateDurationParseError.html)
*/
enum class DateDurationParseError {
    InvalidStructure,
    TimeNotSupported,
    MissingValue,
    DuplicateUnit,
    NumberOverflow,
    PlusNotAllowed;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateDurationParseErrorLib> = DateDurationParseErrorLib::class.java
        internal val lib: DateDurationParseErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateDurationParseError {
            return DateDurationParseError.entries[native]
        }

        fun default(): DateDurationParseError {
            return InvalidStructure
        }
    }
}
class DateDurationParseErrorError internal constructor(internal val value: DateDurationParseError): Exception("Rust error result for DateDurationParseError") {
    override fun toString(): String {
        return "DateDurationParseError error with value " + value
    }

    fun getValue(): DateDurationParseError = value
}

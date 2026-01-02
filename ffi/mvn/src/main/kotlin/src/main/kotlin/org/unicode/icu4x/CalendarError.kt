package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CalendarErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/struct.RangeError.html), [2](https://docs.rs/icu/2.1.1/icu/calendar/enum.DateError.html), [3](https://docs.rs/icu/2.1.1/icu/calendar/error/enum.MonthCodeParseError.html)
*/
enum class CalendarError {
    Unknown,
    OutOfRange,
    UnknownEra,
    UnknownMonthCode;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CalendarErrorLib> = CalendarErrorLib::class.java
        internal val lib: CalendarErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CalendarError {
            return CalendarError.entries[native]
        }

        fun default(): CalendarError {
            return Unknown
        }
    }
}
class CalendarErrorError internal constructor(internal val value: CalendarError): Exception("Rust error result for CalendarError") {
    override fun toString(): String {
        return "CalendarError error with value " + value
    }

    fun getValue(): CalendarError = value
}

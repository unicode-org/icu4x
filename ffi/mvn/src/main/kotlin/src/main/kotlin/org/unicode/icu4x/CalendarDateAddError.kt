package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CalendarDateAddErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/error/enum.DateAddError.html)
*/
enum class CalendarDateAddError {
    Unknown,
    InvalidDay,
    MonthNotInYear,
    Overflow;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CalendarDateAddErrorLib> = CalendarDateAddErrorLib::class.java
        internal val lib: CalendarDateAddErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CalendarDateAddError {
            return CalendarDateAddError.entries[native]
        }

        fun default(): CalendarDateAddError {
            return Unknown
        }
    }
}
class CalendarDateAddErrorError internal constructor(internal val value: CalendarDateAddError): Exception("Rust error result for CalendarDateAddError") {
    override fun toString(): String {
        return "CalendarDateAddError error with value " + value
    }

    fun getValue(): CalendarDateAddError = value
}

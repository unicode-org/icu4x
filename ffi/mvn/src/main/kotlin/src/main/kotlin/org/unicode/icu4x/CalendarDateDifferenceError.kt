package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CalendarDateDifferenceErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/cal/enum.AnyCalendarDifferenceError.html)
*/
enum class CalendarDateDifferenceError {
    Unknown,
    MismatchedCalendars;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CalendarDateDifferenceErrorLib> = CalendarDateDifferenceErrorLib::class.java
        internal val lib: CalendarDateDifferenceErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CalendarDateDifferenceError {
            return CalendarDateDifferenceError.entries[native]
        }

        fun default(): CalendarDateDifferenceError {
            return Unknown
        }
    }
}
class CalendarDateDifferenceErrorError internal constructor(internal val value: CalendarDateDifferenceError): Exception("Rust error result for CalendarDateDifferenceError") {
    override fun toString(): String {
        return "CalendarDateDifferenceError error with value " + value
    }

    fun getValue(): CalendarDateDifferenceError = value
}

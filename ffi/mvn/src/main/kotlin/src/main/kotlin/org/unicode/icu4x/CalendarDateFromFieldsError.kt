package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CalendarDateFromFieldsErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/error/enum.DateFromFieldsError.html)
*/
enum class CalendarDateFromFieldsError {
    Unknown,
    OutOfRange,
    UnknownEra,
    MonthCodeInvalidSyntax,
    MonthCodeNotInCalendar,
    MonthCodeNotInYear,
    InconsistentYear,
    InconsistentMonth,
    NotEnoughFields;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CalendarDateFromFieldsErrorLib> = CalendarDateFromFieldsErrorLib::class.java
        internal val lib: CalendarDateFromFieldsErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CalendarDateFromFieldsError {
            return CalendarDateFromFieldsError.entries[native]
        }

        fun default(): CalendarDateFromFieldsError {
            return Unknown
        }
    }
}
class CalendarDateFromFieldsErrorError internal constructor(internal val value: CalendarDateFromFieldsError): Exception("Rust error result for CalendarDateFromFieldsError") {
    override fun toString(): String {
        return "CalendarDateFromFieldsError error with value " + value
    }

    fun getValue(): CalendarDateFromFieldsError = value
}
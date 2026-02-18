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
enum class CalendarDateFromFieldsError(val inner: Int) {
    Unknown(0),
    InvalidDay(1),
    InvalidOrdinalMonth(9),
    InvalidEra(2),
    MonthCodeInvalidSyntax(3),
    MonthNotInCalendar(4),
    MonthNotInYear(5),
    InconsistentYear(6),
    InconsistentMonth(7),
    NotEnoughFields(8),
    TooManyFields(10),
    Overflow(11);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<CalendarDateFromFieldsErrorLib> = CalendarDateFromFieldsErrorLib::class.java
        internal val lib: CalendarDateFromFieldsErrorLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): CalendarDateFromFieldsError {
            return when (native) {
                0 -> Unknown
                1 -> InvalidDay
                9 -> InvalidOrdinalMonth
                2 -> InvalidEra
                3 -> MonthCodeInvalidSyntax
                4 -> MonthNotInCalendar
                5 -> MonthNotInYear
                6 -> InconsistentYear
                7 -> InconsistentMonth
                8 -> NotEnoughFields
                10 -> TooManyFields
                11 -> Overflow
                else -> throw RuntimeException("Failed to find variant ${native} of type CalendarDateFromFieldsError")
            }
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

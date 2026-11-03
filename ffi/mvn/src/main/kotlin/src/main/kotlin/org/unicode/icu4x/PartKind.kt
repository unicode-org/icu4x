package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PartKindLib: Library {
}
/**
 * The kind of a [PartSpan], corresponding to `writeable::Part`.
 *
 * See the [Rust documentation for `Part`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class PartKind {
    Unknown,
    Error,
    ListElement,
    ListLiteral,
    DecimalPlusSign,
    DecimalMinusSign,
    DecimalInteger,
    DecimalFraction,
    DecimalGroup,
    DecimalDecimal,
    DateTimeEra,
    DateTimeYear,
    DateTimeRelatedYear,
    DateTimeYearName,
    DateTimeMonth,
    DateTimeDay,
    DateTimeExtendedYear,
    DateTimeJulianDay,
    DateTimeWeekday,
    DateTimeDayPeriod,
    DateTimeHour,
    DateTimeMinute,
    DateTimeSecond,
    DateTimeTimeZoneName,
    DateTimeStartRange,
    DateTimeEndRange,
    RelativeTimeLiteral,
    DurationLiteral,
    UnitYear,
    UnitMonth,
    UnitWeek,
    UnitDay,
    UnitHour,
    UnitMinute,
    UnitSecond,
    UnitMillisecond,
    UnitMicrosecond,
    UnitNanosecond;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<PartKindLib> = PartKindLib::class.java
        internal val lib: PartKindLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): PartKind {
            return PartKind.entries[native]
        }

        fun default(): PartKind {
            return Unknown
        }
    }
}

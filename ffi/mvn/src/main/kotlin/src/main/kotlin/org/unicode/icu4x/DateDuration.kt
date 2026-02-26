package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateDurationLib: Library {
    fun icu4x_DateDuration_from_string_mv1(v: Slice): ResultDateDurationNativeInt
    fun icu4x_DateDuration_for_years_mv1(years: Int): DateDurationNative
    fun icu4x_DateDuration_for_months_mv1(months: Int): DateDurationNative
    fun icu4x_DateDuration_for_weeks_mv1(weeks: Int): DateDurationNative
    fun icu4x_DateDuration_for_days_mv1(days: Long): DateDurationNative
}

internal class DateDurationNative: Structure(), Structure.ByValue {
    @JvmField
    internal var isNegative: Byte = 0;
    @JvmField
    internal var years: FFIUint32 = FFIUint32();
    @JvmField
    internal var months: FFIUint32 = FFIUint32();
    @JvmField
    internal var weeks: FFIUint32 = FFIUint32();
    @JvmField
    internal var days: FFIUint64 = FFIUint64();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("isNegative", "years", "months", "weeks", "days")
    }
}




internal class OptionDateDurationNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DateDurationNative = DateDurationNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DateDurationNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DateDurationNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DateDurationNative): OptionDateDurationNative {
            return OptionDateDurationNative(value, 1)
        }

        internal fun none(): OptionDateDurationNative {
            return OptionDateDurationNative(DateDurationNative(), 0)
        }
    }

}

/** See the [Rust documentation for `DateDuration`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html) for more information.
*/
class DateDuration (var isNegative: Boolean, var years: UInt, var months: UInt, var weeks: UInt, var days: ULong) {
    companion object {

        internal val libClass: Class<DateDurationLib> = DateDurationLib::class.java
        internal val lib: DateDurationLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateDurationNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DateDurationNative): DateDuration {
            val isNegative: Boolean = nativeStruct.isNegative > 0
            val years: UInt = nativeStruct.years.toUInt()
            val months: UInt = nativeStruct.months.toUInt()
            val weeks: UInt = nativeStruct.weeks.toUInt()
            val days: ULong = nativeStruct.days.toULong()

            return DateDuration(isNegative, years, months, weeks, days)
        }

        @JvmStatic
        
        /** Creates a new [DateDuration] from an ISO 8601 string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html#method.try_from_str) for more information.
        */
        fun fromString(v: String): Result<DateDuration> {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_DateDuration_from_string_mv1(vSliceMemory.slice);
            if (returnVal.isOk == 1.toByte()) {
                val returnStruct = DateDuration.fromNative(returnVal.union.ok)
                vSliceMemory?.close()
                return returnStruct.ok()
            } else {
                return DateDurationParseErrorError(DateDurationParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Returns a new [DateDuration] representing a number of years.
        *
        *See the [Rust documentation for `for_years`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html#method.for_years) for more information.
        */
        fun forYears(years: Int): DateDuration {
            
            val returnVal = lib.icu4x_DateDuration_for_years_mv1(years);
            val returnStruct = DateDuration.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** Returns a new [DateDuration] representing a number of months.
        *
        *See the [Rust documentation for `for_months`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html#method.for_months) for more information.
        */
        fun forMonths(months: Int): DateDuration {
            
            val returnVal = lib.icu4x_DateDuration_for_months_mv1(months);
            val returnStruct = DateDuration.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** Returns a new [DateDuration] representing a number of weeks.
        *
        *See the [Rust documentation for `for_weeks`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html#method.for_weeks) for more information.
        */
        fun forWeeks(weeks: Int): DateDuration {
            
            val returnVal = lib.icu4x_DateDuration_for_weeks_mv1(weeks);
            val returnStruct = DateDuration.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** Returns a new [DateDuration] representing a number of days.
        *
        *See the [Rust documentation for `for_days`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html#method.for_days) for more information.
        */
        fun forDays(days: Long): DateDuration {
            
            val returnVal = lib.icu4x_DateDuration_for_days_mv1(days);
            val returnStruct = DateDuration.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): DateDurationNative {
        var native = DateDurationNative()
        native.isNegative = if (this.isNegative) 1 else 0
        native.years = FFIUint32(this.years)
        native.months = FFIUint32(this.months)
        native.weeks = FFIUint32(this.weeks)
        native.days = FFIUint64(this.days)
        return native
    }

}
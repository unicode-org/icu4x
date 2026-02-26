package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateDurationLib: Library {
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
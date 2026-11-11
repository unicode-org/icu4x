package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationLib: Library {
}

internal class DurationNative: Structure(), Structure.ByValue {
    @JvmField
    internal var sign: Int = DurationSign.default().toNative();
    @JvmField
    internal var years: FFIUint64 = FFIUint64();
    @JvmField
    internal var months: FFIUint64 = FFIUint64();
    @JvmField
    internal var weeks: FFIUint64 = FFIUint64();
    @JvmField
    internal var days: FFIUint64 = FFIUint64();
    @JvmField
    internal var hours: FFIUint64 = FFIUint64();
    @JvmField
    internal var minutes: FFIUint64 = FFIUint64();
    @JvmField
    internal var seconds: FFIUint64 = FFIUint64();
    @JvmField
    internal var milliseconds: FFIUint64 = FFIUint64();
    @JvmField
    internal var microseconds: FFIUint64 = FFIUint64();
    @JvmField
    internal var nanoseconds: FFIUint64 = FFIUint64();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("sign", "years", "months", "weeks", "days", "hours", "minutes", "seconds", "milliseconds", "microseconds", "nanoseconds")
    }
}




internal class OptionDurationNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DurationNative = DurationNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DurationNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DurationNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DurationNative): OptionDurationNative {
            return OptionDurationNative(value, 1)
        }

        internal fun none(): OptionDurationNative {
            return OptionDurationNative(DurationNative(), 0)
        }
    }

}

/**
 * See the [Rust documentation for `Duration`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.Duration.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class Duration (var sign: DurationSign, var years: ULong, var months: ULong, var weeks: ULong, var days: ULong, var hours: ULong, var minutes: ULong, var seconds: ULong, var milliseconds: ULong, var microseconds: ULong, var nanoseconds: ULong) {
    companion object {

        internal val libClass: Class<DurationLib> = DurationLib::class.java
        internal val lib: DurationLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DurationNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DurationNative): Duration {
            val sign: DurationSign = DurationSign.fromNative(nativeStruct.sign)
            val years: ULong = nativeStruct.years.toULong()
            val months: ULong = nativeStruct.months.toULong()
            val weeks: ULong = nativeStruct.weeks.toULong()
            val days: ULong = nativeStruct.days.toULong()
            val hours: ULong = nativeStruct.hours.toULong()
            val minutes: ULong = nativeStruct.minutes.toULong()
            val seconds: ULong = nativeStruct.seconds.toULong()
            val milliseconds: ULong = nativeStruct.milliseconds.toULong()
            val microseconds: ULong = nativeStruct.microseconds.toULong()
            val nanoseconds: ULong = nativeStruct.nanoseconds.toULong()

            return Duration(sign, years, months, weeks, days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds)
        }

    }
    internal fun toNative(): DurationNative {
        var native = DurationNative()
        native.sign = this.sign.toNative()
        native.years = FFIUint64(this.years)
        native.months = FFIUint64(this.months)
        native.weeks = FFIUint64(this.weeks)
        native.days = FFIUint64(this.days)
        native.hours = FFIUint64(this.hours)
        native.minutes = FFIUint64(this.minutes)
        native.seconds = FFIUint64(this.seconds)
        native.milliseconds = FFIUint64(this.milliseconds)
        native.microseconds = FFIUint64(this.microseconds)
        native.nanoseconds = FFIUint64(this.nanoseconds)
        return native
    }

}
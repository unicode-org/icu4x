package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationFormatterOptionsLib: Library {
}

internal class DurationFormatterOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var base: Int = DurationBaseStyle.default().toNative();
    @JvmField
    internal var year: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var yearVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var month: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var monthVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var week: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var weekVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var day: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var dayVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var hour: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var hourVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var minute: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var minuteVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var second: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var secondVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var millisecond: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var millisecondVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var microsecond: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var microsecondVisibility: Int = DurationFieldDisplay.default().toNative();
    @JvmField
    internal var nanosecond: Int = DurationUnitStyle.default().toNative();
    @JvmField
    internal var nanosecondVisibility: Int = DurationFieldDisplay.default().toNative();
    /**
     * Number of fractional digits to use when formatting sub-second units.
     * Pass `-1` for `FractionalDigits::ShowAll`, or `0..=9` for `FractionalDigits::Fixed(n)`.
     */
    @JvmField
    internal var fractionalDigits: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("base", "year", "yearVisibility", "month", "monthVisibility", "week", "weekVisibility", "day", "dayVisibility", "hour", "hourVisibility", "minute", "minuteVisibility", "second", "secondVisibility", "millisecond", "millisecondVisibility", "microsecond", "microsecondVisibility", "nanosecond", "nanosecondVisibility", "fractionalDigits")
    }
}




internal class OptionDurationFormatterOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DurationFormatterOptionsNative = DurationFormatterOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DurationFormatterOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DurationFormatterOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DurationFormatterOptionsNative): OptionDurationFormatterOptionsNative {
            return OptionDurationFormatterOptionsNative(value, 1)
        }

        internal fun none(): OptionDurationFormatterOptionsNative {
            return OptionDurationFormatterOptionsNative(DurationFormatterOptionsNative(), 0)
        }
    }

}

/**
 * See the [Rust documentation for `DurationFormatterOptions`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/struct.DurationFormatterOptions.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationFormatterOptions (var base: DurationBaseStyle, var year: DurationUnitStyle, var yearVisibility: DurationFieldDisplay, var month: DurationUnitStyle, var monthVisibility: DurationFieldDisplay, var week: DurationUnitStyle, var weekVisibility: DurationFieldDisplay, var day: DurationUnitStyle, var dayVisibility: DurationFieldDisplay, var hour: DurationUnitStyle, var hourVisibility: DurationFieldDisplay, var minute: DurationUnitStyle, var minuteVisibility: DurationFieldDisplay, var second: DurationUnitStyle, var secondVisibility: DurationFieldDisplay, var millisecond: DurationUnitStyle, var millisecondVisibility: DurationFieldDisplay, var microsecond: DurationUnitStyle, var microsecondVisibility: DurationFieldDisplay, var nanosecond: DurationUnitStyle, var nanosecondVisibility: DurationFieldDisplay, var fractionalDigits: Byte) {
    companion object {

        internal val libClass: Class<DurationFormatterOptionsLib> = DurationFormatterOptionsLib::class.java
        internal val lib: DurationFormatterOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DurationFormatterOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DurationFormatterOptionsNative): DurationFormatterOptions {
            val base: DurationBaseStyle = DurationBaseStyle.fromNative(nativeStruct.base)
            val year: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.year)
            val yearVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.yearVisibility)
            val month: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.month)
            val monthVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.monthVisibility)
            val week: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.week)
            val weekVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.weekVisibility)
            val day: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.day)
            val dayVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.dayVisibility)
            val hour: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.hour)
            val hourVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.hourVisibility)
            val minute: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.minute)
            val minuteVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.minuteVisibility)
            val second: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.second)
            val secondVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.secondVisibility)
            val millisecond: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.millisecond)
            val millisecondVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.millisecondVisibility)
            val microsecond: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.microsecond)
            val microsecondVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.microsecondVisibility)
            val nanosecond: DurationUnitStyle = DurationUnitStyle.fromNative(nativeStruct.nanosecond)
            val nanosecondVisibility: DurationFieldDisplay = DurationFieldDisplay.fromNative(nativeStruct.nanosecondVisibility)
            val fractionalDigits: Byte = nativeStruct.fractionalDigits

            return DurationFormatterOptions(base, year, yearVisibility, month, monthVisibility, week, weekVisibility, day, dayVisibility, hour, hourVisibility, minute, minuteVisibility, second, secondVisibility, millisecond, millisecondVisibility, microsecond, microsecondVisibility, nanosecond, nanosecondVisibility, fractionalDigits)
        }

    }
    internal fun toNative(): DurationFormatterOptionsNative {
        var native = DurationFormatterOptionsNative()
        native.base = this.base.toNative()
        native.year = this.year.toNative()
        native.yearVisibility = this.yearVisibility.toNative()
        native.month = this.month.toNative()
        native.monthVisibility = this.monthVisibility.toNative()
        native.week = this.week.toNative()
        native.weekVisibility = this.weekVisibility.toNative()
        native.day = this.day.toNative()
        native.dayVisibility = this.dayVisibility.toNative()
        native.hour = this.hour.toNative()
        native.hourVisibility = this.hourVisibility.toNative()
        native.minute = this.minute.toNative()
        native.minuteVisibility = this.minuteVisibility.toNative()
        native.second = this.second.toNative()
        native.secondVisibility = this.secondVisibility.toNative()
        native.millisecond = this.millisecond.toNative()
        native.millisecondVisibility = this.millisecondVisibility.toNative()
        native.microsecond = this.microsecond.toNative()
        native.microsecondVisibility = this.microsecondVisibility.toNative()
        native.nanosecond = this.nanosecond.toNative()
        native.nanosecondVisibility = this.nanosecondVisibility.toNative()
        native.fractionalDigits = this.fractionalDigits
        return native
    }

}
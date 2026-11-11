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
    internal var year: OptionInt = OptionInt.none();
    @JvmField
    internal var yearVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var month: OptionInt = OptionInt.none();
    @JvmField
    internal var monthVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var week: OptionInt = OptionInt.none();
    @JvmField
    internal var weekVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var day: OptionInt = OptionInt.none();
    @JvmField
    internal var dayVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var hour: OptionInt = OptionInt.none();
    @JvmField
    internal var hourVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var minute: OptionInt = OptionInt.none();
    @JvmField
    internal var minuteVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var second: OptionInt = OptionInt.none();
    @JvmField
    internal var secondVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var millisecond: OptionInt = OptionInt.none();
    @JvmField
    internal var millisecondVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var microsecond: OptionInt = OptionInt.none();
    @JvmField
    internal var microsecondVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var nanosecond: OptionInt = OptionInt.none();
    @JvmField
    internal var nanosecondVisibility: OptionInt = OptionInt.none();
    @JvmField
    internal var fractionalDigits: OptionFFIUint8 = OptionFFIUint8.none();

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
class DurationFormatterOptions (var base: DurationBaseStyle, var year: DurationUnitStyle?, var yearVisibility: DurationFieldDisplay?, var month: DurationUnitStyle?, var monthVisibility: DurationFieldDisplay?, var week: DurationUnitStyle?, var weekVisibility: DurationFieldDisplay?, var day: DurationUnitStyle?, var dayVisibility: DurationFieldDisplay?, var hour: DurationUnitStyle?, var hourVisibility: DurationFieldDisplay?, var minute: DurationUnitStyle?, var minuteVisibility: DurationFieldDisplay?, var second: DurationUnitStyle?, var secondVisibility: DurationFieldDisplay?, var millisecond: DurationUnitStyle?, var millisecondVisibility: DurationFieldDisplay?, var microsecond: DurationUnitStyle?, var microsecondVisibility: DurationFieldDisplay?, var nanosecond: DurationUnitStyle?, var nanosecondVisibility: DurationFieldDisplay?, var fractionalDigits: UByte?) {
    companion object {

        internal val libClass: Class<DurationFormatterOptionsLib> = DurationFormatterOptionsLib::class.java
        internal val lib: DurationFormatterOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DurationFormatterOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DurationFormatterOptionsNative): DurationFormatterOptions {
            val base: DurationBaseStyle = DurationBaseStyle.fromNative(nativeStruct.base)
            val year: DurationUnitStyle? = nativeStruct.year.option()?.let { DurationUnitStyle.fromNative(it) }
            val yearVisibility: DurationFieldDisplay? = nativeStruct.yearVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val month: DurationUnitStyle? = nativeStruct.month.option()?.let { DurationUnitStyle.fromNative(it) }
            val monthVisibility: DurationFieldDisplay? = nativeStruct.monthVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val week: DurationUnitStyle? = nativeStruct.week.option()?.let { DurationUnitStyle.fromNative(it) }
            val weekVisibility: DurationFieldDisplay? = nativeStruct.weekVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val day: DurationUnitStyle? = nativeStruct.day.option()?.let { DurationUnitStyle.fromNative(it) }
            val dayVisibility: DurationFieldDisplay? = nativeStruct.dayVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val hour: DurationUnitStyle? = nativeStruct.hour.option()?.let { DurationUnitStyle.fromNative(it) }
            val hourVisibility: DurationFieldDisplay? = nativeStruct.hourVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val minute: DurationUnitStyle? = nativeStruct.minute.option()?.let { DurationUnitStyle.fromNative(it) }
            val minuteVisibility: DurationFieldDisplay? = nativeStruct.minuteVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val second: DurationUnitStyle? = nativeStruct.second.option()?.let { DurationUnitStyle.fromNative(it) }
            val secondVisibility: DurationFieldDisplay? = nativeStruct.secondVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val millisecond: DurationUnitStyle? = nativeStruct.millisecond.option()?.let { DurationUnitStyle.fromNative(it) }
            val millisecondVisibility: DurationFieldDisplay? = nativeStruct.millisecondVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val microsecond: DurationUnitStyle? = nativeStruct.microsecond.option()?.let { DurationUnitStyle.fromNative(it) }
            val microsecondVisibility: DurationFieldDisplay? = nativeStruct.microsecondVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val nanosecond: DurationUnitStyle? = nativeStruct.nanosecond.option()?.let { DurationUnitStyle.fromNative(it) }
            val nanosecondVisibility: DurationFieldDisplay? = nativeStruct.nanosecondVisibility.option()?.let { DurationFieldDisplay.fromNative(it) }
            val fractionalDigits: UByte? = nativeStruct.fractionalDigits.option()?.let { it.toUByte() }

            return DurationFormatterOptions(base, year, yearVisibility, month, monthVisibility, week, weekVisibility, day, dayVisibility, hour, hourVisibility, minute, minuteVisibility, second, secondVisibility, millisecond, millisecondVisibility, microsecond, microsecondVisibility, nanosecond, nanosecondVisibility, fractionalDigits)
        }

    }
    internal fun toNative(): DurationFormatterOptionsNative {
        var native = DurationFormatterOptionsNative()
        native.base = this.base.toNative()
        native.year = this.year?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.yearVisibility = this.yearVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.month = this.month?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.monthVisibility = this.monthVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.week = this.week?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.weekVisibility = this.weekVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.day = this.day?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.dayVisibility = this.dayVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.hour = this.hour?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.hourVisibility = this.hourVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.minute = this.minute?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.minuteVisibility = this.minuteVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.second = this.second?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.secondVisibility = this.secondVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.millisecond = this.millisecond?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.millisecondVisibility = this.millisecondVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.microsecond = this.microsecond?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.microsecondVisibility = this.microsecondVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.nanosecond = this.nanosecond?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.nanosecondVisibility = this.nanosecondVisibility?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.fractionalDigits = this.fractionalDigits?.let { OptionFFIUint8.some(FFIUint8(it)) } ?: OptionFFIUint8.none()
        return native
    }

}
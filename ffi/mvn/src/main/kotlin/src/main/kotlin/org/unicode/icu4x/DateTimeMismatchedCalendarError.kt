package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeMismatchedCalendarErrorLib: Library {
}

internal class DateTimeMismatchedCalendarErrorNative: Structure(), Structure.ByValue {
    @JvmField
    internal var thisKind: Int = CalendarKind.default().toNative();
    @JvmField
    internal var dateKind: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("thisKind", "dateKind")
    }
}




internal class OptionDateTimeMismatchedCalendarErrorNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DateTimeMismatchedCalendarErrorNative = DateTimeMismatchedCalendarErrorNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DateTimeMismatchedCalendarErrorNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DateTimeMismatchedCalendarErrorNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DateTimeMismatchedCalendarErrorNative): OptionDateTimeMismatchedCalendarErrorNative {
            return OptionDateTimeMismatchedCalendarErrorNative(value, 1)
        }

        internal fun none(): OptionDateTimeMismatchedCalendarErrorNative {
            return OptionDateTimeMismatchedCalendarErrorNative(DateTimeMismatchedCalendarErrorNative(), 0)
        }
    }

}

/** See the [Rust documentation for `MismatchedCalendarError`](https://docs.rs/icu/2.1.1/icu/datetime/struct.MismatchedCalendarError.html) for more information.
*/
class DateTimeMismatchedCalendarError (var thisKind: CalendarKind, var dateKind: CalendarKind?): Exception("Rust error result for DateTimeMismatchedCalendarError") {
    companion object {

        internal val libClass: Class<DateTimeMismatchedCalendarErrorLib> = DateTimeMismatchedCalendarErrorLib::class.java
        internal val lib: DateTimeMismatchedCalendarErrorLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateTimeMismatchedCalendarErrorNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DateTimeMismatchedCalendarErrorNative): DateTimeMismatchedCalendarError {
            val thisKind: CalendarKind = CalendarKind.fromNative(nativeStruct.thisKind)
            val dateKind: CalendarKind? = nativeStruct.dateKind.option()?.let { CalendarKind.fromNative(it) }

            return DateTimeMismatchedCalendarError(thisKind, dateKind)
        }

    }
    internal fun toNative(): DateTimeMismatchedCalendarErrorNative {
        var native = DateTimeMismatchedCalendarErrorNative()
        native.thisKind = this.thisKind.toNative()
        native.dateKind = this.dateKind?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
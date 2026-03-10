package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateDifferenceOptionsLib: Library {
}

internal class DateDifferenceOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var largestUnit: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("largestUnit")
    }
}




internal class OptionDateDifferenceOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DateDifferenceOptionsNative = DateDifferenceOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DateDifferenceOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DateDifferenceOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DateDifferenceOptionsNative): OptionDateDifferenceOptionsNative {
            return OptionDateDifferenceOptionsNative(value, 1)
        }

        internal fun none(): OptionDateDifferenceOptionsNative {
            return OptionDateDifferenceOptionsNative(DateDifferenceOptionsNative(), 0)
        }
    }

}

/** See the [Rust documentation for `DateDifferenceOptions`](https://docs.rs/icu/2.1.1/icu/calendar/options/struct.DateDifferenceOptions.html) for more information.
*/
class DateDifferenceOptions (var largestUnit: DateDurationUnit?) {
    companion object {

        internal val libClass: Class<DateDifferenceOptionsLib> = DateDifferenceOptionsLib::class.java
        internal val lib: DateDifferenceOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateDifferenceOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DateDifferenceOptionsNative): DateDifferenceOptions {
            val largestUnit: DateDurationUnit? = nativeStruct.largestUnit.option()?.let { DateDurationUnit.fromNative(it) }

            return DateDifferenceOptions(largestUnit)
        }

    }
    internal fun toNative(): DateDifferenceOptionsNative {
        var native = DateDifferenceOptionsNative()
        native.largestUnit = this.largestUnit?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
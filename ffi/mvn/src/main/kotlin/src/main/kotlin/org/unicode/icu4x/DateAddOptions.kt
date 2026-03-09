package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateAddOptionsLib: Library {
}

internal class DateAddOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var overflow: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("overflow")
    }
}




internal class OptionDateAddOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DateAddOptionsNative = DateAddOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DateAddOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DateAddOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DateAddOptionsNative): OptionDateAddOptionsNative {
            return OptionDateAddOptionsNative(value, 1)
        }

        internal fun none(): OptionDateAddOptionsNative {
            return OptionDateAddOptionsNative(DateAddOptionsNative(), 0)
        }
    }

}

/** See the [Rust documentation for `DateAddOptions`](https://docs.rs/icu/2.1.1/icu/calendar/options/struct.DateAddOptions.html) for more information.
*/
class DateAddOptions (var overflow: DateOverflow?) {
    companion object {

        internal val libClass: Class<DateAddOptionsLib> = DateAddOptionsLib::class.java
        internal val lib: DateAddOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateAddOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DateAddOptionsNative): DateAddOptions {
            val overflow: DateOverflow? = nativeStruct.overflow.option()?.let { DateOverflow.fromNative(it) }

            return DateAddOptions(overflow)
        }

    }
    internal fun toNative(): DateAddOptionsNative {
        var native = DateAddOptionsNative()
        native.overflow = this.overflow?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateFromFieldsOptionsLib: Library {
}

internal class DateFromFieldsOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var overflow: OptionInt = OptionInt.none();
    @JvmField
    internal var missingFieldsStrategy: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("overflow", "missingFieldsStrategy")
    }
}




internal class OptionDateFromFieldsOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DateFromFieldsOptionsNative = DateFromFieldsOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DateFromFieldsOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DateFromFieldsOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DateFromFieldsOptionsNative): OptionDateFromFieldsOptionsNative {
            return OptionDateFromFieldsOptionsNative(value, 1)
        }

        internal fun none(): OptionDateFromFieldsOptionsNative {
            return OptionDateFromFieldsOptionsNative(DateFromFieldsOptionsNative(), 0)
        }
    }

}

/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `DateFromFieldsOptions`](https://docs.rs/icu/2.1.1/icu/calendar/options/struct.DateFromFieldsOptions.html) for more information.
*/
class DateFromFieldsOptions (var overflow: DateOverflow?, var missingFieldsStrategy: DateMissingFieldsStrategy?) {
    companion object {

        internal val libClass: Class<DateFromFieldsOptionsLib> = DateFromFieldsOptionsLib::class.java
        internal val lib: DateFromFieldsOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateFromFieldsOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DateFromFieldsOptionsNative): DateFromFieldsOptions {
            val overflow: DateOverflow? = nativeStruct.overflow.option()?.let { DateOverflow.fromNative(it) }
            val missingFieldsStrategy: DateMissingFieldsStrategy? = nativeStruct.missingFieldsStrategy.option()?.let { DateMissingFieldsStrategy.fromNative(it) }

            return DateFromFieldsOptions(overflow, missingFieldsStrategy)
        }

    }
    internal fun toNative(): DateFromFieldsOptionsNative {
        var native = DateFromFieldsOptionsNative()
        native.overflow = this.overflow?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.missingFieldsStrategy = this.missingFieldsStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
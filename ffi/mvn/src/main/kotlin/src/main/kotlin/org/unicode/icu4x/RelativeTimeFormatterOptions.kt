package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RelativeTimeFormatterOptionsLib: Library {
}

internal class RelativeTimeFormatterOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var numeric: OptionInt = OptionInt.none();
    @JvmField
    internal var groupingStrategy: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("numeric", "groupingStrategy")
    }
}




internal class OptionRelativeTimeFormatterOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: RelativeTimeFormatterOptionsNative = RelativeTimeFormatterOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): RelativeTimeFormatterOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: RelativeTimeFormatterOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: RelativeTimeFormatterOptionsNative): OptionRelativeTimeFormatterOptionsNative {
            return OptionRelativeTimeFormatterOptionsNative(value, 1)
        }

        internal fun none(): OptionRelativeTimeFormatterOptionsNative {
            return OptionRelativeTimeFormatterOptionsNative(RelativeTimeFormatterOptionsNative(), 0)
        }
    }

}

/**
 * A bag of options for defining how to format time using [RelativeTimeFormatter].
 *
 * See the [Rust documentation for `RelativeTimeFormatterOptions`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatterOptions.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class RelativeTimeFormatterOptions (var numeric: RelativeTimeNumeric?, var groupingStrategy: DecimalGroupingStrategy?) {
    companion object {

        internal val libClass: Class<RelativeTimeFormatterOptionsLib> = RelativeTimeFormatterOptionsLib::class.java
        internal val lib: RelativeTimeFormatterOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(RelativeTimeFormatterOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: RelativeTimeFormatterOptionsNative): RelativeTimeFormatterOptions {
            val numeric: RelativeTimeNumeric? = nativeStruct.numeric.option()?.let { RelativeTimeNumeric.fromNative(it) }
            val groupingStrategy: DecimalGroupingStrategy? = nativeStruct.groupingStrategy.option()?.let { DecimalGroupingStrategy.fromNative(it) }

            return RelativeTimeFormatterOptions(numeric, groupingStrategy)
        }

    }
    internal fun toNative(): RelativeTimeFormatterOptionsNative {
        var native = RelativeTimeFormatterOptionsNative()
        native.numeric = this.numeric?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.groupingStrategy = this.groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
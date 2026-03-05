package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TitlecaseOptionsLib: Library {
    fun icu4x_TitlecaseOptionsV1_default_mv1(): TitlecaseOptionsNative
}

internal class TitlecaseOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var leadingAdjustment: OptionInt = OptionInt.none();
    @JvmField
    internal var trailingCase: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("leadingAdjustment", "trailingCase")
    }
}




internal class OptionTitlecaseOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: TitlecaseOptionsNative = TitlecaseOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TitlecaseOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: TitlecaseOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: TitlecaseOptionsNative): OptionTitlecaseOptionsNative {
            return OptionTitlecaseOptionsNative(value, 1)
        }

        internal fun none(): OptionTitlecaseOptionsNative {
            return OptionTitlecaseOptionsNative(TitlecaseOptionsNative(), 0)
        }
    }

}

/** See the [Rust documentation for `TitlecaseOptions`](https://docs.rs/icu/2.1.1/icu/casemap/options/struct.TitlecaseOptions.html) for more information.
*/
class TitlecaseOptions (var leadingAdjustment: LeadingAdjustment?, var trailingCase: TrailingCase?) {
    companion object {

        internal val libClass: Class<TitlecaseOptionsLib> = TitlecaseOptionsLib::class.java
        internal val lib: TitlecaseOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TitlecaseOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: TitlecaseOptionsNative): TitlecaseOptions {
            val leadingAdjustment: LeadingAdjustment? = nativeStruct.leadingAdjustment.option()?.let { LeadingAdjustment.fromNative(it) }
            val trailingCase: TrailingCase? = nativeStruct.trailingCase.option()?.let { TrailingCase.fromNative(it) }

            return TitlecaseOptions(leadingAdjustment, trailingCase)
        }

        @JvmStatic
        
        /** See the [Rust documentation for `default`](https://docs.rs/icu/2.1.1/icu/casemap/options/struct.TitlecaseOptions.html#method.default) for more information.
        */
        fun default_(): TitlecaseOptions {
            
            val returnVal = lib.icu4x_TitlecaseOptionsV1_default_mv1();
            val returnStruct = TitlecaseOptions.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): TitlecaseOptionsNative {
        var native = TitlecaseOptionsNative()
        native.leadingAdjustment = this.leadingAdjustment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.trailingCase = this.trailingCase?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
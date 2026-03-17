package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LineBreakOptionsLib: Library {
}

internal class LineBreakOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var strictness: OptionInt = OptionInt.none();
    @JvmField
    internal var wordOption: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("strictness", "wordOption")
    }
}




internal class OptionLineBreakOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: LineBreakOptionsNative = LineBreakOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): LineBreakOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: LineBreakOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: LineBreakOptionsNative): OptionLineBreakOptionsNative {
            return OptionLineBreakOptionsNative(value, 1)
        }

        internal fun none(): OptionLineBreakOptionsNative {
            return OptionLineBreakOptionsNative(LineBreakOptionsNative(), 0)
        }
    }

}

/** See the [Rust documentation for `LineBreakOptions`](https://docs.rs/icu/2.1.1/icu/segmenter/options/struct.LineBreakOptions.html) for more information.
*/
class LineBreakOptions (var strictness: LineBreakStrictness?, var wordOption: LineBreakWordOption?) {
    companion object {

        internal val libClass: Class<LineBreakOptionsLib> = LineBreakOptionsLib::class.java
        internal val lib: LineBreakOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(LineBreakOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: LineBreakOptionsNative): LineBreakOptions {
            val strictness: LineBreakStrictness? = nativeStruct.strictness.option()?.let { LineBreakStrictness.fromNative(it) }
            val wordOption: LineBreakWordOption? = nativeStruct.wordOption.option()?.let { LineBreakWordOption.fromNative(it) }

            return LineBreakOptions(strictness, wordOption)
        }

    }
    internal fun toNative(): LineBreakOptionsNative {
        var native = LineBreakOptionsNative()
        native.strictness = this.strictness?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.wordOption = this.wordOption?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
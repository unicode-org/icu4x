package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PartSpanLib: Library {
}

internal class PartSpanNative: Structure(), Structure.ByValue {
    @JvmField
    internal var start: FFISizet = FFISizet();
    @JvmField
    internal var end: FFISizet = FFISizet();
    @JvmField
    internal var kind: Int = PartKind.default().toNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("start", "end", "kind")
    }
}




internal class OptionPartSpanNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: PartSpanNative = PartSpanNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): PartSpanNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: PartSpanNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: PartSpanNative): OptionPartSpanNative {
            return OptionPartSpanNative(value, 1)
        }

        internal fun none(): OptionPartSpanNative {
            return OptionPartSpanNative(PartSpanNative(), 0)
        }
    }

}

/**
 * A span within a formatted string annotated with a [PartKind].
 *
 * `start` and `end` are UTF-8 byte offsets into the string written to `DiplomatWrite`.
 * Spans are recorded in pre-order: ordered by `start` ascending, then `end` descending,
 * with outer spans preceding inner nested spans.
 *
 * See the [Rust documentation for `Part`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class PartSpan (var start: ULong, var end: ULong, var kind: PartKind) {
    companion object {

        internal val libClass: Class<PartSpanLib> = PartSpanLib::class.java
        internal val lib: PartSpanLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(PartSpanNative::class.java).toLong()

        internal fun fromNative(nativeStruct: PartSpanNative): PartSpan {
            val start: ULong = nativeStruct.start.toULong()
            val end: ULong = nativeStruct.end.toULong()
            val kind: PartKind = PartKind.fromNative(nativeStruct.kind)

            return PartSpan(start, end, kind)
        }

    }
    internal fun toNative(): PartSpanNative {
        var native = PartSpanNative()
        native.start = FFISizet(this.start)
        native.end = FFISizet(this.end)
        native.kind = this.kind.toNative()
        return native
    }

}
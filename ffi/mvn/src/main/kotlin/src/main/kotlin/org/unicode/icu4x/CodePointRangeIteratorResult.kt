package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CodePointRangeIteratorResultLib: Library {
}

internal class CodePointRangeIteratorResultNative: Structure(), Structure.ByValue {
    @JvmField
    internal var start: Int = 0;
    @JvmField
    internal var end: Int = 0;
    @JvmField
    internal var done: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("start", "end", "done")
    }
}




internal class OptionCodePointRangeIteratorResultNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: CodePointRangeIteratorResultNative = CodePointRangeIteratorResultNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): CodePointRangeIteratorResultNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: CodePointRangeIteratorResultNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: CodePointRangeIteratorResultNative): OptionCodePointRangeIteratorResultNative {
            return OptionCodePointRangeIteratorResultNative(value, 1)
        }

        internal fun none(): OptionCodePointRangeIteratorResultNative {
            return OptionCodePointRangeIteratorResultNative(CodePointRangeIteratorResultNative(), 0)
        }
    }

}

/** Result of a single iteration of [CodePointRangeIterator].
*Logically can be considered to be an `Option<RangeInclusive<DiplomatChar>>`,
*
*`start` and `end` represent an inclusive range of code points `[start, end]`,
*and `done` will be true if the iterator has already finished. The last contentful
*iteration will NOT produce a range `done=true`, in other words `start` and `end` are useful
*values if and only if `done=false`.
*/
class CodePointRangeIteratorResult (var start: Int, var end: Int, var done: Boolean) {
    companion object {

        internal val libClass: Class<CodePointRangeIteratorResultLib> = CodePointRangeIteratorResultLib::class.java
        internal val lib: CodePointRangeIteratorResultLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CodePointRangeIteratorResultNative::class.java).toLong()

        internal fun fromNative(nativeStruct: CodePointRangeIteratorResultNative): CodePointRangeIteratorResult {
            val start: Int = nativeStruct.start
            val end: Int = nativeStruct.end
            val done: Boolean = nativeStruct.done > 0

            return CodePointRangeIteratorResult(start, end, done)
        }

    }
}
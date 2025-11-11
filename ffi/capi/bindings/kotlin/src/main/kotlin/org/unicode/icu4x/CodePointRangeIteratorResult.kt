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

/** Result of a single iteration of [CodePointRangeIterator].
*Logically can be considered to be an `Option<RangeInclusive<DiplomatChar>>`,
*
*`start` and `end` represent an inclusive range of code points `[start, end]`,
*and `done` will be true if the iterator has already finished. The last contentful
*iteration will NOT produce a range `done=true`, in other words `start` and `end` are useful
*values if and only if `done=false`.
*/
class CodePointRangeIteratorResult internal constructor (
    internal val nativeStruct: CodePointRangeIteratorResultNative) {
    val start: Int = nativeStruct.start
    val end: Int = nativeStruct.end
    val done: Boolean = nativeStruct.done > 0

    companion object {
        internal val libClass: Class<CodePointRangeIteratorResultLib> = CodePointRangeIteratorResultLib::class.java
        internal val lib: CodePointRangeIteratorResultLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CodePointRangeIteratorResultNative::class.java).toLong()
    }

}

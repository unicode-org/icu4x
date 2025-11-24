package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CodePointRangeIteratorLib: Library {
    fun icu4x_CodePointRangeIterator_destroy_mv1(handle: Pointer)
    fun icu4x_CodePointRangeIterator_next_mv1(handle: Pointer): CodePointRangeIteratorResultNative
}
/** An iterator over code point ranges, produced by `CodePointSetData` or
*one of the `CodePointMapData` types
*/
class CodePointRangeIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class CodePointRangeIteratorCleaner(val handle: Pointer, val lib: CodePointRangeIteratorLib) : Runnable {
        override fun run() {
            lib.icu4x_CodePointRangeIterator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CodePointRangeIteratorLib> = CodePointRangeIteratorLib::class.java
        internal val lib: CodePointRangeIteratorLib = Native.load("icu4x", libClass)
    }
    
    /** Advance the iterator by one and return the next range.
    *
    *If the iterator is out of items, `done` will be true
    */
    fun next(): CodePointRangeIteratorResult {
        
        val returnVal = lib.icu4x_CodePointRangeIterator_next_mv1(handle);
        
        val returnStruct = CodePointRangeIteratorResult(returnVal)
        return returnStruct
    }

}
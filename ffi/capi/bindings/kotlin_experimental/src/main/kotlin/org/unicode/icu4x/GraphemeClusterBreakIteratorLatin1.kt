package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface GraphemeClusterBreakIteratorLatin1Lib: Library {
    fun icu4x_GraphemeClusterBreakIteratorLatin1_destroy_mv1(handle: Pointer)
    fun icu4x_GraphemeClusterBreakIteratorLatin1_next_mv1(handle: Pointer): Int
}
/** See the [Rust documentation for `GraphemeClusterBreakIterator`](https://docs.rs/icu/2.1.1/icu/segmenter/iterators/struct.GraphemeClusterBreakIterator.html) for more information.
*/
class GraphemeClusterBreakIteratorLatin1 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class GraphemeClusterBreakIteratorLatin1Cleaner(val handle: Pointer, val lib: GraphemeClusterBreakIteratorLatin1Lib) : Runnable {
        override fun run() {
            lib.icu4x_GraphemeClusterBreakIteratorLatin1_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<GraphemeClusterBreakIteratorLatin1Lib> = GraphemeClusterBreakIteratorLatin1Lib::class.java
        internal val lib: GraphemeClusterBreakIteratorLatin1Lib = Native.load("icu4x", libClass)
    }
    
    /** Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
    *out of range of a 32-bit signed integer.
    *
    *See the [Rust documentation for `next`](https://docs.rs/icu/2.1.1/icu/segmenter/iterators/struct.GraphemeClusterBreakIterator.html#method.next) for more information.
    */
    fun next(): Int {
        
        val returnVal = lib.icu4x_GraphemeClusterBreakIteratorLatin1_next_mv1(handle);
        return (returnVal)
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface GraphemeClusterSegmenterLib: Library {
    fun icu4x_GraphemeClusterSegmenter_destroy_mv1(handle: Pointer)
    fun icu4x_GraphemeClusterSegmenter_create_mv1(): Pointer
    fun icu4x_GraphemeClusterSegmenter_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_GraphemeClusterSegmenter_segment_utf16_mv1(handle: Pointer, input: Slice): Pointer
}
/** An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints
*in strings.
*
*See the [Rust documentation for `GraphemeClusterSegmenter`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.GraphemeClusterSegmenter.html) for more information.
*/
class GraphemeClusterSegmenter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class GraphemeClusterSegmenterCleaner(val handle: Pointer, val lib: GraphemeClusterSegmenterLib) : Runnable {
        override fun run() {
            lib.icu4x_GraphemeClusterSegmenter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, GraphemeClusterSegmenter.GraphemeClusterSegmenterCleaner(handle, GraphemeClusterSegmenter.lib));
    }

    companion object {
        internal val libClass: Class<GraphemeClusterSegmenterLib> = GraphemeClusterSegmenterLib::class.java
        internal val lib: GraphemeClusterSegmenterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct an [GraphemeClusterSegmenter] using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.new) for more information.
        */
        fun create(): GraphemeClusterSegmenter {
            
            val returnVal = lib.icu4x_GraphemeClusterSegmenter_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = GraphemeClusterSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct an [GraphemeClusterSegmenter].
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<GraphemeClusterSegmenter> {
            
            val returnVal = lib.icu4x_GraphemeClusterSegmenter_create_with_provider_mv1(provider.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = GraphemeClusterSegmenter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** Segments a string.
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.GraphemeClusterSegmenterBorrowed.html#method.segment_utf16) for more information.
    */
    fun segment(input: String): GraphemeClusterBreakIteratorUtf16 {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        val inputSliceMemory = PrimitiveArrayTools.borrowUtf16(input).into(listOf(aEdges))
        
        val returnVal = lib.icu4x_GraphemeClusterSegmenter_segment_utf16_mv1(handle, inputSliceMemory.slice);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = GraphemeClusterBreakIteratorUtf16(handle, selfEdges, aEdges, true)
        return returnOpaque
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LineSegmenterLib: Library {
    fun icu4x_LineSegmenter_destroy_mv1(handle: Pointer)
    fun icu4x_LineSegmenter_create_auto_mv1(): Pointer
    fun icu4x_LineSegmenter_create_lstm_mv1(): Pointer
    fun icu4x_LineSegmenter_create_dictionary_mv1(): Pointer
    fun icu4x_LineSegmenter_segment_utf16_mv1(handle: Pointer, input: Slice): Pointer
}
/** An ICU4X line-break segmenter, capable of finding breakpoints in strings.
*
*See the [Rust documentation for `LineSegmenter`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html) for more information.
*/
class LineSegmenter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LineSegmenterCleaner(val handle: Pointer, val lib: LineSegmenterLib) : Runnable {
        override fun run() {
            lib.icu4x_LineSegmenter_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LineSegmenterLib> = LineSegmenterLib::class.java
        internal val lib: LineSegmenterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a [LineSegmenter] with default options (no locale-based tailoring) using compiled data. It automatically loads the best
        *available payload data for Burmese, Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
        */
        fun createAuto(): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_auto_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, LineSegmenter.LineSegmenterCleaner(handle, LineSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with default options (no locale-based tailoring) and LSTM payload data for
        *Burmese, Khmer, Lao, and Thai, using compiled data.
        *
        *See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
        */
        fun createLstm(): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_lstm_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, LineSegmenter.LineSegmenterCleaner(handle, LineSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with default options (no locale-based tailoring) and dictionary payload data for
        *Burmese, Khmer, Lao, and Thai, using compiled data
        *
        *See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
        */
        fun createDictionary(): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_dictionary_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, LineSegmenter.LineSegmenterCleaner(handle, LineSegmenter.lib));
            return returnOpaque
        }
    }
    
    /** Segments a string.
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenterBorrowed.html#method.segment_utf16) for more information.
    */
    fun segment(input: String): LineBreakIteratorUtf16 {
        val (inputMem, inputSlice) = PrimitiveArrayTools.borrowUtf16(input)
        
        val returnVal = lib.icu4x_LineSegmenter_segment_utf16_mv1(handle, inputSlice);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this) + listOf(inputMem)
        val handle = returnVal 
        val returnOpaque = LineBreakIteratorUtf16(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, LineBreakIteratorUtf16.LineBreakIteratorUtf16Cleaner(handle, LineBreakIteratorUtf16.lib));
        return returnOpaque
    }

}
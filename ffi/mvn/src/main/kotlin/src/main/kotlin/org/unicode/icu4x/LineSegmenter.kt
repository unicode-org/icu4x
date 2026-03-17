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
    fun icu4x_LineSegmenter_create_for_non_complex_scripts_mv1(): Pointer
    fun icu4x_LineSegmenter_create_auto_with_options_v2_mv1(contentLocale: Pointer?, options: LineBreakOptionsNative): Pointer
    fun icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1(provider: Pointer, contentLocale: Pointer?, options: LineBreakOptionsNative): ResultPointerInt
    fun icu4x_LineSegmenter_create_lstm_with_options_v2_mv1(contentLocale: Pointer?, options: LineBreakOptionsNative): Pointer
    fun icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1(provider: Pointer, contentLocale: Pointer?, options: LineBreakOptionsNative): ResultPointerInt
    fun icu4x_LineSegmenter_create_dictionary_with_options_v2_mv1(contentLocale: Pointer?, options: LineBreakOptionsNative): Pointer
    fun icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1(provider: Pointer, contentLocale: Pointer?, options: LineBreakOptionsNative): ResultPointerInt
    fun icu4x_LineSegmenter_create_for_non_complex_scripts_with_options_v2_mv1(contentLocale: Pointer?, options: LineBreakOptionsNative): Pointer
    fun icu4x_LineSegmenter_create_for_non_complex_scripts_with_options_v2_and_provider_mv1(provider: Pointer, contentLocale: Pointer?, options: LineBreakOptionsNative): ResultPointerInt
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
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class LineSegmenterCleaner(val handle: Pointer, val lib: LineSegmenterLib) : Runnable {
        override fun run() {
            lib.icu4x_LineSegmenter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, LineSegmenter.LineSegmenterCleaner(handle, LineSegmenter.lib));
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
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
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
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
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
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with default options (no locale-based tailoring) and no support for scripts requiring complex context dependent line breaks
        *(Burmese, Khmer, Lao, and Thai), using compiled data
        *
        *See the [Rust documentation for `new_for_non_complex_scripts`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_for_non_complex_scripts) for more information.
        */
        fun createForNonComplexScripts(): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_for_non_complex_scripts_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options using compiled data. It automatically loads the best
        *available payload data for Burmese, Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
        */
        fun auto_with_options(contentLocale: Locale?, options: LineBreakOptions): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_auto_with_options_v2_mv1(contentLocale?.handle, options.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options. It automatically loads the best
        *available payload data for Burmese, Khmer, Lao, and Thai, using a particular data source.
        *
        *See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
        */
        fun auto_with_options_and_provider(provider: DataProvider, contentLocale: Locale?, options: LineBreakOptions): Result<LineSegmenter> {
            
            val returnVal = lib.icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1(provider.handle, contentLocale?.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = LineSegmenter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options and LSTM payload data for
        *Burmese, Khmer, Lao, and Thai, using compiled data.
        *
        *See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
        */
        fun lstm_with_options(contentLocale: Locale?, options: LineBreakOptions): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_lstm_with_options_v2_mv1(contentLocale?.handle, options.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options and LSTM payload data for
        *Burmese, Khmer, Lao, and Thai, using a particular data source.
        *
        *See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
        */
        fun lstm_with_options_and_provider(provider: DataProvider, contentLocale: Locale?, options: LineBreakOptions): Result<LineSegmenter> {
            
            val returnVal = lib.icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1(provider.handle, contentLocale?.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = LineSegmenter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options and dictionary payload data for
        *Burmese, Khmer, Lao, and Thai, using compiled data.
        *
        *See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
        */
        fun dictionary_with_options(contentLocale: Locale?, options: LineBreakOptions): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_dictionary_with_options_v2_mv1(contentLocale?.handle, options.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options and dictionary payload data for
        *Burmese, Khmer, Lao, and Thai, using a particular data source.
        *
        *See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
        */
        fun dictionary_with_options_and_provider(provider: DataProvider, contentLocale: Locale?, options: LineBreakOptions): Result<LineSegmenter> {
            
            val returnVal = lib.icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1(provider.handle, contentLocale?.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = LineSegmenter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options and no support for scripts requiring complex context dependent line breaks
        *(Burmese, Khmer, Lao, and Thai), using compiled data.
        *
        *See the [Rust documentation for `new_for_non_complex_scripts`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_for_non_complex_scripts) for more information.
        */
        fun for_non_complex_scripts_with_options(contentLocale: Locale?, options: LineBreakOptions): LineSegmenter {
            
            val returnVal = lib.icu4x_LineSegmenter_create_for_non_complex_scripts_with_options_v2_mv1(contentLocale?.handle, options.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LineSegmenter(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [LineSegmenter] with custom options and no support for complex languages
        *(Burmese, Khmer, Lao, and Thai), using a particular data source.
        *
        *See the [Rust documentation for `new_for_non_complex_scripts`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.new_for_non_complex_scripts) for more information.
        */
        fun for_non_complex_scripts_with_options_and_provider(provider: DataProvider, contentLocale: Locale?, options: LineBreakOptions): Result<LineSegmenter> {
            
            val returnVal = lib.icu4x_LineSegmenter_create_for_non_complex_scripts_with_options_v2_and_provider_mv1(provider.handle, contentLocale?.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = LineSegmenter(handle, selfEdges, true)
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
    *See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenterBorrowed.html#method.segment_utf16) for more information.
    */
    fun segment(input: String): LineBreakIteratorUtf16 {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        val inputSliceMemory = PrimitiveArrayTools.borrowUtf16(input).into(listOf(aEdges))
        
        val returnVal = lib.icu4x_LineSegmenter_segment_utf16_mv1(handle, inputSliceMemory.slice);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = LineBreakIteratorUtf16(handle, selfEdges, aEdges, true)
        return returnOpaque
    }

}
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
    fun icu4x_LineSegmenter_load_lstm_models_with_provider_mv1(handle: Pointer, provider: Pointer): ResultUnitInt
    fun icu4x_LineSegmenter_load_dictinoary_models_with_provider_mv1(handle: Pointer, provider: Pointer): ResultUnitInt
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
            val returnOpaque = LineSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, LineSegmenter.LineSegmenterCleaner(handle, LineSegmenter.lib));
            return returnOpaque
        }
    }
    
    /** Loads available LSMT models from the given provider.
    *
    *See the [Rust documentation for `load_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.load_lstm) for more information.
    */
    fun loadLstmModelsWithProvider(provider: DataProvider): Result<Unit> {
        
        val returnVal = lib.icu4x_LineSegmenter_load_lstm_models_with_provider_mv1(handle, provider.handle);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** Loads available dictionary models from the given provider.
    *
    *See the [Rust documentation for `load_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.LineSegmenter.html#method.load_dictionary) for more information.
    */
    fun loadDictinoaryModelsWithProvider(provider: DataProvider): Result<Unit> {
        
        val returnVal = lib.icu4x_LineSegmenter_load_dictinoary_models_with_provider_mv1(handle, provider.handle);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
        }
    }

}
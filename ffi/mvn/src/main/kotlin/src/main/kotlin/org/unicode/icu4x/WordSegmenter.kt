package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface WordSegmenterLib: Library {
    fun icu4x_WordSegmenter_destroy_mv1(handle: Pointer)
    fun icu4x_WordSegmenter_create_auto_mv1(): Pointer
    fun icu4x_WordSegmenter_create_auto_with_content_locale_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_auto_with_content_locale_and_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_lstm_mv1(): Pointer
    fun icu4x_WordSegmenter_create_lstm_with_content_locale_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_lstm_with_content_locale_and_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_dictionary_mv1(): Pointer
    fun icu4x_WordSegmenter_create_dictionary_with_content_locale_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_dictionary_with_content_locale_and_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_for_non_complex_scripts_mv1(): Pointer
    fun icu4x_WordSegmenter_create_for_non_complex_scripts_with_content_locale_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_create_for_non_complex_scripts_with_content_locale_and_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_WordSegmenter_segment_utf16_mv1(handle: Pointer, input: Slice): Pointer
}
/** An ICU4X word-break segmenter, capable of finding word breakpoints in strings.
*
*See the [Rust documentation for `WordSegmenter`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html) for more information.
*/
class WordSegmenter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class WordSegmenterCleaner(val handle: Pointer, val lib: WordSegmenterLib) : Runnable {
        override fun run() {
            lib.icu4x_WordSegmenter_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<WordSegmenterLib> = WordSegmenterLib::class.java
        internal val lib: WordSegmenterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a [WordSegmenter] with automatically selecting the best available LSTM
        *or dictionary payload data, using compiled data. This does not assume any content locale.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `new_auto`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.new_auto) for more information.
        */
        fun createAuto(): WordSegmenter {
            
            val returnVal = lib.icu4x_WordSegmenter_create_auto_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = WordSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with automatically selecting the best available LSTM
        *or dictionary payload data, using compiled data.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `try_new_auto`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_auto) for more information.
        */
        fun createAutoWithContentLocale(locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_auto_with_content_locale_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with automatically selecting the best available LSTM
        *or dictionary payload data, using a particular data source.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `try_new_auto`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_auto) for more information.
        */
        fun createAutoWithContentLocaleAndProvider(provider: DataProvider, locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_auto_with_content_locale_and_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with LSTM payload data for Burmese, Khmer, Lao, and
        *Thai, using compiled data. This does not assume any content locale.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `new_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.new_lstm) for more information.
        */
        fun createLstm(): WordSegmenter {
            
            val returnVal = lib.icu4x_WordSegmenter_create_lstm_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = WordSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with LSTM payload data for Burmese, Khmer, Lao, and
        *Thai, using compiled data.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `try_new_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_lstm) for more information.
        */
        fun createLstmWithContentLocale(locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_lstm_with_content_locale_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with LSTM payload data for Burmese, Khmer, Lao, and
        *Thai, using a particular data source.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `try_new_lstm`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_lstm) for more information.
        */
        fun createLstmWithContentLocaleAndProvider(provider: DataProvider, locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_lstm_with_content_locale_and_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with dictionary payload data for Chinese, Japanese,
        *Burmese, Khmer, Lao, and Thai, using compiled data. This does not assume any content locale.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and dictionary for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.new_dictionary) for more information.
        */
        fun createDictionary(): WordSegmenter {
            
            val returnVal = lib.icu4x_WordSegmenter_create_dictionary_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = WordSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with dictionary payload data for Chinese, Japanese,
        *Burmese, Khmer, Lao, and Thai, using compiled data.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and dictionary for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `try_new_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_dictionary) for more information.
        */
        fun createDictionaryWithContentLocale(locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_dictionary_with_content_locale_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with dictionary payload data for Chinese, Japanese,
        *Burmese, Khmer, Lao, and Thai, using a particular data source.
        *
        *Note: currently, it uses dictionary for Chinese and Japanese, and dictionary for Burmese,
        *Khmer, Lao, and Thai.
        *
        *See the [Rust documentation for `try_new_dictionary`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_dictionary) for more information.
        */
        fun createDictionaryWithContentLocaleAndProvider(provider: DataProvider, locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_dictionary_with_content_locale_and_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with no support for complex scripts (Chinese, Japanese,
        *Burmese, Khmer, Lao, and Thai), using compiled data. This does not assume any content locale.
        *
        *See the [Rust documentation for `new_for_non_complex_scripts`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.new_for_non_complex_scripts) for more information.
        */
        fun createForNonComplexScripts(): WordSegmenter {
            
            val returnVal = lib.icu4x_WordSegmenter_create_for_non_complex_scripts_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = WordSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with no support for complex scripts (Chinese, Japanese,
        *Burmese, Khmer, Lao, and Thai), using compiled data.
        *
        *See the [Rust documentation for `try_new_for_non_complex_scripts`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_for_non_complex_scripts) for more information.
        */
        fun createForNonComplexScriptsWithContentLocale(locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_for_non_complex_scripts_with_content_locale_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [WordSegmenter] with no support for complex scripts (Chinese, Japanese,
        *Burmese, Khmer, Lao, and Thai), using a particular data source.
        *
        *See the [Rust documentation for `try_new_for_non_complex_scripts`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenter.html#method.try_new_for_non_complex_scripts) for more information.
        */
        fun createForNonComplexScriptsWithContentLocaleAndProvider(provider: DataProvider, locale: Locale): Result<WordSegmenter> {
            
            val returnVal = lib.icu4x_WordSegmenter_create_for_non_complex_scripts_with_content_locale_and_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WordSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, WordSegmenter.WordSegmenterCleaner(handle, WordSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Segments a string.
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.WordSegmenterBorrowed.html#method.segment_utf16) for more information.
    */
    fun segment(input: String): WordBreakIteratorUtf16 {
        val (inputMem, inputSlice) = PrimitiveArrayTools.borrowUtf16(input)
        
        val returnVal = lib.icu4x_WordSegmenter_segment_utf16_mv1(handle, inputSlice);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this) + listOf(inputMem)
        val handle = returnVal 
        val returnOpaque = WordBreakIteratorUtf16(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, WordBreakIteratorUtf16.WordBreakIteratorUtf16Cleaner(handle, WordBreakIteratorUtf16.lib));
        return returnOpaque
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface SentenceSegmenterLib: Library {
    fun icu4x_SentenceSegmenter_destroy_mv1(handle: Pointer)
    fun icu4x_SentenceSegmenter_create_mv1(): Pointer
    fun icu4x_SentenceSegmenter_create_with_content_locale_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_SentenceSegmenter_create_with_content_locale_and_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
}
/** An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.
*
*See the [Rust documentation for `SentenceSegmenter`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.SentenceSegmenter.html) for more information.
*/
class SentenceSegmenter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class SentenceSegmenterCleaner(val handle: Pointer, val lib: SentenceSegmenterLib) : Runnable {
        override fun run() {
            lib.icu4x_SentenceSegmenter_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<SentenceSegmenterLib> = SentenceSegmenterLib::class.java
        internal val lib: SentenceSegmenterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a [SentenceSegmenter] using compiled data. This does not assume any content locale.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/segmenter/struct.SentenceSegmenter.html#method.new) for more information.
        */
        fun create(): SentenceSegmenter {
            
            val returnVal = lib.icu4x_SentenceSegmenter_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = SentenceSegmenter(handle, selfEdges)
            CLEANER.register(returnOpaque, SentenceSegmenter.SentenceSegmenterCleaner(handle, SentenceSegmenter.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a [SentenceSegmenter] for content known to be of a given locale, using compiled data.
        */
        fun createWithContentLocale(locale: Locale): Result<SentenceSegmenter> {
            
            val returnVal = lib.icu4x_SentenceSegmenter_create_with_content_locale_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = SentenceSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, SentenceSegmenter.SentenceSegmenterCleaner(handle, SentenceSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [SentenceSegmenter]  for content known to be of a given locale, using a particular data source.
        */
        fun createWithContentLocaleAndProvider(provider: DataProvider, locale: Locale): Result<SentenceSegmenter> {
            
            val returnVal = lib.icu4x_SentenceSegmenter_create_with_content_locale_and_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = SentenceSegmenter(handle, selfEdges)
                CLEANER.register(returnOpaque, SentenceSegmenter.SentenceSegmenterCleaner(handle, SentenceSegmenter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }

}
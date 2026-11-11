package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationFormatterLib: Library {
    fun icu4x_DurationFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_DurationFormatter_create_v1_mv1(locale: Pointer, options: DurationFormatterOptionsNative): ResultPointerInt
    fun icu4x_DurationFormatter_create_v1_with_provider_mv1(provider: Pointer, locale: Pointer, options: DurationFormatterOptionsNative): ResultPointerInt
    fun icu4x_DurationFormatter_format_mv1(handle: Pointer, duration: DurationNative, write: Pointer): Unit
}
/**
 * See the [Rust documentation for `DurationFormatter`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationFormatter internal constructor (
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

    private class DurationFormatterCleaner(val handle: Pointer, val lib: DurationFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_DurationFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, DurationFormatter.DurationFormatterCleaner(handle, DurationFormatter.lib));
    }

    companion object {
        internal val libClass: Class<DurationFormatterLib> = DurationFormatterLib::class.java
        internal val lib: DurationFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /**
         * Creates a new [DurationFormatter] from locale data and an options bag using compiled data.
         *
         * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.try_new) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun create(locale: Locale, options: DurationFormatterOptions): Result<DurationFormatter> {
            
            val returnVal = lib.icu4x_DurationFormatter_create_v1_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DurationFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DurationFormatterErrorError(DurationFormatterError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Creates a new [DurationFormatter] from locale data and an options bag using a particular data source.
         *
         * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.try_new) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun create_with_provider(provider: DataProvider, locale: Locale, options: DurationFormatterOptions): Result<DurationFormatter> {
            
            val returnVal = lib.icu4x_DurationFormatter_create_v1_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DurationFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DurationFormatterErrorError(DurationFormatterError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /**
     * Formats a [Duration] to a string.
     *
     * See the [Rust documentation for `format`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.format) for more information.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun format(duration: Duration): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DurationFormatter_format_mv1(handle, duration.toNative(), write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
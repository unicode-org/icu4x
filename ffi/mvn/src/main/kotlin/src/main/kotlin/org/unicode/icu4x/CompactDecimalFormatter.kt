package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CompactDecimalFormatterLib: Library {
    fun icu4x_CompactDecimalFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_CompactDecimalFormatter_create_short_mv1(locale: Pointer, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_CompactDecimalFormatter_create_short_with_provider_mv1(provider: Pointer, locale: Pointer, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_CompactDecimalFormatter_create_long_mv1(locale: Pointer, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_CompactDecimalFormatter_create_long_with_provider_mv1(provider: Pointer, locale: Pointer, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_CompactDecimalFormatter_compact_exponent_for_magnitude_mv1(handle: Pointer, magnitude: Short): FFIUint8
    fun icu4x_CompactDecimalFormatter_format_mv1(handle: Pointer, value: Pointer, write: Pointer): Unit
}
/**
 * An ICU4X Compact Decimal Format object, capable of formatting a [Decimal] in compact notation
 * and querying locale-specific compact exponents.
 *
 * See the [Rust documentation for `CompactDecimalFormatter`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class CompactDecimalFormatter internal constructor (
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

    private class CompactDecimalFormatterCleaner(val handle: Pointer, val lib: CompactDecimalFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_CompactDecimalFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, CompactDecimalFormatter.CompactDecimalFormatterCleaner(handle, CompactDecimalFormatter.lib));
    }

    companion object {
        internal val libClass: Class<CompactDecimalFormatterLib> = CompactDecimalFormatterLib::class.java
        internal val lib: CompactDecimalFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /**
         * Creates a new short [CompactDecimalFormatter], using compiled data.
         *
         * See the [Rust documentation for `try_new_short`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_short) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShort(locale: Locale, groupingStrategy: DecimalGroupingStrategy?): Result<CompactDecimalFormatter> {
            
            val returnVal = lib.icu4x_CompactDecimalFormatter_create_short_mv1(locale.handle, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = CompactDecimalFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Creates a new short [CompactDecimalFormatter], using a particular data source.
         *
         * See the [Rust documentation for `try_new_short`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_short) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortWithProvider(provider: DataProvider, locale: Locale, groupingStrategy: DecimalGroupingStrategy?): Result<CompactDecimalFormatter> {
            
            val returnVal = lib.icu4x_CompactDecimalFormatter_create_short_with_provider_mv1(provider.handle, locale.handle, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = CompactDecimalFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Creates a new long [CompactDecimalFormatter], using compiled data.
         *
         * See the [Rust documentation for `try_new_long`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_long) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLong(locale: Locale, groupingStrategy: DecimalGroupingStrategy?): Result<CompactDecimalFormatter> {
            
            val returnVal = lib.icu4x_CompactDecimalFormatter_create_long_mv1(locale.handle, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = CompactDecimalFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Creates a new long [CompactDecimalFormatter], using a particular data source.
         *
         * See the [Rust documentation for `try_new_long`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_long) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongWithProvider(provider: DataProvider, locale: Locale, groupingStrategy: DecimalGroupingStrategy?): Result<CompactDecimalFormatter> {
            
            val returnVal = lib.icu4x_CompactDecimalFormatter_create_long_with_provider_mv1(provider.handle, locale.handle, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = CompactDecimalFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /**
     * Returns the compact decimal exponent that should be used for a number of
     * the given magnitude when using this formatter.
     *
     * See the [Rust documentation for `compact_exponent_for_magnitude`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.compact_exponent_for_magnitude) for more information.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun compactExponentForMagnitude(magnitude: Short): UByte {
        
        val returnVal = lib.icu4x_CompactDecimalFormatter_compact_exponent_for_magnitude_mv1(handle, magnitude);
        return (returnVal.toUByte())
    }
    
    /**
     * Formats a [Decimal] in compact notation to a string.
     *
     * See the [Rust documentation for `format`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.format) for more information.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun format(value: Decimal): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_CompactDecimalFormatter_format_mv1(handle, value.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
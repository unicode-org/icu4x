package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalFormatterLib: Library {
    fun icu4x_DecimalFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_DecimalFormatter_create_with_grouping_strategy_mv1(locale: Pointer, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_DecimalFormatter_create_with_grouping_strategy_and_provider_mv1(provider: Pointer, locale: Pointer, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_DecimalFormatter_create_with_manual_data_mv1(plusSignPrefix: Slice, plusSignSuffix: Slice, minusSignPrefix: Slice, minusSignSuffix: Slice, decimalSeparator: Slice, groupingSeparator: Slice, primaryGroupSize: FFIUint8, secondaryGroupSize: FFIUint8, minGroupSize: FFIUint8, digits: Slice, groupingStrategy: OptionInt): ResultPointerInt
    fun icu4x_DecimalFormatter_format_mv1(handle: Pointer, value: Pointer, write: Pointer): Unit
}
/** An ICU4X Decimal Format object, capable of formatting a [Decimal] as a string.
*
*See the [Rust documentation for `DecimalFormatter`](https://docs.rs/icu/2.1.1/icu/decimal/struct.DecimalFormatter.html) for more information.
*/
class DecimalFormatter internal constructor (
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

    private class DecimalFormatterCleaner(val handle: Pointer, val lib: DecimalFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_DecimalFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, DecimalFormatter.DecimalFormatterCleaner(handle, DecimalFormatter.lib));
    }

    companion object {
        internal val libClass: Class<DecimalFormatterLib> = DecimalFormatterLib::class.java
        internal val lib: DecimalFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [DecimalFormatter], using compiled data
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/decimal/struct.DecimalFormatter.html#method.try_new) for more information.
        */
        fun createWithGroupingStrategy(locale: Locale, groupingStrategy: DecimalGroupingStrategy?): Result<DecimalFormatter> {
            
            val returnVal = lib.icu4x_DecimalFormatter_create_with_grouping_strategy_mv1(locale.handle, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DecimalFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [DecimalFormatter], using a particular data source.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/decimal/struct.DecimalFormatter.html#method.try_new) for more information.
        */
        fun createWithGroupingStrategyAndProvider(provider: DataProvider, locale: Locale, groupingStrategy: DecimalGroupingStrategy?): Result<DecimalFormatter> {
            
            val returnVal = lib.icu4x_DecimalFormatter_create_with_grouping_strategy_and_provider_mv1(provider.handle, locale.handle, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DecimalFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [DecimalFormatter] from preconstructed locale data.
        *
        *See the [Rust documentation for `DecimalSymbolsV1`](https://docs.rs/icu/2.1.1/icu/decimal/provider/struct.DecimalSymbolsV1.html) for more information.
        */
        fun createWithManualData(plusSignPrefix: String, plusSignSuffix: String, minusSignPrefix: String, minusSignSuffix: String, decimalSeparator: String, groupingSeparator: String, primaryGroupSize: UByte, secondaryGroupSize: UByte, minGroupSize: UByte, digits: IntArray, groupingStrategy: DecimalGroupingStrategy?): Result<DecimalFormatter> {
            val plusSignPrefixSliceMemory = PrimitiveArrayTools.borrowUtf8(plusSignPrefix)
            val plusSignSuffixSliceMemory = PrimitiveArrayTools.borrowUtf8(plusSignSuffix)
            val minusSignPrefixSliceMemory = PrimitiveArrayTools.borrowUtf8(minusSignPrefix)
            val minusSignSuffixSliceMemory = PrimitiveArrayTools.borrowUtf8(minusSignSuffix)
            val decimalSeparatorSliceMemory = PrimitiveArrayTools.borrowUtf8(decimalSeparator)
            val groupingSeparatorSliceMemory = PrimitiveArrayTools.borrowUtf8(groupingSeparator)
            val digitsSliceMemory = PrimitiveArrayTools.borrow(digits)
            
            val returnVal = lib.icu4x_DecimalFormatter_create_with_manual_data_mv1(plusSignPrefixSliceMemory.slice, plusSignSuffixSliceMemory.slice, minusSignPrefixSliceMemory.slice, minusSignSuffixSliceMemory.slice, decimalSeparatorSliceMemory.slice, groupingSeparatorSliceMemory.slice, FFIUint8(primaryGroupSize), FFIUint8(secondaryGroupSize), FFIUint8(minGroupSize), digitsSliceMemory.slice, groupingStrategy?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    val selfEdges: List<Any> = listOf()
                    val handle = nativeOkVal 
                    val returnOpaque = DecimalFormatter(handle, selfEdges, true)
                    return returnOpaque.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                plusSignPrefixSliceMemory.close()
                plusSignSuffixSliceMemory.close()
                minusSignPrefixSliceMemory.close()
                minusSignSuffixSliceMemory.close()
                decimalSeparatorSliceMemory.close()
                groupingSeparatorSliceMemory.close()
                digitsSliceMemory.close()
            }
        }
    }
    
    /** Formats a [Decimal] to a string.
    *
    *See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/decimal/struct.DecimalFormatter.html#method.format) for more information.
    */
    fun format(value: Decimal): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DecimalFormatter_format_mv1(handle, value.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
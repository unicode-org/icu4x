package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface VariantOffsetsCalculatorLib: Library {
    fun icu4x_VariantOffsetsCalculator_destroy_mv1(handle: Pointer)
    fun icu4x_VariantOffsetsCalculator_create_mv1(): Pointer
    fun icu4x_VariantOffsetsCalculator_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1(handle: Pointer, timeZone: Pointer, utcDate: Pointer, utcTime: Pointer): OptionVariantOffsetsNative
    fun icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_timestamp_mv1(handle: Pointer, timeZone: Pointer, timestamp: Long): OptionVariantOffsetsNative
}
/** See the [Rust documentation for `VariantOffsetsCalculator`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.VariantOffsetsCalculator.html) for more information.
*/
class VariantOffsetsCalculator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class VariantOffsetsCalculatorCleaner(val handle: Pointer, val lib: VariantOffsetsCalculatorLib) : Runnable {
        override fun run() {
            lib.icu4x_VariantOffsetsCalculator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<VariantOffsetsCalculatorLib> = VariantOffsetsCalculatorLib::class.java
        internal val lib: VariantOffsetsCalculatorLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new [VariantOffsetsCalculator] instance using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.VariantOffsetsCalculator.html#method.new) for more information.
        */
        fun create(): VariantOffsetsCalculator {
            
            val returnVal = lib.icu4x_VariantOffsetsCalculator_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = VariantOffsetsCalculator(handle, selfEdges)
            CLEANER.register(returnOpaque, VariantOffsetsCalculator.VariantOffsetsCalculatorCleaner(handle, VariantOffsetsCalculator.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new [VariantOffsetsCalculator] instance using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.VariantOffsetsCalculator.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<VariantOffsetsCalculator> {
            
            val returnVal = lib.icu4x_VariantOffsetsCalculator_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = VariantOffsetsCalculator(handle, selfEdges)
                CLEANER.register(returnOpaque, VariantOffsetsCalculator.VariantOffsetsCalculatorCleaner(handle, VariantOffsetsCalculator.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `compute_offsets_from_time_zone_and_name_timestamp`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.VariantOffsetsCalculatorBorrowed.html#method.compute_offsets_from_time_zone_and_name_timestamp) for more information.
    */
    fun computeOffsetsFromTimeZoneAndDateTime(timeZone: TimeZone, utcDate: IsoDate, utcTime: Time): VariantOffsets? {
        
        val returnVal = lib.icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1(handle, timeZone.handle, utcDate.handle, utcTime.handle);
        
        val intermediateOption = returnVal.option() ?: return null

        val returnStruct = VariantOffsets.fromNative(intermediateOption)
        return returnStruct
                                
    }
    
    /** See the [Rust documentation for `compute_offsets_from_time_zone_and_name_timestamp`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.VariantOffsetsCalculatorBorrowed.html#method.compute_offsets_from_time_zone_and_name_timestamp) for more information.
    */
    fun computeOffsetsFromTimeZoneAndTimestamp(timeZone: TimeZone, timestamp: Long): VariantOffsets? {
        
        val returnVal = lib.icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_timestamp_mv1(handle, timeZone.handle, timestamp);
        
        val intermediateOption = returnVal.option() ?: return null

        val returnStruct = VariantOffsets.fromNative(intermediateOption)
        return returnStruct
                                
    }

}
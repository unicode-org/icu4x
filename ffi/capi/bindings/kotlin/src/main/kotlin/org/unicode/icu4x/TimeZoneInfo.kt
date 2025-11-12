package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneInfoLib: Library {
    fun icu4x_TimeZoneInfo_destroy_mv1(handle: Pointer)
    fun icu4x_TimeZoneInfo_utc_mv1(): Pointer
    fun icu4x_TimeZoneInfo_id_mv1(handle: Pointer): Pointer
    fun icu4x_TimeZoneInfo_at_date_time_iso_mv1(handle: Pointer, date: Pointer, time: Pointer): Pointer
    fun icu4x_TimeZoneInfo_at_timestamp_mv1(handle: Pointer, timestamp: Long): Pointer
    fun icu4x_TimeZoneInfo_zone_name_date_time_mv1(handle: Pointer): OptionIsoDateTimeNative
    fun icu4x_TimeZoneInfo_with_variant_mv1(handle: Pointer, timeVariant: Int): Pointer
    fun icu4x_TimeZoneInfo_offset_mv1(handle: Pointer): Pointer?
    fun icu4x_TimeZoneInfo_infer_variant_mv1(handle: Pointer, offsetCalculator: Pointer): OptionUnit
    fun icu4x_TimeZoneInfo_variant_mv1(handle: Pointer): OptionInt
}
/** See the [Rust documentation for `TimeZoneInfo`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html) for more information.
*/
class TimeZoneInfo internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class TimeZoneInfoCleaner(val handle: Pointer, val lib: TimeZoneInfoLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeZoneInfo_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TimeZoneInfoLib> = TimeZoneInfoLib::class.java
        internal val lib: TimeZoneInfoLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a time zone for UTC (Coordinated Universal Time).
        *
        *See the [Rust documentation for `utc`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.utc) for more information.
        */
        fun utc(): TimeZoneInfo {
            
            val returnVal = lib.icu4x_TimeZoneInfo_utc_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = TimeZoneInfo(handle, selfEdges)
            CLEANER.register(returnOpaque, TimeZoneInfo.TimeZoneInfoCleaner(handle, TimeZoneInfo.lib));
            return returnOpaque
        }
    }
    
    /** See the [Rust documentation for `id`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.id) for more information.
    */
    fun id(): TimeZone {
        
        val returnVal = lib.icu4x_TimeZoneInfo_id_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZone(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZone.TimeZoneCleaner(handle, TimeZone.lib));
        return returnOpaque
    }
    
    /** Sets the datetime at which to interpret the time zone
    *for display name lookup.
    *
    *Notes:
    *
    *- If not set, the formatting datetime is used if possible.
    *- If the offset is not set, the datetime is interpreted as UTC.
    *- The constraints are the same as with `ZoneNameTimestamp` in Rust.
    *- Set to year 1000 or 9999 for a reference far in the past or future.
    *
    *See the [Rust documentation for `at_date_time_iso`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.at_date_time_iso) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.ZoneNameTimestamp.html)
    */
    fun atDateTimeIso(date: IsoDate, time: Time): TimeZoneInfo {
        
        val returnVal = lib.icu4x_TimeZoneInfo_at_date_time_iso_mv1(handle, date.handle, time.handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZoneInfo(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZoneInfo.TimeZoneInfoCleaner(handle, TimeZoneInfo.lib));
        return returnOpaque
    }
    
    /** Sets the timestamp, in milliseconds since Unix epoch, at which to interpret the time zone
    *for display name lookup.
    *
    *Notes:
    *
    *- If not set, the formatting datetime is used if possible.
    *- The constraints are the same as with `ZoneNameTimestamp` in Rust.
    *
    *See the [Rust documentation for `with_zone_name_timestamp`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.with_zone_name_timestamp) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.ZoneNameTimestamp.html#method.from_zoned_date_time_iso), [2](https://docs.rs/icu/2.1.1/icu/time/zone/struct.ZoneNameTimestamp.html)
    */
    fun atTimestamp(timestamp: Long): TimeZoneInfo {
        
        val returnVal = lib.icu4x_TimeZoneInfo_at_timestamp_mv1(handle, timestamp);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZoneInfo(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZoneInfo.TimeZoneInfoCleaner(handle, TimeZoneInfo.lib));
        return returnOpaque
    }
    
    /** Returns the DateTime for the UTC zone name reference time
    *
    *See the [Rust documentation for `zone_name_timestamp`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.zone_name_timestamp) for more information.
    */
    fun zoneNameDateTime(): IsoDateTime? {
        
        val returnVal = lib.icu4x_TimeZoneInfo_zone_name_date_time_mv1(handle);
        
        val intermediateOption = returnVal.option() ?: return null

        val returnStruct = IsoDateTime(intermediateOption)
        return returnStruct
                                
    }
    
    /** See the [Rust documentation for `with_variant`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.with_variant) for more information.
    */
    fun withVariant(timeVariant: TimeZoneVariant): TimeZoneInfo {
        
        val returnVal = lib.icu4x_TimeZoneInfo_with_variant_mv1(handle, timeVariant.toNative());
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZoneInfo(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZoneInfo.TimeZoneInfoCleaner(handle, TimeZoneInfo.lib));
        return returnOpaque
    }
    
    /** See the [Rust documentation for `offset`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.offset) for more information.
    */
    fun offset(): UtcOffset? {
        
        val returnVal = lib.icu4x_TimeZoneInfo_offset_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = UtcOffset(handle, selfEdges)
        CLEANER.register(returnOpaque, UtcOffset.UtcOffsetCleaner(handle, UtcOffset.lib));
        return returnOpaque
    }
    
    /** See the [Rust documentation for `infer_variant`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.infer_variant) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/enum.TimeZoneVariant.html)
    */
    fun inferVariant(offsetCalculator: VariantOffsetsCalculator): Unit? {
        
        val returnVal = lib.icu4x_TimeZoneInfo_infer_variant_mv1(handle, offsetCalculator.handle);
        returnVal.option() ?: return null
    }
    
    /** See the [Rust documentation for `variant`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.variant) for more information.
    */
    fun variant(): TimeZoneVariant? {
        
        val returnVal = lib.icu4x_TimeZoneInfo_variant_mv1(handle);
        
        val intermediateOption = returnVal.option() ?: return null
        return TimeZoneVariant.fromNative(intermediateOption)
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface UtcOffsetLib: Library {
    fun icu4x_UtcOffset_destroy_mv1(handle: Pointer)
    fun icu4x_UtcOffset_from_seconds_mv1(seconds: Int): ResultPointerTimeZoneInvalidOffsetErrorNative
    fun icu4x_UtcOffset_from_string_mv1(offset: Slice): ResultPointerTimeZoneInvalidOffsetErrorNative
    fun icu4x_UtcOffset_seconds_mv1(handle: Pointer): Int
    fun icu4x_UtcOffset_is_non_negative_mv1(handle: Pointer): Byte
    fun icu4x_UtcOffset_is_zero_mv1(handle: Pointer): Byte
    fun icu4x_UtcOffset_hours_part_mv1(handle: Pointer): Int
    fun icu4x_UtcOffset_minutes_part_mv1(handle: Pointer): FFIUint32
    fun icu4x_UtcOffset_seconds_part_mv1(handle: Pointer): FFIUint32
}
/** See the [Rust documentation for `UtcOffset`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html) for more information.
*/
class UtcOffset internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class UtcOffsetCleaner(val handle: Pointer, val lib: UtcOffsetLib) : Runnable {
        override fun run() {
            lib.icu4x_UtcOffset_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<UtcOffsetLib> = UtcOffsetLib::class.java
        internal val lib: UtcOffsetLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates an offset from seconds.
        *
        *Errors if the offset seconds are out of range.
        *
        *See the [Rust documentation for `try_from_seconds`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.try_from_seconds) for more information.
        */
        fun fromSeconds(seconds: Int): Result<UtcOffset> {
            
            val returnVal = lib.icu4x_UtcOffset_from_seconds_mv1(seconds);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = UtcOffset(handle, selfEdges)
                CLEANER.register(returnOpaque, UtcOffset.UtcOffsetCleaner(handle, UtcOffset.lib));
                return returnOpaque.ok()
            } else {
                return TimeZoneInvalidOffsetError().err()
            }
        }
        @JvmStatic
        
        /** Creates an offset from a string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.try_from_str) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
        */
        fun fromString(offset: String): Result<UtcOffset> {
            val offsetSliceMemory = PrimitiveArrayTools.borrowUtf8(offset)
            
            val returnVal = lib.icu4x_UtcOffset_from_string_mv1(offsetSliceMemory.slice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = UtcOffset(handle, selfEdges)
                CLEANER.register(returnOpaque, UtcOffset.UtcOffsetCleaner(handle, UtcOffset.lib));
                offsetSliceMemory?.close()
                return returnOpaque.ok()
            } else {
                return TimeZoneInvalidOffsetError().err()
            }
        }
    }
    
    /** Returns the value as offset seconds.
    *
    *See the [Rust documentation for `offset`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.offset) for more information.
    *
    *See the [Rust documentation for `to_seconds`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.to_seconds) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
    */
    fun seconds(): Int {
        
        val returnVal = lib.icu4x_UtcOffset_seconds_mv1(handle);
        return (returnVal)
    }
    
    /** Returns whether the offset is positive.
    *
    *See the [Rust documentation for `is_non_negative`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.is_non_negative) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
    */
    fun isNonNegative(): Boolean {
        
        val returnVal = lib.icu4x_UtcOffset_is_non_negative_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Returns whether the offset is zero.
    *
    *See the [Rust documentation for `is_zero`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.is_zero) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
    */
    fun isZero(): Boolean {
        
        val returnVal = lib.icu4x_UtcOffset_is_zero_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Returns the hours part of the offset.
    *
    *See the [Rust documentation for `hours_part`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.hours_part) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
    */
    fun hoursPart(): Int {
        
        val returnVal = lib.icu4x_UtcOffset_hours_part_mv1(handle);
        return (returnVal)
    }
    
    /** Returns the minutes part of the offset.
    *
    *See the [Rust documentation for `minutes_part`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.minutes_part) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
    */
    fun minutesPart(): UInt {
        
        val returnVal = lib.icu4x_UtcOffset_minutes_part_mv1(handle);
        return (returnVal.toUInt())
    }
    
    /** Returns the seconds part of the offset.
    *
    *See the [Rust documentation for `seconds_part`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html#method.seconds_part) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.UtcOffset.html)
    */
    fun secondsPart(): UInt {
        
        val returnVal = lib.icu4x_UtcOffset_seconds_part_mv1(handle);
        return (returnVal.toUInt())
    }

}
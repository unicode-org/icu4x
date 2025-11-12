package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeLib: Library {
    fun icu4x_Time_destroy_mv1(handle: Pointer)
    fun icu4x_Time_create_mv1(hour: FFIUint8, minute: FFIUint8, second: FFIUint8, subsecond: FFIUint32): ResultPointerInt
    fun icu4x_Time_from_string_mv1(v: Slice): ResultPointerInt
    fun icu4x_Time_start_of_day_mv1(): ResultPointerInt
    fun icu4x_Time_noon_mv1(): ResultPointerInt
    fun icu4x_Time_hour_mv1(handle: Pointer): FFIUint8
    fun icu4x_Time_minute_mv1(handle: Pointer): FFIUint8
    fun icu4x_Time_second_mv1(handle: Pointer): FFIUint8
    fun icu4x_Time_subsecond_mv1(handle: Pointer): FFIUint32
}
/** An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
*
*See the [Rust documentation for `Time`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html) for more information.
*/
class Time internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class TimeCleaner(val handle: Pointer, val lib: TimeLib) : Runnable {
        override fun run() {
            lib.icu4x_Time_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TimeLib> = TimeLib::class.java
        internal val lib: TimeLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [Time] given field values
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#method.try_new) for more information.
        */
        fun create(hour: UByte, minute: UByte, second: UByte, subsecond: UInt): Result<Time> {
            
            val returnVal = lib.icu4x_Time_create_mv1(FFIUint8(hour), FFIUint8(minute), FFIUint8(second), FFIUint32(subsecond));
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Time(handle, selfEdges)
                CLEANER.register(returnOpaque, Time.TimeCleaner(handle, Time.lib));
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [Time] from an IXDTF string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#method.try_from_str) for more information.
        */
        fun fromString(v: String): Result<Time> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_Time_from_string_mv1(vSlice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Time(handle, selfEdges)
                CLEANER.register(returnOpaque, Time.TimeCleaner(handle, Time.lib));
                if (vMem != null) vMem.close()
                return returnOpaque.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [Time] representing the start of the day (00:00:00.000).
        *
        *See the [Rust documentation for `start_of_day`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#method.start_of_day) for more information.
        */
        fun startOfDay(): Result<Time> {
            
            val returnVal = lib.icu4x_Time_start_of_day_mv1();
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Time(handle, selfEdges)
                CLEANER.register(returnOpaque, Time.TimeCleaner(handle, Time.lib));
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [Time] representing noon (12:00:00.000).
        *
        *See the [Rust documentation for `noon`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#method.noon) for more information.
        */
        fun noon(): Result<Time> {
            
            val returnVal = lib.icu4x_Time_noon_mv1();
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Time(handle, selfEdges)
                CLEANER.register(returnOpaque, Time.TimeCleaner(handle, Time.lib));
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Returns the hour in this time
    *
    *See the [Rust documentation for `hour`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#structfield.hour) for more information.
    */
    fun hour(): UByte {
        
        val returnVal = lib.icu4x_Time_hour_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the minute in this time
    *
    *See the [Rust documentation for `minute`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#structfield.minute) for more information.
    */
    fun minute(): UByte {
        
        val returnVal = lib.icu4x_Time_minute_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the second in this time
    *
    *See the [Rust documentation for `second`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#structfield.second) for more information.
    */
    fun second(): UByte {
        
        val returnVal = lib.icu4x_Time_second_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the subsecond in this time as nanoseconds
    *
    *See the [Rust documentation for `subsecond`](https://docs.rs/icu/2.1.1/icu/time/struct.Time.html#structfield.subsecond) for more information.
    */
    fun subsecond(): UInt {
        
        val returnVal = lib.icu4x_Time_subsecond_mv1(handle);
        return (returnVal.toUInt())
    }

}
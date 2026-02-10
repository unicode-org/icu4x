package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneLib: Library {
    fun icu4x_TimeZone_destroy_mv1(handle: Pointer)
    fun icu4x_TimeZone_unknown_mv1(): Pointer
    fun icu4x_TimeZone_is_unknown_mv1(handle: Pointer): Byte
    fun icu4x_TimeZone_create_from_bcp47_mv1(id: Slice): Pointer
    fun icu4x_TimeZone_with_offset_mv1(handle: Pointer, offset: Pointer): Pointer
    fun icu4x_TimeZone_without_offset_mv1(handle: Pointer): Pointer
}
/** See the [Rust documentation for `TimeZone`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZone.html) for more information.
*/
class TimeZone internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class TimeZoneCleaner(val handle: Pointer, val lib: TimeZoneLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeZone_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TimeZoneLib> = TimeZoneLib::class.java
        internal val lib: TimeZoneLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** The unknown time zone.
        *
        *See the [Rust documentation for `unknown`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.unknown) for more information.
        */
        fun unknown(): TimeZone {
            
            val returnVal = lib.icu4x_TimeZone_unknown_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = TimeZone(handle, selfEdges)
            CLEANER.register(returnOpaque, TimeZone.TimeZoneCleaner(handle, TimeZone.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Creates a time zone from a BCP-47 string.
        *
        *Returns the unknown time zone if the string is not a valid BCP-47 subtag.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZone.html)
        */
        fun createFromBcp47(id: String): TimeZone {
            val idSliceMemory = PrimitiveArrayTools.borrowUtf8(id)
            
            val returnVal = lib.icu4x_TimeZone_create_from_bcp47_mv1(idSliceMemory.slice);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = TimeZone(handle, selfEdges)
            CLEANER.register(returnOpaque, TimeZone.TimeZoneCleaner(handle, TimeZone.lib));
            idSliceMemory?.close()
            return returnOpaque
        }
    }
    
    /** Whether the time zone is the unknown zone.
    *
    *See the [Rust documentation for `is_unknown`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZone.html#method.is_unknown) for more information.
    */
    fun isUnknown(): Boolean {
        
        val returnVal = lib.icu4x_TimeZone_is_unknown_mv1(handle);
        return (returnVal > 0)
    }
    
    /** See the [Rust documentation for `with_offset`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZone.html#method.with_offset) for more information.
    */
    fun withOffset(offset: UtcOffset): TimeZoneInfo {
        
        val returnVal = lib.icu4x_TimeZone_with_offset_mv1(handle, offset.handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZoneInfo(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZoneInfo.TimeZoneInfoCleaner(handle, TimeZoneInfo.lib));
        return returnOpaque
    }
    
    /** See the [Rust documentation for `without_offset`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZone.html#method.without_offset) for more information.
    */
    fun withoutOffset(): TimeZoneInfo {
        
        val returnVal = lib.icu4x_TimeZone_without_offset_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZoneInfo(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZoneInfo.TimeZoneInfoCleaner(handle, TimeZoneInfo.lib));
        return returnOpaque
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CalendarLib: Library {
    fun icu4x_Calendar_destroy_mv1(handle: Pointer)
    fun icu4x_Calendar_create_mv1(kind: Int): Pointer
    fun icu4x_Calendar_create_with_provider_mv1(provider: Pointer, kind: Int): ResultPointerInt
    fun icu4x_Calendar_kind_mv1(handle: Pointer): Int
}
/** See the [Rust documentation for `AnyCalendar`](https://docs.rs/icu/2.1.1/icu/calendar/enum.AnyCalendar.html) for more information.
*/
class Calendar internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CalendarCleaner(val handle: Pointer, val lib: CalendarLib) : Runnable {
        override fun run() {
            lib.icu4x_Calendar_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CalendarLib> = CalendarLib::class.java
        internal val lib: CalendarLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [Calendar] for the specified kind, using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/calendar/enum.AnyCalendar.html#method.new) for more information.
        */
        fun create(kind: CalendarKind): Calendar {
            
            val returnVal = lib.icu4x_Calendar_create_mv1(kind.toNative());
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Calendar(handle, selfEdges)
            CLEANER.register(returnOpaque, Calendar.CalendarCleaner(handle, Calendar.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Creates a new [Calendar] for the specified kind, using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/calendar/enum.AnyCalendar.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider, kind: CalendarKind): Result<Calendar> {
            
            val returnVal = lib.icu4x_Calendar_create_with_provider_mv1(provider.handle, kind.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Calendar(handle, selfEdges)
                CLEANER.register(returnOpaque, Calendar.CalendarCleaner(handle, Calendar.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Returns the kind of this calendar
    *
    *See the [Rust documentation for `kind`](https://docs.rs/icu/2.1.1/icu/calendar/enum.AnyCalendar.html#method.kind) for more information.
    */
    fun kind(): CalendarKind {
        
        val returnVal = lib.icu4x_Calendar_kind_mv1(handle);
        return (CalendarKind.fromNative(returnVal))
    }

}
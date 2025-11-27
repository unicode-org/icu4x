package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneIteratorLib: Library {
    fun icu4x_TimeZoneIterator_destroy_mv1(handle: Pointer)
    fun icu4x_TimeZoneIterator_next_mv1(handle: Pointer): Pointer?
}
typealias TimeZoneIteratorIteratorItem = TimeZone?
/** See the [Rust documentation for `TimeZoneIter`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneIter.html) for more information.
*/
class TimeZoneIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<TimeZone?> {

    internal class TimeZoneIteratorCleaner(val handle: Pointer, val lib: TimeZoneIteratorLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeZoneIterator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TimeZoneIteratorLib> = TimeZoneIteratorLib::class.java
        internal val lib: TimeZoneIteratorLib = Native.load("icu4x", libClass)
    }
    
    /** See the [Rust documentation for `next`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneIter.html#method.next) for more information.
    */
    internal fun nextInternal(): TimeZone? {
        
        val returnVal = lib.icu4x_TimeZoneIterator_next_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = TimeZone(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZone.TimeZoneCleaner(handle, TimeZone.lib));
        return returnOpaque
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): TimeZone?{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}
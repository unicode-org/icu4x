package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneAndCanonicalIteratorLib: Library {
    fun icu4x_TimeZoneAndCanonicalIterator_destroy_mv1(handle: Pointer)
    fun icu4x_TimeZoneAndCanonicalIterator_next_mv1(handle: Pointer): OptionTimeZoneAndCanonicalNative
}
typealias TimeZoneAndCanonicalIteratorIteratorItem = TimeZoneAndCanonical
/** See the [Rust documentation for `TimeZoneAndCanonicalIter`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonicalIter.html) for more information.
*/
class TimeZoneAndCanonicalIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<TimeZoneAndCanonical> {

    internal class TimeZoneAndCanonicalIteratorCleaner(val handle: Pointer, val lib: TimeZoneAndCanonicalIteratorLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeZoneAndCanonicalIterator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TimeZoneAndCanonicalIteratorLib> = TimeZoneAndCanonicalIteratorLib::class.java
        internal val lib: TimeZoneAndCanonicalIteratorLib = Native.load("icu4x", libClass)
    }
    
    /** See the [Rust documentation for `next`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonicalIter.html#method.next) for more information.
    */
    internal fun nextInternal(): TimeZoneAndCanonical? {
        
        val returnVal = lib.icu4x_TimeZoneAndCanonicalIterator_next_mv1(handle);
        
        val intermediateOption = returnVal.option() ?: return null

        val aEdges: List<Any?> = listOf(this)
        val returnStruct = TimeZoneAndCanonical(intermediateOption, aEdges)
        return returnStruct
                                
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): TimeZoneAndCanonical{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}
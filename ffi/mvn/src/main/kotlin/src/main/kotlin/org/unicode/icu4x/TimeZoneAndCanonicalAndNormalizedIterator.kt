package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneAndCanonicalAndNormalizedIteratorLib: Library {
    fun icu4x_TimeZoneAndCanonicalAndNormalizedIterator_destroy_mv1(handle: Pointer)
    fun icu4x_TimeZoneAndCanonicalAndNormalizedIterator_next_mv1(handle: Pointer): OptionTimeZoneAndCanonicalAndNormalizedNative
}
typealias TimeZoneAndCanonicalAndNormalizedIteratorIteratorItem = TimeZoneAndCanonicalAndNormalized
/** See the [Rust documentation for `TimeZoneAndCanonicalAndNormalizedIter`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonicalAndNormalizedIter.html) for more information.
*/
class TimeZoneAndCanonicalAndNormalizedIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<TimeZoneAndCanonicalAndNormalized> {

    internal class TimeZoneAndCanonicalAndNormalizedIteratorCleaner(val handle: Pointer, val lib: TimeZoneAndCanonicalAndNormalizedIteratorLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeZoneAndCanonicalAndNormalizedIterator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TimeZoneAndCanonicalAndNormalizedIteratorLib> = TimeZoneAndCanonicalAndNormalizedIteratorLib::class.java
        internal val lib: TimeZoneAndCanonicalAndNormalizedIteratorLib = Native.load("icu4x", libClass)
    }
    
    /** See the [Rust documentation for `next`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonicalAndNormalizedIter.html#method.next) for more information.
    */
    internal fun nextInternal(): TimeZoneAndCanonicalAndNormalized? {
        
        val returnVal = lib.icu4x_TimeZoneAndCanonicalAndNormalizedIterator_next_mv1(handle);
        
        val intermediateOption = returnVal.option() ?: return null

        val aEdges: List<Any?> = listOf(this)
        val returnStruct = TimeZoneAndCanonicalAndNormalized.fromNative(intermediateOption, aEdges)
        return returnStruct
                                
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): TimeZoneAndCanonicalAndNormalized{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}
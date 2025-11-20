package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface WeekdaySetIteratorLib: Library {
    fun icu4x_WeekdaySetIterator_destroy_mv1(handle: Pointer)
    fun icu4x_WeekdaySetIterator_next_mv1(handle: Pointer): OptionInt
}
typealias WeekdaySetIteratorIteratorItem = Weekday
/** Documents which days of the week are considered to be a part of the weekend
*
*See the [Rust documentation for `WeekdaySetIterator`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekdaySetIterator.html) for more information.
*/
class WeekdaySetIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
): Iterator<Weekday> {

    internal class WeekdaySetIteratorCleaner(val handle: Pointer, val lib: WeekdaySetIteratorLib) : Runnable {
        override fun run() {
            lib.icu4x_WeekdaySetIterator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<WeekdaySetIteratorLib> = WeekdaySetIteratorLib::class.java
        internal val lib: WeekdaySetIteratorLib = Native.load("icu4x", libClass)
    }
    
    /** See the [Rust documentation for `next`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekdaySetIterator.html#method.next) for more information.
    */
    internal fun nextInternal(): Weekday? {
        
        val returnVal = lib.icu4x_WeekdaySetIterator_next_mv1(handle);
        
        val intermediateOption = returnVal.option() ?: return null
        return Weekday.fromNative(intermediateOption)
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): Weekday{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}
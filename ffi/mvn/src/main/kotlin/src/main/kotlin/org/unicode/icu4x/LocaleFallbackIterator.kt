package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleFallbackIteratorLib: Library {
    fun icu4x_LocaleFallbackIterator_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleFallbackIterator_next_mv1(handle: Pointer): Pointer?
}
typealias LocaleFallbackIteratorIteratorItem = Locale?
/** An iterator over the locale under fallback.
*
*See the [Rust documentation for `LocaleFallbackIterator`](https://docs.rs/icu/2.1.1/icu/locale/fallback/struct.LocaleFallbackIterator.html) for more information.
*/
class LocaleFallbackIterator internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
): Iterator<Locale?> {

    internal class LocaleFallbackIteratorCleaner(val handle: Pointer, val lib: LocaleFallbackIteratorLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleFallbackIterator_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleFallbackIteratorLib> = LocaleFallbackIteratorLib::class.java
        internal val lib: LocaleFallbackIteratorLib = Native.load("icu4x", libClass)
    }
    
    internal fun nextInternal(): Locale? {
        
        val returnVal = lib.icu4x_LocaleFallbackIterator_next_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = Locale(handle, selfEdges)
        CLEANER.register(returnOpaque, Locale.LocaleCleaner(handle, Locale.lib));
        return returnOpaque
    }

    var iterVal = nextInternal()

    override fun hasNext(): Boolean {
       return iterVal != null
    }

    override fun next(): Locale?{
        val returnVal = iterVal
        if (returnVal == null) {
            throw NoSuchElementException()
        } else {
            iterVal = nextInternal()
            return returnVal
        }
    }

}
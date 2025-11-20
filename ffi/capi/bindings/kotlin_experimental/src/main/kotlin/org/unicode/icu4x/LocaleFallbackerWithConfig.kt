package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleFallbackerWithConfigLib: Library {
    fun icu4x_LocaleFallbackerWithConfig_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1(handle: Pointer, locale: Pointer): Pointer
}
/** An object that runs the ICU4X locale fallback algorithm with specific configurations.
*
*See the [Rust documentation for `LocaleFallbacker`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html) for more information.
*
*See the [Rust documentation for `LocaleFallbackerWithConfig`](https://docs.rs/icu/2.1.1/icu/locale/fallback/struct.LocaleFallbackerWithConfig.html) for more information.
*/
class LocaleFallbackerWithConfig internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class LocaleFallbackerWithConfigCleaner(val handle: Pointer, val lib: LocaleFallbackerWithConfigLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleFallbackerWithConfig_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleFallbackerWithConfigLib> = LocaleFallbackerWithConfigLib::class.java
        internal val lib: LocaleFallbackerWithConfigLib = Native.load("icu4x", libClass)
    }
    
    /** Creates an iterator from a locale with each step of fallback.
    *
    *See the [Rust documentation for `fallback_for`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html#method.fallback_for) for more information.
    */
    fun fallbackForLocale(locale: Locale): LocaleFallbackIterator {
        
        val returnVal = lib.icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1(handle, locale.handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = LocaleFallbackIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, LocaleFallbackIterator.LocaleFallbackIteratorCleaner(handle, LocaleFallbackIterator.lib));
        return returnOpaque
    }

}
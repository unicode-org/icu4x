package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PartsLib: Library {
    fun icu4x_Parts_destroy_mv1(handle: Pointer)
    fun icu4x_Parts_len_mv1(handle: Pointer): FFISizet
    fun icu4x_Parts_is_empty_mv1(handle: Pointer): Byte
    fun icu4x_Parts_get_mv1(handle: Pointer, index: FFISizet): OptionPartSpanNative
}
/**
 * An immutable collection of [PartSpan]s drained from a [PartsSink].
 *
 * Because [Parts] is immutable, it can safely lend out a contiguous `&[PartSpan]`
 * slice in C/C++ via [Parts::as_slice].
 *
 * See the [Rust documentation for `Part`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class Parts internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class PartsCleaner(val handle: Pointer, val lib: PartsLib) : Runnable {
        override fun run() {
            lib.icu4x_Parts_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, Parts.PartsCleaner(handle, Parts.lib));
    }

    companion object {
        internal val libClass: Class<PartsLib> = PartsLib::class.java
        internal val lib: PartsLib = Native.load("icu4x", libClass)
    }
    
    /**
     * Returns the number of recorded [PartSpan]s.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun len(): ULong {
        
        val returnVal = lib.icu4x_Parts_len_mv1(handle);
        return (returnVal.toULong())
    }
    
    /**
     * Returns whether there are no recorded [PartSpan]s.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun isEmpty(): Boolean {
        
        val returnVal = lib.icu4x_Parts_is_empty_mv1(handle);
        return (returnVal > 0)
    }
    
    /**
     * Returns the [PartSpan] at `index`, or `None` if out of bounds.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    internal fun getInternal(index: ULong): PartSpan? {
        
        val returnVal = lib.icu4x_Parts_get_mv1(handle, FFISizet(index));
        
        val intermediateOption = returnVal.option() ?: return null
        val returnStruct = PartSpan.fromNative(intermediateOption)
        return returnStruct
                                
    }

    operator fun get(index: ULong): PartSpan {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}
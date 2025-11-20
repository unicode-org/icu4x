package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ReorderedIndexMapLib: Library {
    fun icu4x_ReorderedIndexMap_destroy_mv1(handle: Pointer)
    fun icu4x_ReorderedIndexMap_as_slice_mv1(handle: Pointer): Slice
    fun icu4x_ReorderedIndexMap_len_mv1(handle: Pointer): FFISizet
    fun icu4x_ReorderedIndexMap_is_empty_mv1(handle: Pointer): Byte
    fun icu4x_ReorderedIndexMap_get_mv1(handle: Pointer, index: FFISizet): FFISizet
}
/** Thin wrapper around a vector that maps visual indices to source indices
*
*`map[visualIndex] = sourceIndex`
*
*Produced by `reorder_visual()` on [Bidi].
*/
class ReorderedIndexMap internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class ReorderedIndexMapCleaner(val handle: Pointer, val lib: ReorderedIndexMapLib) : Runnable {
        override fun run() {
            lib.icu4x_ReorderedIndexMap_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<ReorderedIndexMapLib> = ReorderedIndexMapLib::class.java
        internal val lib: ReorderedIndexMapLib = Native.load("icu4x", libClass)
    }
    
    /** Get this as a slice/array of indices
    */
    fun asSlice(): ULongArray {
        
        val returnVal = lib.icu4x_ReorderedIndexMap_as_slice_mv1(handle);
            return PrimitiveArrayTools.getULongArray(returnVal)
    }
    
    /** The length of this map
    */
    fun len(): ULong {
        
        val returnVal = lib.icu4x_ReorderedIndexMap_len_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** Whether this map is empty
    */
    fun isEmpty(): Boolean {
        
        val returnVal = lib.icu4x_ReorderedIndexMap_is_empty_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Get element at `index`. Returns 0 when out of bounds
    *(note that 0 is also a valid in-bounds value, please use `len()`
    *to avoid out-of-bounds)
    */
    internal fun getInternal(index: ULong): ULong {
        
        val returnVal = lib.icu4x_ReorderedIndexMap_get_mv1(handle, FFISizet(index));
        return (returnVal.toULong())
    }

    operator fun get(index: ULong): ULong {
        val returnVal = getInternal(index)
        if (returnVal == null) {
            throw IndexOutOfBoundsException("Index $index is out of bounds.")
        } else {
            return returnVal
        }
    }

}
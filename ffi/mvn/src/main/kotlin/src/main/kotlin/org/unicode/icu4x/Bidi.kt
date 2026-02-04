package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BidiLib: Library {
    fun icu4x_Bidi_destroy_mv1(handle: Pointer)
    fun icu4x_Bidi_create_mv1(): Pointer
    fun icu4x_Bidi_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_Bidi_reorder_visual_mv1(handle: Pointer, levels: Slice): Pointer
    fun icu4x_Bidi_level_is_rtl_mv1(level: FFIUint8): Byte
    fun icu4x_Bidi_level_is_ltr_mv1(level: FFIUint8): Byte
    fun icu4x_Bidi_level_rtl_mv1(): FFIUint8
    fun icu4x_Bidi_level_ltr_mv1(): FFIUint8
}
/** An ICU4X Bidi object, containing loaded bidi data
*
*See the [Rust documentation for `BidiClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiClass.html) for more information.
*/
class Bidi internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class BidiCleaner(val handle: Pointer, val lib: BidiLib) : Runnable {
        override fun run() {
            lib.icu4x_Bidi_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<BidiLib> = BidiLib::class.java
        internal val lib: BidiLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [Bidi] from locale data using compiled data.
        */
        fun create(): Bidi {
            
            val returnVal = lib.icu4x_Bidi_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Bidi(handle, selfEdges)
            CLEANER.register(returnOpaque, Bidi.BidiCleaner(handle, Bidi.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Creates a new [Bidi] from locale data, and a particular data source.
        */
        fun createWithProvider(provider: DataProvider): Result<Bidi> {
            
            val returnVal = lib.icu4x_Bidi_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Bidi(handle, selfEdges)
                CLEANER.register(returnOpaque, Bidi.BidiCleaner(handle, Bidi.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Check if a Level returned by `level_at` is an RTL level.
        *
        *Invalid levels (numbers greater than 125) will be assumed LTR
        *
        *See the [Rust documentation for `is_rtl`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/level/struct.Level.html#method.is_rtl) for more information.
        */
        fun levelIsRtl(level: UByte): Boolean {
            
            val returnVal = lib.icu4x_Bidi_level_is_rtl_mv1(FFIUint8(level));
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Check if a Level returned by `level_at` is an LTR level.
        *
        *Invalid levels (numbers greater than 125) will be assumed LTR
        *
        *See the [Rust documentation for `is_ltr`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/level/struct.Level.html#method.is_ltr) for more information.
        */
        fun levelIsLtr(level: UByte): Boolean {
            
            val returnVal = lib.icu4x_Bidi_level_is_ltr_mv1(FFIUint8(level));
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Get a basic RTL Level value
        *
        *See the [Rust documentation for `rtl`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/level/struct.Level.html#method.rtl) for more information.
        */
        fun levelRtl(): UByte {
            
            val returnVal = lib.icu4x_Bidi_level_rtl_mv1();
            return (returnVal.toUByte())
        }
        @JvmStatic
        
        /** Get a simple LTR Level value
        *
        *See the [Rust documentation for `ltr`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/level/struct.Level.html#method.ltr) for more information.
        */
        fun levelLtr(): UByte {
            
            val returnVal = lib.icu4x_Bidi_level_ltr_mv1();
            return (returnVal.toUByte())
        }
    }
    
    /** Utility function for producing reorderings given a list of levels
    *
    *Produces a map saying which visual index maps to which source index.
    *
    *The levels array must not have values greater than 126 (this is the
    *Bidi maximum explicit depth plus one).
    *Failure to follow this invariant may lead to incorrect results,
    *but is still safe.
    *
    *See the [Rust documentation for `reorder_visual`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/struct.BidiInfo.html#method.reorder_visual) for more information.
    */
    fun reorderVisual(levels: UByteArray): ReorderedIndexMap {
        val levelsSliceMemory = PrimitiveArrayTools.borrow(levels)
        
        val returnVal = lib.icu4x_Bidi_reorder_visual_mv1(handle, levelsSliceMemory.slice);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = ReorderedIndexMap(handle, selfEdges)
        CLEANER.register(returnOpaque, ReorderedIndexMap.ReorderedIndexMapCleaner(handle, ReorderedIndexMap.lib));
        levelsSliceMemory?.close()
        return returnOpaque
    }

}
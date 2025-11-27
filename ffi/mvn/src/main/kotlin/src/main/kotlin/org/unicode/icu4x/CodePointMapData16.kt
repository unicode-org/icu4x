package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CodePointMapData16Lib: Library {
    fun icu4x_CodePointMapData16_destroy_mv1(handle: Pointer)
    fun icu4x_CodePointMapData16_get_mv1(handle: Pointer, cp: Int): FFIUint16
    fun icu4x_CodePointMapData16_iter_ranges_for_value_mv1(handle: Pointer, value: FFIUint16): Pointer
    fun icu4x_CodePointMapData16_iter_ranges_for_value_complemented_mv1(handle: Pointer, value: FFIUint16): Pointer
    fun icu4x_CodePointMapData16_get_set_for_value_mv1(handle: Pointer, value: FFIUint16): Pointer
    fun icu4x_CodePointMapData16_create_script_mv1(): Pointer
    fun icu4x_CodePointMapData16_create_script_with_provider_mv1(provider: Pointer): ResultPointerInt
}
/** An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
*
*For properties whose values fit into 16 bits.
*
*See the [Rust documentation for `properties`](https://docs.rs/icu/2.1.1/icu/properties/index.html) for more information.
*
*See the [Rust documentation for `CodePointMapData`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapData.html) for more information.
*
*See the [Rust documentation for `CodePointMapDataBorrowed`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html) for more information.
*/
class CodePointMapData16 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CodePointMapData16Cleaner(val handle: Pointer, val lib: CodePointMapData16Lib) : Runnable {
        override fun run() {
            lib.icu4x_CodePointMapData16_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CodePointMapData16Lib> = CodePointMapData16Lib::class.java
        internal val lib: CodePointMapData16Lib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a map for the `Script` property, using compiled data.
        *
        *See the [Rust documentation for `Script`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html) for more information.
        */
        fun createScript(): CodePointMapData16 {
            
            val returnVal = lib.icu4x_CodePointMapData16_create_script_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointMapData16(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointMapData16.CodePointMapData16Cleaner(handle, CodePointMapData16.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `Script` property, using a particular data source.
        *
        *See the [Rust documentation for `Script`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html) for more information.
        */
        fun createScriptWithProvider(provider: DataProvider): Result<CodePointMapData16> {
            
            val returnVal = lib.icu4x_CodePointMapData16_create_script_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CodePointMapData16(handle, selfEdges)
                CLEANER.register(returnOpaque, CodePointMapData16.CodePointMapData16Cleaner(handle, CodePointMapData16.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Gets the value for a code point.
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.get) for more information.
    */
    fun get(cp: Int): UShort {
        
        val returnVal = lib.icu4x_CodePointMapData16_get_mv1(handle, cp);
        return (returnVal.toUShort())
    }
    
    /** Produces an iterator over ranges of code points that map to `value`
    *
    *See the [Rust documentation for `iter_ranges_for_value`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value) for more information.
    */
    fun iterRangesForValue(value: UShort): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointMapData16_iter_ranges_for_value_mv1(handle, FFIUint16(value));
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }
    
    /** Produces an iterator over ranges of code points that do not map to `value`
    *
    *See the [Rust documentation for `iter_ranges_for_value_complemented`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented) for more information.
    */
    fun iterRangesForValueComplemented(value: UShort): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_CodePointMapData16_iter_ranges_for_value_complemented_mv1(handle, FFIUint16(value));
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }
    
    /** Gets a [CodePointSetData] representing all entries in this map that map to the given value
    *
    *See the [Rust documentation for `get_set_for_value`](https://docs.rs/icu/2.1.1/icu/properties/struct.CodePointMapDataBorrowed.html#method.get_set_for_value) for more information.
    */
    fun getSetForValue(value: UShort): CodePointSetData {
        
        val returnVal = lib.icu4x_CodePointMapData16_get_set_for_value_mv1(handle, FFIUint16(value));
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = CodePointSetData(handle, selfEdges)
        CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
        return returnOpaque
    }

}
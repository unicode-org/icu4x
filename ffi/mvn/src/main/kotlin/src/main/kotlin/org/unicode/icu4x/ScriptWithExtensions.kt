package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ScriptWithExtensionsLib: Library {
    fun icu4x_ScriptWithExtensions_destroy_mv1(handle: Pointer)
    fun icu4x_ScriptWithExtensions_create_mv1(): Pointer
    fun icu4x_ScriptWithExtensions_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_ScriptWithExtensions_get_script_val_mv1(handle: Pointer, ch: Int): FFIUint16
    fun icu4x_ScriptWithExtensions_has_script_mv1(handle: Pointer, ch: Int, script: FFIUint16): Byte
    fun icu4x_ScriptWithExtensions_as_borrowed_mv1(handle: Pointer): Pointer
    fun icu4x_ScriptWithExtensions_iter_ranges_for_script_mv1(handle: Pointer, script: FFIUint16): Pointer
}
/** An ICU4X `ScriptWithExtensions` map object, capable of holding a map of codepoints to scriptextensions values
*
*See the [Rust documentation for `ScriptWithExtensions`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensions.html) for more information.
*/
class ScriptWithExtensions internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class ScriptWithExtensionsCleaner(val handle: Pointer, val lib: ScriptWithExtensionsLib) : Runnable {
        override fun run() {
            lib.icu4x_ScriptWithExtensions_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<ScriptWithExtensionsLib> = ScriptWithExtensionsLib::class.java
        internal val lib: ScriptWithExtensionsLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a map for the `Script`/`Script_Extensions` properties, using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensions.html#method.new) for more information.
        */
        fun create(): ScriptWithExtensions {
            
            val returnVal = lib.icu4x_ScriptWithExtensions_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ScriptWithExtensions(handle, selfEdges)
            CLEANER.register(returnOpaque, ScriptWithExtensions.ScriptWithExtensionsCleaner(handle, ScriptWithExtensions.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `Script`/`Script_Extensions` properties, using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensions.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<ScriptWithExtensions> {
            
            val returnVal = lib.icu4x_ScriptWithExtensions_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ScriptWithExtensions(handle, selfEdges)
                CLEANER.register(returnOpaque, ScriptWithExtensions.ScriptWithExtensionsCleaner(handle, ScriptWithExtensions.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Get the Script property value for a code point
    *
    *See the [Rust documentation for `get_script_val`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val) for more information.
    */
    fun getScriptVal(ch: Int): UShort {
        
        val returnVal = lib.icu4x_ScriptWithExtensions_get_script_val_mv1(handle, ch);
        return (returnVal.toUShort())
    }
    
    /** Check if the `Script_Extensions` property of the given code point covers the given script
    *
    *See the [Rust documentation for `has_script`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script) for more information.
    */
    fun hasScript(ch: Int, script: UShort): Boolean {
        
        val returnVal = lib.icu4x_ScriptWithExtensions_has_script_mv1(handle, ch, FFIUint16(script));
        return (returnVal > 0)
    }
    
    /** Borrow this object for a slightly faster variant with more operations
    *
    *See the [Rust documentation for `as_borrowed`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensions.html#method.as_borrowed) for more information.
    */
    fun asBorrowed(): ScriptWithExtensionsBorrowed {
        
        val returnVal = lib.icu4x_ScriptWithExtensions_as_borrowed_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = ScriptWithExtensionsBorrowed(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, ScriptWithExtensionsBorrowed.ScriptWithExtensionsBorrowedCleaner(handle, ScriptWithExtensionsBorrowed.lib));
        return returnOpaque
    }
    
    /** Get a list of ranges of code points that contain this script in their `Script_Extensions` values
    *
    *See the [Rust documentation for `get_script_extensions_ranges`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_ranges) for more information.
    */
    fun iterRangesForScript(script: UShort): CodePointRangeIterator {
        
        val returnVal = lib.icu4x_ScriptWithExtensions_iter_ranges_for_script_mv1(handle, FFIUint16(script));
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = CodePointRangeIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, CodePointRangeIterator.CodePointRangeIteratorCleaner(handle, CodePointRangeIterator.lib));
        return returnOpaque
    }

}
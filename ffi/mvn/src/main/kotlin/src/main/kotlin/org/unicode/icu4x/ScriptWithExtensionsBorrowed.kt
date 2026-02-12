package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ScriptWithExtensionsBorrowedLib: Library {
    fun icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(handle: Pointer)
    fun icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv1(handle: Pointer, ch: Int): FFIUint16
    fun icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(handle: Pointer, ch: Int): Pointer
    fun icu4x_ScriptWithExtensionsBorrowed_has_script_mv1(handle: Pointer, ch: Int, script: FFIUint16): Byte
    fun icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv1(handle: Pointer, script: FFIUint16): Pointer
}
/** A slightly faster `ScriptWithExtensions` object
*
*See the [Rust documentation for `ScriptWithExtensionsBorrowed`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html) for more information.
*/
class ScriptWithExtensionsBorrowed internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class ScriptWithExtensionsBorrowedCleaner(val handle: Pointer, val lib: ScriptWithExtensionsBorrowedLib) : Runnable {
        override fun run() {
            lib.icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<ScriptWithExtensionsBorrowedLib> = ScriptWithExtensionsBorrowedLib::class.java
        internal val lib: ScriptWithExtensionsBorrowedLib = Native.load("icu4x", libClass)
    }
    
    /** Get the Script property value for a code point
    *Get the Script property value for a code point
    *
    *See the [Rust documentation for `get_script_val`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val) for more information.
    */
    fun getScriptVal(ch: Int): UShort {
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv1(handle, ch);
        return (returnVal.toUShort())
    }
    
    /** Get the Script property value for a code point
    *
    *See the [Rust documentation for `get_script_extensions_val`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_val) for more information.
    */
    fun getScriptExtensionsVal(ch: Int): ScriptExtensionsSet {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(handle, ch);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = ScriptExtensionsSet(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, ScriptExtensionsSet.ScriptExtensionsSetCleaner(handle, ScriptExtensionsSet.lib));
        return returnOpaque
    }
    
    /** Check if the `Script_Extensions` property of the given code point covers the given script
    *
    *See the [Rust documentation for `has_script`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script) for more information.
    */
    fun hasScript(ch: Int, script: UShort): Boolean {
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_has_script_mv1(handle, ch, FFIUint16(script));
        return (returnVal > 0)
    }
    
    /** Build the `CodePointSetData` corresponding to a codepoints matching a particular script
    *in their `Script_Extensions`
    *
    *See the [Rust documentation for `get_script_extensions_set`](https://docs.rs/icu/2.1.1/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_set) for more information.
    */
    fun getScriptExtensionsSet(script: UShort): CodePointSetData {
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv1(handle, FFIUint16(script));
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = CodePointSetData(handle, selfEdges)
        CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
        return returnOpaque
    }

}
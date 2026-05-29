package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ScriptWithExtensionsBorrowedLib: Library {
    fun icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(handle: Pointer)
    fun icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv2(handle: Pointer, ch: Int): Int
    fun icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(handle: Pointer, ch: Int): Pointer
    fun icu4x_ScriptWithExtensionsBorrowed_has_script_mv2(handle: Pointer, ch: Int, script: Int): Byte
    fun icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv2(handle: Pointer, script: Int): Pointer
}
/** A slightly faster `ScriptWithExtensions` object
*
*See the [Rust documentation for `ScriptWithExtensionsBorrowed`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html) for more information.
*/
class ScriptWithExtensionsBorrowed internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class ScriptWithExtensionsBorrowedCleaner(val handle: Pointer, val lib: ScriptWithExtensionsBorrowedLib) : Runnable {
        override fun run() {
            lib.icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ScriptWithExtensionsBorrowed.ScriptWithExtensionsBorrowedCleaner(handle, ScriptWithExtensionsBorrowed.lib));
    }

    companion object {
        internal val libClass: Class<ScriptWithExtensionsBorrowedLib> = ScriptWithExtensionsBorrowedLib::class.java
        internal val lib: ScriptWithExtensionsBorrowedLib = Native.load("icu4x", libClass)
    }
    
    /** Get the Script property value for a code point
    *
    *See the [Rust documentation for `get_script_val`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val) for more information.
    */
    fun getScriptVal(ch: Int): Script {
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv2(handle, ch);
        return (Script.fromNative(returnVal))
    }
    
    /** Get the Script property value for a code point
    *
    *See the [Rust documentation for `get_script_extensions_val`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_val) for more information.
    */
    fun getScriptExtensionsVal(ch: Int): ScriptExtensionsSet {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(handle, ch);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = ScriptExtensionsSet(handle, selfEdges, aEdges, true)
        return returnOpaque
    }
    
    /** Check if the `Script_Extensions` property of the given code point covers the given script
    *
    *See the [Rust documentation for `has_script`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script) for more information.
    */
    fun hasScript(ch: Int, script: Script): Boolean {
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_has_script_mv2(handle, ch, script.toNative());
        return (returnVal > 0)
    }
    
    /** Build the `CodePointSetData` corresponding to a codepoints matching a particular script
    *in their `Script_Extensions`
    *
    *See the [Rust documentation for `get_script_extensions_set`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_set) for more information.
    */
    fun getScriptExtensionsSet(script: Script): CodePointSetData {
        
        val returnVal = lib.icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv2(handle, script.toNative());
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = CodePointSetData(handle, selfEdges, true)
        return returnOpaque
    }

}
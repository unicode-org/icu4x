package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ScriptExtensionsSetLib: Library {
    fun icu4x_ScriptExtensionsSet_destroy_mv1(handle: Pointer)
    fun icu4x_ScriptExtensionsSet_contains_mv2(handle: Pointer, script: Int): Byte
    fun icu4x_ScriptExtensionsSet_count_mv1(handle: Pointer): FFISizet
    fun icu4x_ScriptExtensionsSet_script_at_mv2(handle: Pointer, index: FFISizet): OptionInt
}
/** An object that represents the `Script_Extensions` property for a single character
*
*See the [Rust documentation for `ScriptExtensionsSet`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptExtensionsSet.html) for more information.
*/
class ScriptExtensionsSet internal constructor (
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

    private class ScriptExtensionsSetCleaner(val handle: Pointer, val lib: ScriptExtensionsSetLib) : Runnable {
        override fun run() {
            lib.icu4x_ScriptExtensionsSet_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ScriptExtensionsSet.ScriptExtensionsSetCleaner(handle, ScriptExtensionsSet.lib));
    }

    companion object {
        internal val libClass: Class<ScriptExtensionsSetLib> = ScriptExtensionsSetLib::class.java
        internal val lib: ScriptExtensionsSetLib = Native.load("icu4x", libClass)
    }
    
    /** Check if the `Script_Extensions` property of the given code point covers the given script
    *
    *See the [Rust documentation for `contains`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptExtensionsSet.html#method.contains) for more information.
    */
    fun contains(script: Script): Boolean {
        
        val returnVal = lib.icu4x_ScriptExtensionsSet_contains_mv2(handle, script.toNative());
        return (returnVal > 0)
    }
    
    /** Get the number of scripts contained in here
    *
    *See the [Rust documentation for `iter`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter) for more information.
    */
    fun count(): ULong {
        
        val returnVal = lib.icu4x_ScriptExtensionsSet_count_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** Get script at index
    *
    *See the [Rust documentation for `iter`](https://docs.rs/icu/2.2.0/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter) for more information.
    */
    fun scriptAt(index: ULong): Script? {
        
        val returnVal = lib.icu4x_ScriptExtensionsSet_script_at_mv2(handle, FFISizet(index));
        
        val intermediateOption = returnVal.option() ?: return null
        return Script.fromNative(intermediateOption)
    }

}
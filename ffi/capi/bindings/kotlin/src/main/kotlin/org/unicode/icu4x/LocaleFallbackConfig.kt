package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleFallbackConfigLib: Library {
}

internal class LocaleFallbackConfigNative: Structure(), Structure.ByValue {
    /** Choice of priority mode.
    */
    @JvmField
    internal var priority: Int = LocaleFallbackPriority.default().toNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("priority")
    }
}

/** Collection of configurations for the ICU4X fallback algorithm.
*
*See the [Rust documentation for `LocaleFallbackConfig`](https://docs.rs/icu/2.1.1/icu/locale/fallback/struct.LocaleFallbackConfig.html) for more information.
*/
class LocaleFallbackConfig internal constructor (
    internal val nativeStruct: LocaleFallbackConfigNative) {
    val priority: LocaleFallbackPriority = LocaleFallbackPriority.fromNative(nativeStruct.priority)

    companion object {
        internal val libClass: Class<LocaleFallbackConfigLib> = LocaleFallbackConfigLib::class.java
        internal val lib: LocaleFallbackConfigLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(LocaleFallbackConfigNative::class.java).toLong()
    }

}

package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleFallbackPriorityLib: Library {
}
/** Priority mode for the ICU4X fallback algorithm.
*
*See the [Rust documentation for `LocaleFallbackPriority`](https://docs.rs/icu/2.1.1/icu/locale/fallback/enum.LocaleFallbackPriority.html) for more information.
*/
enum class LocaleFallbackPriority {
    Language,
    Region;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LocaleFallbackPriorityLib> = LocaleFallbackPriorityLib::class.java
        internal val lib: LocaleFallbackPriorityLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LocaleFallbackPriority {
            return LocaleFallbackPriority.entries[native]
        }

        fun default(): LocaleFallbackPriority {
            return Language
        }
    }
}
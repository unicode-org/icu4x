package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DisplayNamesFallbackLib: Library {
}
/** 🚧 This API is experimental and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `Fallback`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/enum.Fallback.html) for more information.
*/
enum class DisplayNamesFallback {
    Code,
    None;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DisplayNamesFallbackLib> = DisplayNamesFallbackLib::class.java
        internal val lib: DisplayNamesFallbackLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DisplayNamesFallback {
            return DisplayNamesFallback.entries[native]
        }

        fun default(): DisplayNamesFallback {
            return Code
        }
    }
}

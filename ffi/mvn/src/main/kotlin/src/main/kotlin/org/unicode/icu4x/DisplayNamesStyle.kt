package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DisplayNamesStyleLib: Library {
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `Style`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/enum.Style.html) for more information.
*/
enum class DisplayNamesStyle {
    Narrow,
    Short,
    Long,
    Menu;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DisplayNamesStyleLib> = DisplayNamesStyleLib::class.java
        internal val lib: DisplayNamesStyleLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DisplayNamesStyle {
            return DisplayNamesStyle.entries[native]
        }

        fun default(): DisplayNamesStyle {
            return Narrow
        }
    }
}

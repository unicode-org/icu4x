package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleDirectionLib: Library {
}
/** See the [Rust documentation for `Direction`](https://docs.rs/icu/2.1.1/icu/locale/enum.Direction.html) for more information.
*/
enum class LocaleDirection {
    LeftToRight,
    RightToLeft,
    Unknown;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LocaleDirectionLib> = LocaleDirectionLib::class.java
        internal val lib: LocaleDirectionLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LocaleDirection {
            return LocaleDirection.entries[native]
        }

        fun default(): LocaleDirection {
            return LeftToRight
        }
    }
}
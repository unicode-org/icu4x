package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationBaseStyleLib: Library {
}
/**
 * See the [Rust documentation for `BaseStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.BaseStyle.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class DurationBaseStyle {
    Narrow,
    Short,
    Long,
    Digital;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DurationBaseStyleLib> = DurationBaseStyleLib::class.java
        internal val lib: DurationBaseStyleLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DurationBaseStyle {
            return DurationBaseStyle.entries[native]
        }

        fun default(): DurationBaseStyle {
            return Narrow
        }
    }
}

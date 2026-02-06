package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LineBreakWordOptionLib: Library {
}
/** See the [Rust documentation for `LineBreakWordOption`](https://docs.rs/icu/2.1.1/icu/segmenter/options/enum.LineBreakWordOption.html) for more information.
*/
enum class LineBreakWordOption {
    Normal,
    BreakAll,
    KeepAll;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LineBreakWordOptionLib> = LineBreakWordOptionLib::class.java
        internal val lib: LineBreakWordOptionLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LineBreakWordOption {
            return LineBreakWordOption.entries[native]
        }

        fun default(): LineBreakWordOption {
            return Normal
        }
    }
}

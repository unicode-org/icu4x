package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LineBreakStrictnessLib: Library {
}
/** See the [Rust documentation for `LineBreakStrictness`](https://docs.rs/icu/2.1.1/icu/segmenter/options/enum.LineBreakStrictness.html) for more information.
*/
enum class LineBreakStrictness {
    Loose,
    Normal,
    Strict,
    Anywhere;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LineBreakStrictnessLib> = LineBreakStrictnessLib::class.java
        internal val lib: LineBreakStrictnessLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LineBreakStrictness {
            return LineBreakStrictness.entries[native]
        }

        fun default(): LineBreakStrictness {
            return Loose
        }
    }
}
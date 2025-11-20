package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BidiDirectionLib: Library {
}
/** See the [Rust documentation for `Direction`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/enum.Direction.html) for more information.
*/
enum class BidiDirection {
    Ltr,
    Rtl,
    Mixed;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<BidiDirectionLib> = BidiDirectionLib::class.java
        internal val lib: BidiDirectionLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): BidiDirection {
            return BidiDirection.entries[native]
        }

        fun default(): BidiDirection {
            return Ltr
        }
    }
}
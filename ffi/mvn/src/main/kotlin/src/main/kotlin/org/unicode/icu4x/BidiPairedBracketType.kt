package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BidiPairedBracketTypeLib: Library {
}
/** See the [Rust documentation for `BidiPairedBracketType`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.BidiPairedBracketType.html) for more information.
*/
enum class BidiPairedBracketType {
    Open,
    Close,
    None;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<BidiPairedBracketTypeLib> = BidiPairedBracketTypeLib::class.java
        internal val lib: BidiPairedBracketTypeLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): BidiPairedBracketType {
            return BidiPairedBracketType.entries[native]
        }

        fun default(): BidiPairedBracketType {
            return Open
        }
    }
}

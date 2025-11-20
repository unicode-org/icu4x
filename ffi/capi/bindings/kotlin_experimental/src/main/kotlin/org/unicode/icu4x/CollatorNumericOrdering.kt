package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorNumericOrderingLib: Library {
}
/** See the [Rust documentation for `CollationNumericOrdering`](https://docs.rs/icu/2.1.1/icu/collator/preferences/enum.CollationNumericOrdering.html) for more information.
*/
enum class CollatorNumericOrdering {
    Off,
    On;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CollatorNumericOrderingLib> = CollatorNumericOrderingLib::class.java
        internal val lib: CollatorNumericOrderingLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CollatorNumericOrdering {
            return CollatorNumericOrdering.entries[native]
        }

        fun default(): CollatorNumericOrdering {
            return Off
        }
    }
}
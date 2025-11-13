package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorCaseFirstLib: Library {
}
/** See the [Rust documentation for `CollationCaseFirst`](https://docs.rs/icu/2.1.1/icu/collator/preferences/enum.CollationCaseFirst.html) for more information.
*/
enum class CollatorCaseFirst {
    Off,
    Lower,
    Upper;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CollatorCaseFirstLib> = CollatorCaseFirstLib::class.java
        internal val lib: CollatorCaseFirstLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CollatorCaseFirst {
            return CollatorCaseFirst.entries[native]
        }

        fun default(): CollatorCaseFirst {
            return Off
        }
    }
}
package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorCaseLevelLib: Library {
}
/** See the [Rust documentation for `CaseLevel`](https://docs.rs/icu/2.1.1/icu/collator/options/enum.CaseLevel.html) for more information.
*/
enum class CollatorCaseLevel {
    Off,
    On;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CollatorCaseLevelLib> = CollatorCaseLevelLib::class.java
        internal val lib: CollatorCaseLevelLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CollatorCaseLevel {
            return CollatorCaseLevel.entries[native]
        }

        fun default(): CollatorCaseLevel {
            return Off
        }
    }
}
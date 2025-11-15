package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorAlternateHandlingLib: Library {
}
/** See the [Rust documentation for `AlternateHandling`](https://docs.rs/icu/2.1.1/icu/collator/options/enum.AlternateHandling.html) for more information.
*/
enum class CollatorAlternateHandling {
    NonIgnorable,
    Shifted;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CollatorAlternateHandlingLib> = CollatorAlternateHandlingLib::class.java
        internal val lib: CollatorAlternateHandlingLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CollatorAlternateHandling {
            return CollatorAlternateHandling.entries[native]
        }

        fun default(): CollatorAlternateHandling {
            return NonIgnorable
        }
    }
}
package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorStrengthLib: Library {
}
/** See the [Rust documentation for `Strength`](https://docs.rs/icu/2.1.1/icu/collator/options/enum.Strength.html) for more information.
*/
enum class CollatorStrength {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Identical;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CollatorStrengthLib> = CollatorStrengthLib::class.java
        internal val lib: CollatorStrengthLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CollatorStrength {
            return CollatorStrength.entries[native]
        }

        fun default(): CollatorStrength {
            return Primary
        }
    }
}

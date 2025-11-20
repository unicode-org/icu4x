package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LeadingAdjustmentLib: Library {
}
/** See the [Rust documentation for `LeadingAdjustment`](https://docs.rs/icu/2.1.1/icu/casemap/options/enum.LeadingAdjustment.html) for more information.
*/
enum class LeadingAdjustment {
    Auto,
    None,
    ToCased;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LeadingAdjustmentLib> = LeadingAdjustmentLib::class.java
        internal val lib: LeadingAdjustmentLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LeadingAdjustment {
            return LeadingAdjustment.entries[native]
        }

        fun default(): LeadingAdjustment {
            return Auto
        }
    }
}
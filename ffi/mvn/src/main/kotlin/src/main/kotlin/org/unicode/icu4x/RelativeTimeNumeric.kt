package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RelativeTimeNumericLib: Library {
}
/**
 * Configures whether to always use numeric formatting even when special formatting is available.
 *
 * See the [Rust documentation for `Numeric`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/options/enum.Numeric.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class RelativeTimeNumeric {
    Always,
    Auto;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<RelativeTimeNumericLib> = RelativeTimeNumericLib::class.java
        internal val lib: RelativeTimeNumericLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): RelativeTimeNumeric {
            return RelativeTimeNumeric.entries[native]
        }

        fun default(): RelativeTimeNumeric {
            return Always
        }
    }
}

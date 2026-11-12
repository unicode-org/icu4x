package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationSignLib: Library {
}
/**
 * See the [Rust documentation for `DurationSign`](https://docs.rs/icu/2.4.0/icu/experimental/duration/enum.DurationSign.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class DurationSign {
    Positive,
    Negative;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DurationSignLib> = DurationSignLib::class.java
        internal val lib: DurationSignLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DurationSign {
            return DurationSign.entries[native]
        }

        fun default(): DurationSign {
            return Positive
        }
    }
}

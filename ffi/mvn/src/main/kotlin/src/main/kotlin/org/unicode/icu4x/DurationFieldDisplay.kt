package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationFieldDisplayLib: Library {
}
/**
 * See the [Rust documentation for `FieldDisplay`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.FieldDisplay.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class DurationFieldDisplay {
    Auto,
    Always;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DurationFieldDisplayLib> = DurationFieldDisplayLib::class.java
        internal val lib: DurationFieldDisplayLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DurationFieldDisplay {
            return DurationFieldDisplay.entries[native]
        }

        fun default(): DurationFieldDisplay {
            return Auto
        }
    }
}

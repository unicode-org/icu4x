package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TransformResultLib: Library {
}
/** See the [Rust documentation for `TransformResult`](https://docs.rs/icu/2.1.1/icu/locale/enum.TransformResult.html) for more information.
*/
enum class TransformResult {
    Modified,
    Unmodified;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<TransformResultLib> = TransformResultLib::class.java
        internal val lib: TransformResultLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): TransformResult {
            return TransformResult.entries[native]
        }

        fun default(): TransformResult {
            return Modified
        }
    }
}

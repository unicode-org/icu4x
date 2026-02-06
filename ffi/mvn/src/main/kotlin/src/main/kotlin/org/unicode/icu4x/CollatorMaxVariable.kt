package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorMaxVariableLib: Library {
}
/** See the [Rust documentation for `MaxVariable`](https://docs.rs/icu/2.1.1/icu/collator/options/enum.MaxVariable.html) for more information.
*/
enum class CollatorMaxVariable {
    Space,
    Punctuation,
    Symbol,
    Currency;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<CollatorMaxVariableLib> = CollatorMaxVariableLib::class.java
        internal val lib: CollatorMaxVariableLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): CollatorMaxVariable {
            return CollatorMaxVariable.entries[native]
        }

        fun default(): CollatorMaxVariable {
            return Space
        }
    }
}

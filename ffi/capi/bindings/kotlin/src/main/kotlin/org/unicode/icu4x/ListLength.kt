package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ListLengthLib: Library {
}
/** See the [Rust documentation for `ListLength`](https://docs.rs/icu/2.1.1/icu/list/options/enum.ListLength.html) for more information.
*/
enum class ListLength {
    Wide,
    Short,
    Narrow;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<ListLengthLib> = ListLengthLib::class.java
        internal val lib: ListLengthLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): ListLength {
            return ListLength.entries[native]
        }

        fun default(): ListLength {
            return Wide
        }
    }
}
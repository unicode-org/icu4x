package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeAlignmentLib: Library {
}
/** See the [Rust documentation for `Alignment`](https://docs.rs/icu/2.1.1/icu/datetime/options/enum.Alignment.html) for more information.
*/
enum class DateTimeAlignment {
    Auto,
    Column;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateTimeAlignmentLib> = DateTimeAlignmentLib::class.java
        internal val lib: DateTimeAlignmentLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateTimeAlignment {
            return DateTimeAlignment.entries[native]
        }

        fun default(): DateTimeAlignment {
            return Auto
        }
    }
}
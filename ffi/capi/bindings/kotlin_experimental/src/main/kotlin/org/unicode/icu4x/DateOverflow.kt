package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateOverflowLib: Library {
}
/** 🚧 This API is experimental and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `Overflow`](https://docs.rs/icu/2.1.1/icu/calendar/options/enum.Overflow.html) for more information.
*/
enum class DateOverflow {
    Constrain,
    Reject;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateOverflowLib> = DateOverflowLib::class.java
        internal val lib: DateOverflowLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateOverflow {
            return DateOverflow.entries[native]
        }

        fun default(): DateOverflow {
            return Constrain
        }
    }
}
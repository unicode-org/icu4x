package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeLengthLib: Library {
}
/** See the [Rust documentation for `Length`](https://docs.rs/icu/2.1.1/icu/datetime/options/enum.Length.html) for more information.
*/
enum class DateTimeLength {
    Long,
    Medium,
    Short;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateTimeLengthLib> = DateTimeLengthLib::class.java
        internal val lib: DateTimeLengthLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateTimeLength {
            return DateTimeLength.entries[native]
        }

        fun default(): DateTimeLength {
            return Long
        }
    }
}

package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateMissingFieldsStrategyLib: Library {
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `MissingFieldsStrategy`](https://docs.rs/icu/2.1.1/icu/calendar/options/enum.MissingFieldsStrategy.html) for more information.
*/
enum class DateMissingFieldsStrategy {
    Reject,
    Ecma;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateMissingFieldsStrategyLib> = DateMissingFieldsStrategyLib::class.java
        internal val lib: DateMissingFieldsStrategyLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateMissingFieldsStrategy {
            return DateMissingFieldsStrategy.entries[native]
        }

        fun default(): DateMissingFieldsStrategy {
            return Reject
        }
    }
}

package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateDurationUnitLib: Library {
}
/** See the [Rust documentation for `DateDurationUnit`](https://docs.rs/icu/2.1.1/icu/calendar/types/enum.DateDurationUnit.html) for more information.
*/
enum class DateDurationUnit {
    Years,
    Months,
    Weeks,
    Days;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateDurationUnitLib> = DateDurationUnitLib::class.java
        internal val lib: DateDurationUnitLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateDurationUnit {
            return DateDurationUnit.entries[native]
        }

        fun default(): DateDurationUnit {
            return Years
        }
    }
}

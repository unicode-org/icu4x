package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface WeekdayLib: Library {
}
/** See the [Rust documentation for `Weekday`](https://docs.rs/icu/2.1.1/icu/calendar/types/enum.Weekday.html) for more information.
*/
enum class Weekday(val inner: Int) {
    Monday(1),
    Tuesday(2),
    Wednesday(3),
    Thursday(4),
    Friday(5),
    Saturday(6),
    Sunday(7);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<WeekdayLib> = WeekdayLib::class.java
        internal val lib: WeekdayLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): Weekday {
            return when (native) {
                1 -> Monday
                2 -> Tuesday
                3 -> Wednesday
                4 -> Thursday
                5 -> Friday
                6 -> Saturday
                7 -> Sunday
                else -> throw RuntimeException("Failed to find variant ${native} of type Weekday")
            }
        }

        fun default(): Weekday {
            return Monday
        }
    }
}
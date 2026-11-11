package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationUnitStyleLib: Library {
}
/**
 * See the [Rust documentation for `YearStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.YearStyle.html) for more information.
 *
 * See the [Rust documentation for `HourStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.HourStyle.html) for more information.
 *
 * See the [Rust documentation for `MilliSecondStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.MilliSecondStyle.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class DurationUnitStyle {
    Default,
    Narrow,
    Short,
    Long,
    TwoDigit,
    Numeric,
    Fractional;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DurationUnitStyleLib> = DurationUnitStyleLib::class.java
        internal val lib: DurationUnitStyleLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DurationUnitStyle {
            return DurationUnitStyle.entries[native]
        }

        fun default(): DurationUnitStyle {
            return Default
        }
    }
}

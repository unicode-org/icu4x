package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneVariantLib: Library {
    fun icu4x_TimeZoneVariant_from_rearguard_isdst_mv1(isdst: Boolean): Int
}
/** See the [Rust documentation for `TimeZoneVariant`](https://docs.rs/icu/2.1.1/icu/time/zone/enum.TimeZoneVariant.html) for more information.
*/
enum class TimeZoneVariant {
    Standard,
    Daylight;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<TimeZoneVariantLib> = TimeZoneVariantLib::class.java
        internal val lib: TimeZoneVariantLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): TimeZoneVariant {
            return TimeZoneVariant.entries[native]
        }

        fun default(): TimeZoneVariant {
            return Standard
        }
        @JvmStatic
        
        /** See the [Rust documentation for `from_rearguard_isdst`](https://docs.rs/icu/2.1.1/icu/time/zone/enum.TimeZoneVariant.html#method.from_rearguard_isdst) for more information.
        *
        *See the [Rust documentation for `with_variant`](https://docs.rs/icu/2.1.1/icu/time/struct.TimeZoneInfo.html#method.with_variant) for more information.
        */
        fun fromRearguardIsdst(isdst: Boolean): TimeZoneVariant {
            
            val returnVal = lib.icu4x_TimeZoneVariant_from_rearguard_isdst_mv1(isdst);
            return (TimeZoneVariant.fromNative(returnVal))
        }
    }
}
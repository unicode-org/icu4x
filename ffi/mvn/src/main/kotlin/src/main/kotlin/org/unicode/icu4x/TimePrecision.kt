package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimePrecisionLib: Library {
    fun icu4x_TimePrecision_from_subsecond_digits_mv1(digits: FFIUint8): OptionInt
}
/** See the [Rust documentation for `TimePrecision`](https://docs.rs/icu/2.1.1/icu/datetime/options/enum.TimePrecision.html) for more information.
*
*See the [Rust documentation for `SubsecondDigits`](https://docs.rs/icu/2.1.1/icu/datetime/options/enum.SubsecondDigits.html) for more information.
*/
enum class TimePrecision {
    Hour,
    Minute,
    MinuteOptional,
    Second,
    Subsecond1,
    Subsecond2,
    Subsecond3,
    Subsecond4,
    Subsecond5,
    Subsecond6,
    Subsecond7,
    Subsecond8,
    Subsecond9;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<TimePrecisionLib> = TimePrecisionLib::class.java
        internal val lib: TimePrecisionLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): TimePrecision {
            return TimePrecision.entries[native]
        }

        fun default(): TimePrecision {
            return Hour
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_from_int`](https://docs.rs/icu/2.1.1/icu/datetime/options/enum.SubsecondDigits.html#method.try_from_int) for more information.
        */
        fun fromSubsecondDigits(digits: UByte): TimePrecision? {
            
            val returnVal = lib.icu4x_TimePrecision_from_subsecond_digits_mv1(FFIUint8(digits));
            
            val intermediateOption = returnVal.option() ?: return null
            return TimePrecision.fromNative(intermediateOption)
        }
    }
}
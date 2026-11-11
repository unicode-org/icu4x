package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DurationFormatterErrorLib: Library {
}
/**
 * See the [Rust documentation for `DurationFormatterOptionsError`](https://docs.rs/icu/2.4.0/icu/experimental/duration/enum.DurationFormatterOptionsError.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class DurationFormatterError {
    FractionalDigitsOutOfRange,
    PreviousNumeric,
    DisplayAlwaysFractional,
    PreviousFractional,
    InvalidUnitStyle,
    DataError;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DurationFormatterErrorLib> = DurationFormatterErrorLib::class.java
        internal val lib: DurationFormatterErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DurationFormatterError {
            return DurationFormatterError.entries[native]
        }

        fun default(): DurationFormatterError {
            return FractionalDigitsOutOfRange
        }
    }
}
class DurationFormatterErrorError internal constructor(internal val value: DurationFormatterError): Exception("Rust error result for DurationFormatterError") {
    override fun toString(): String {
        return "DurationFormatterError error with value " + value
    }

    fun getValue(): DurationFormatterError = value
}

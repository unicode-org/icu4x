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
 * Additional information: [1](https://docs.rs/icu_provider/2.4.0/icu_provider/struct.DataError.html), [2](https://docs.rs/icu_provider/2.4.0/icu_provider/enum.DataErrorKind.html)
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class DurationFormatterError(val inner: Int) {
    Unknown(0),
    FractionalDigitsOutOfRange(2049),
    PreviousNumeric(2050),
    DisplayAlwaysFractional(2051),
    PreviousFractional(2052),
    InvalidUnitStyle(2053),
    DataMarkerNotFound(1),
    DataIdentifierNotFound(2),
    DataInvalidRequest(3),
    DataInconsistentData(4),
    DataDowncast(5),
    DataDeserialize(6),
    DataCustom(7),
    DataIo(8);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<DurationFormatterErrorLib> = DurationFormatterErrorLib::class.java
        internal val lib: DurationFormatterErrorLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): DurationFormatterError {
            return when (native) {
                0 -> Unknown
                2049 -> FractionalDigitsOutOfRange
                2050 -> PreviousNumeric
                2051 -> DisplayAlwaysFractional
                2052 -> PreviousFractional
                2053 -> InvalidUnitStyle
                1 -> DataMarkerNotFound
                2 -> DataIdentifierNotFound
                3 -> DataInvalidRequest
                4 -> DataInconsistentData
                5 -> DataDowncast
                6 -> DataDeserialize
                7 -> DataCustom
                8 -> DataIo
                else -> throw RuntimeException("Failed to find variant ${native} of type DurationFormatterError")
            }
        }

        fun default(): DurationFormatterError {
            return Unknown
        }
    }
}
class DurationFormatterErrorError internal constructor(internal val value: DurationFormatterError): Exception("Rust error result for DurationFormatterError") {
    override fun toString(): String {
        return "DurationFormatterError error with value " + value
    }

    fun getValue(): DurationFormatterError = value
}

package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeFormatterLoadErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/enum.DateTimeFormatterLoadError.html), [2](https://docs.rs/icu/2.1.1/icu/datetime/pattern/enum.PatternLoadError.html), [3](https://docs.rs/icu_provider/2.1.1/icu_provider/struct.DataError.html), [4](https://docs.rs/icu_provider/2.1.1/icu_provider/enum.DataErrorKind.html)
*/
enum class DateTimeFormatterLoadError(val inner: Int) {
    Unknown(0),
    InvalidDateFields(2049),
    UnsupportedLength(2051),
    ConflictingField(2057),
    FormatterTooSpecific(2058),
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
        internal val libClass: Class<DateTimeFormatterLoadErrorLib> = DateTimeFormatterLoadErrorLib::class.java
        internal val lib: DateTimeFormatterLoadErrorLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): DateTimeFormatterLoadError {
            return when (native) {
                0 -> Unknown
                2049 -> InvalidDateFields
                2051 -> UnsupportedLength
                2057 -> ConflictingField
                2058 -> FormatterTooSpecific
                1 -> DataMarkerNotFound
                2 -> DataIdentifierNotFound
                3 -> DataInvalidRequest
                4 -> DataInconsistentData
                5 -> DataDowncast
                6 -> DataDeserialize
                7 -> DataCustom
                8 -> DataIo
                else -> throw RuntimeException("Failed to find variant ${native} of type DateTimeFormatterLoadError")
            }
        }

        fun default(): DateTimeFormatterLoadError {
            return Unknown
        }
    }
}
class DateTimeFormatterLoadErrorError internal constructor(internal val value: DateTimeFormatterLoadError): Exception("Rust error result for DateTimeFormatterLoadError") {
    override fun toString(): String {
        return "DateTimeFormatterLoadError error with value " + value
    }

    fun getValue(): DateTimeFormatterLoadError = value
}
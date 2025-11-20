package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeWriteErrorLib: Library {
}
/** An error when formatting a datetime.
*
*Currently never returned by any API.
*
*Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/unchecked/enum.FormattedDateTimeUncheckedError.html)
*/
enum class DateTimeWriteError {
    Unknown,
    MissingTimeZoneVariant;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DateTimeWriteErrorLib> = DateTimeWriteErrorLib::class.java
        internal val lib: DateTimeWriteErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DateTimeWriteError {
            return DateTimeWriteError.entries[native]
        }

        fun default(): DateTimeWriteError {
            return Unknown
        }
    }
}
class DateTimeWriteErrorError internal constructor(internal val value: DateTimeWriteError): Exception("Rust error result for DateTimeWriteError") {
    override fun toString(): String {
        return "DateTimeWriteError error with value " + value
    }

    fun getValue(): DateTimeWriteError = value
}
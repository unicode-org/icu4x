package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DataErrorLib: Library {
}
/** Additional information: [1](https://docs.rs/icu_provider/2.1.1/icu_provider/struct.DataError.html), [2](https://docs.rs/icu_provider/2.1.1/icu_provider/enum.DataErrorKind.html)
*/
enum class DataError {
    Unknown,
    MarkerNotFound,
    IdentifierNotFound,
    InvalidRequest,
    InconsistentData,
    Downcast,
    Deserialize,
    Custom,
    Io;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DataErrorLib> = DataErrorLib::class.java
        internal val lib: DataErrorLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DataError {
            return DataError.entries[native]
        }

        fun default(): DataError {
            return Unknown
        }
    }
}
class DataErrorError internal constructor(internal val value: DataError): Exception("Rust error result for DataError") {
    override fun toString(): String {
        return "DataError error with value " + value
    }

    fun getValue(): DataError = value
}

package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneAndCanonicalLib: Library {
}

internal class TimeZoneAndCanonicalNative: Structure(), Structure.ByValue {
    @JvmField
    internal var timeZone: Pointer = Pointer(0);
    @JvmField
    internal var canonical: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("timeZone", "canonical")
    }
}




internal class OptionTimeZoneAndCanonicalNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: TimeZoneAndCanonicalNative = TimeZoneAndCanonicalNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TimeZoneAndCanonicalNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: TimeZoneAndCanonicalNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: TimeZoneAndCanonicalNative): OptionTimeZoneAndCanonicalNative {
            return OptionTimeZoneAndCanonicalNative(value, 1)
        }

        internal fun none(): OptionTimeZoneAndCanonicalNative {
            return OptionTimeZoneAndCanonicalNative(TimeZoneAndCanonicalNative(), 0)
        }
    }

}

/** See the [Rust documentation for `TimeZoneAndCanonical`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonical.html) for more information.
*/
class TimeZoneAndCanonical (var timeZone: TimeZone, var canonical: String) {
    companion object {

        internal val libClass: Class<TimeZoneAndCanonicalLib> = TimeZoneAndCanonicalLib::class.java
        internal val lib: TimeZoneAndCanonicalLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TimeZoneAndCanonicalNative::class.java).toLong()

        internal fun fromNative(nativeStruct: TimeZoneAndCanonicalNative, aEdges: List<Any?>): TimeZoneAndCanonical {
            val timeZone: TimeZone = TimeZone(nativeStruct.timeZone, listOf())
            val canonical: String = PrimitiveArrayTools.getUtf8(nativeStruct.canonical)

            return TimeZoneAndCanonical(timeZone, canonical)
        }

    }
    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}
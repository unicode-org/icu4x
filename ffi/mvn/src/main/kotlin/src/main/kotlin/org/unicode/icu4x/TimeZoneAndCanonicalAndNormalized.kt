package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneAndCanonicalAndNormalizedLib: Library {
}

internal class TimeZoneAndCanonicalAndNormalizedNative: Structure(), Structure.ByValue {
    @JvmField
    internal var timeZone: Pointer = Pointer(0);
    @JvmField
    internal var canonical: Slice = Slice();
    @JvmField
    internal var normalized: Slice = Slice();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("timeZone", "canonical", "normalized")
    }
}




internal class OptionTimeZoneAndCanonicalAndNormalizedNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: TimeZoneAndCanonicalAndNormalizedNative = TimeZoneAndCanonicalAndNormalizedNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TimeZoneAndCanonicalAndNormalizedNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: TimeZoneAndCanonicalAndNormalizedNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: TimeZoneAndCanonicalAndNormalizedNative): OptionTimeZoneAndCanonicalAndNormalizedNative {
            return OptionTimeZoneAndCanonicalAndNormalizedNative(value, 1)
        }

        internal fun none(): OptionTimeZoneAndCanonicalAndNormalizedNative {
            return OptionTimeZoneAndCanonicalAndNormalizedNative(TimeZoneAndCanonicalAndNormalizedNative(), 0)
        }
    }

}

/** See the [Rust documentation for `TimeZoneAndCanonicalAndNormalized`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonicalAndNormalized.html) for more information.
*/
class TimeZoneAndCanonicalAndNormalized (var timeZone: TimeZone, var canonical: String, var normalized: String) {
    companion object {

        internal val libClass: Class<TimeZoneAndCanonicalAndNormalizedLib> = TimeZoneAndCanonicalAndNormalizedLib::class.java
        internal val lib: TimeZoneAndCanonicalAndNormalizedLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TimeZoneAndCanonicalAndNormalizedNative::class.java).toLong()

        internal fun fromNative(nativeStruct: TimeZoneAndCanonicalAndNormalizedNative, aEdges: List<Any?>): TimeZoneAndCanonicalAndNormalized {
            val timeZone: TimeZone = TimeZone(nativeStruct.timeZone, listOf())
            val canonical: String = PrimitiveArrayTools.getUtf8(nativeStruct.canonical)
            val normalized: String = PrimitiveArrayTools.getUtf8(nativeStruct.normalized)

            return TimeZoneAndCanonicalAndNormalized(timeZone, canonical, normalized)
        }

    }
    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}
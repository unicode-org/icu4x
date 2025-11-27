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

/** See the [Rust documentation for `TimeZoneAndCanonicalAndNormalized`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonicalAndNormalized.html) for more information.
*/
class TimeZoneAndCanonicalAndNormalized internal constructor (
    internal val nativeStruct: TimeZoneAndCanonicalAndNormalizedNative,
    internal val aEdges: List<Any?>
    ) {
    val timeZone: TimeZone = TimeZone(nativeStruct.timeZone, listOf())
    val canonical: String = PrimitiveArrayTools.getUtf8(nativeStruct.canonical)
    val normalized: String = PrimitiveArrayTools.getUtf8(nativeStruct.normalized)

    companion object {
        internal val libClass: Class<TimeZoneAndCanonicalAndNormalizedLib> = TimeZoneAndCanonicalAndNormalizedLib::class.java
        internal val lib: TimeZoneAndCanonicalAndNormalizedLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TimeZoneAndCanonicalAndNormalizedNative::class.java).toLong()
    }

}

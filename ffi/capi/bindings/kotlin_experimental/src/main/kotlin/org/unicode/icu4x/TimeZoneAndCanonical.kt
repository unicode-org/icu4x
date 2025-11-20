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

/** See the [Rust documentation for `TimeZoneAndCanonical`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.TimeZoneAndCanonical.html) for more information.
*/
class TimeZoneAndCanonical internal constructor (
    internal val nativeStruct: TimeZoneAndCanonicalNative,
    internal val aEdges: List<Any?>
    ) {
    val timeZone: TimeZone = TimeZone(nativeStruct.timeZone, listOf())
    val canonical: String = PrimitiveArrayTools.getUtf8(nativeStruct.canonical)

    companion object {
        internal val libClass: Class<TimeZoneAndCanonicalLib> = TimeZoneAndCanonicalLib::class.java
        internal val lib: TimeZoneAndCanonicalLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TimeZoneAndCanonicalNative::class.java).toLong()
    }

}

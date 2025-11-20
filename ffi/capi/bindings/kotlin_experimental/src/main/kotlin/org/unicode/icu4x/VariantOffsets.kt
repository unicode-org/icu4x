package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface VariantOffsetsLib: Library {
}

internal class VariantOffsetsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var standard: Pointer = Pointer(0);
    @JvmField
    internal var daylight: Pointer? = null;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("standard", "daylight")
    }
}

/** See the [Rust documentation for `VariantOffsets`](https://docs.rs/icu/2.1.1/icu/time/zone/struct.VariantOffsets.html) for more information.
*/
class VariantOffsets internal constructor (
    internal val nativeStruct: VariantOffsetsNative) {
    val standard: UtcOffset = UtcOffset(nativeStruct.standard, listOf())
    val daylight: UtcOffset? = if (nativeStruct.daylight == null) {
        null
    } else {
        UtcOffset(nativeStruct.daylight!!, listOf())
    }

    companion object {
        internal val libClass: Class<VariantOffsetsLib> = VariantOffsetsLib::class.java
        internal val lib: VariantOffsetsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(VariantOffsetsNative::class.java).toLong()
    }

}

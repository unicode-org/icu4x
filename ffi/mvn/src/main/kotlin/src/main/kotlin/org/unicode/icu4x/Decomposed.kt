package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecomposedLib: Library {
}

internal class DecomposedNative: Structure(), Structure.ByValue {
    @JvmField
    internal var first: Int = 0;
    @JvmField
    internal var second: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("first", "second")
    }
}




internal class OptionDecomposedNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DecomposedNative = DecomposedNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DecomposedNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DecomposedNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DecomposedNative): OptionDecomposedNative {
            return OptionDecomposedNative(value, 1)
        }

        internal fun none(): OptionDecomposedNative {
            return OptionDecomposedNative(DecomposedNative(), 0)
        }
    }

}

/** The outcome of non-recursive canonical decomposition of a character.
*`second` will be NUL when the decomposition expands to a single character
*(which may or may not be the original one)
*
*See the [Rust documentation for `Decomposed`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/enum.Decomposed.html) for more information.
*/
class Decomposed (var first: Int, var second: Int) {
    companion object {

        internal val libClass: Class<DecomposedLib> = DecomposedLib::class.java
        internal val lib: DecomposedLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DecomposedNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DecomposedNative): Decomposed {
            val first: Int = nativeStruct.first
            val second: Int = nativeStruct.second

            return Decomposed(first, second)
        }

    }
}
package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorResolvedOptionsLib: Library {
}

internal class CollatorResolvedOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var strength: Int = CollatorStrength.default().toNative();
    @JvmField
    internal var alternateHandling: Int = CollatorAlternateHandling.default().toNative();
    @JvmField
    internal var caseFirst: Int = CollatorCaseFirst.default().toNative();
    @JvmField
    internal var maxVariable: Int = CollatorMaxVariable.default().toNative();
    @JvmField
    internal var caseLevel: Int = CollatorCaseLevel.default().toNative();
    @JvmField
    internal var numeric: Int = CollatorNumericOrdering.default().toNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("strength", "alternateHandling", "caseFirst", "maxVariable", "caseLevel", "numeric")
    }
}




internal class OptionCollatorResolvedOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: CollatorResolvedOptionsNative = CollatorResolvedOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): CollatorResolvedOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: CollatorResolvedOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: CollatorResolvedOptionsNative): OptionCollatorResolvedOptionsNative {
            return OptionCollatorResolvedOptionsNative(value, 1)
        }

        internal fun none(): OptionCollatorResolvedOptionsNative {
            return OptionCollatorResolvedOptionsNative(CollatorResolvedOptionsNative(), 0)
        }
    }

}

/** See the [Rust documentation for `ResolvedCollatorOptions`](https://docs.rs/icu/2.1.1/icu/collator/options/struct.ResolvedCollatorOptions.html) for more information.
*/
class CollatorResolvedOptions (var strength: CollatorStrength, var alternateHandling: CollatorAlternateHandling, var caseFirst: CollatorCaseFirst, var maxVariable: CollatorMaxVariable, var caseLevel: CollatorCaseLevel, var numeric: CollatorNumericOrdering) {
    companion object {

        internal val libClass: Class<CollatorResolvedOptionsLib> = CollatorResolvedOptionsLib::class.java
        internal val lib: CollatorResolvedOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CollatorResolvedOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: CollatorResolvedOptionsNative): CollatorResolvedOptions {
            val strength: CollatorStrength = CollatorStrength.fromNative(nativeStruct.strength)
            val alternateHandling: CollatorAlternateHandling = CollatorAlternateHandling.fromNative(nativeStruct.alternateHandling)
            val caseFirst: CollatorCaseFirst = CollatorCaseFirst.fromNative(nativeStruct.caseFirst)
            val maxVariable: CollatorMaxVariable = CollatorMaxVariable.fromNative(nativeStruct.maxVariable)
            val caseLevel: CollatorCaseLevel = CollatorCaseLevel.fromNative(nativeStruct.caseLevel)
            val numeric: CollatorNumericOrdering = CollatorNumericOrdering.fromNative(nativeStruct.numeric)

            return CollatorResolvedOptions(strength, alternateHandling, caseFirst, maxVariable, caseLevel, numeric)
        }

    }
}
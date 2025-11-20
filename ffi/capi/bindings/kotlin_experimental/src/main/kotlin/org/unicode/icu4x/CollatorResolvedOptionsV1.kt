package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorResolvedOptionsV1Lib: Library {
}

internal class CollatorResolvedOptionsV1Native: Structure(), Structure.ByValue {
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

/** See the [Rust documentation for `ResolvedCollatorOptions`](https://docs.rs/icu/2.1.1/icu/collator/options/struct.ResolvedCollatorOptions.html) for more information.
*/
class CollatorResolvedOptionsV1 internal constructor (
    internal val nativeStruct: CollatorResolvedOptionsV1Native) {
    val strength: CollatorStrength = CollatorStrength.fromNative(nativeStruct.strength)
    val alternateHandling: CollatorAlternateHandling = CollatorAlternateHandling.fromNative(nativeStruct.alternateHandling)
    val caseFirst: CollatorCaseFirst = CollatorCaseFirst.fromNative(nativeStruct.caseFirst)
    val maxVariable: CollatorMaxVariable = CollatorMaxVariable.fromNative(nativeStruct.maxVariable)
    val caseLevel: CollatorCaseLevel = CollatorCaseLevel.fromNative(nativeStruct.caseLevel)
    val numeric: CollatorNumericOrdering = CollatorNumericOrdering.fromNative(nativeStruct.numeric)

    companion object {
        internal val libClass: Class<CollatorResolvedOptionsV1Lib> = CollatorResolvedOptionsV1Lib::class.java
        internal val lib: CollatorResolvedOptionsV1Lib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CollatorResolvedOptionsV1Native::class.java).toLong()
    }

}

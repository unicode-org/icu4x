package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorOptionsLib: Library {
}

internal class CollatorOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var strength: OptionInt = OptionInt.none();
    @JvmField
    internal var alternateHandling: OptionInt = OptionInt.none();
    @JvmField
    internal var maxVariable: OptionInt = OptionInt.none();
    @JvmField
    internal var caseLevel: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("strength", "alternateHandling", "maxVariable", "caseLevel")
    }
}




internal class OptionCollatorOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: CollatorOptionsNative = CollatorOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): CollatorOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: CollatorOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: CollatorOptionsNative): OptionCollatorOptionsNative {
            return OptionCollatorOptionsNative(value, 1)
        }

        internal fun none(): OptionCollatorOptionsNative {
            return OptionCollatorOptionsNative(CollatorOptionsNative(), 0)
        }
    }

}

/** See the [Rust documentation for `CollatorOptions`](https://docs.rs/icu/2.1.1/icu/collator/options/struct.CollatorOptions.html) for more information.
*/
class CollatorOptions (var strength: CollatorStrength?, var alternateHandling: CollatorAlternateHandling?, var maxVariable: CollatorMaxVariable?, var caseLevel: CollatorCaseLevel?) {
    companion object {

        internal val libClass: Class<CollatorOptionsLib> = CollatorOptionsLib::class.java
        internal val lib: CollatorOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(CollatorOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: CollatorOptionsNative): CollatorOptions {
            val strength: CollatorStrength? = nativeStruct.strength.option()?.let { CollatorStrength.fromNative(it) }
            val alternateHandling: CollatorAlternateHandling? = nativeStruct.alternateHandling.option()?.let { CollatorAlternateHandling.fromNative(it) }
            val maxVariable: CollatorMaxVariable? = nativeStruct.maxVariable.option()?.let { CollatorMaxVariable.fromNative(it) }
            val caseLevel: CollatorCaseLevel? = nativeStruct.caseLevel.option()?.let { CollatorCaseLevel.fromNative(it) }

            return CollatorOptions(strength, alternateHandling, maxVariable, caseLevel)
        }

    }
    internal fun toNative(): CollatorOptionsNative {
        var native = CollatorOptionsNative()
        native.strength = this.strength?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.alternateHandling = this.alternateHandling?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.maxVariable = this.maxVariable?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.caseLevel = this.caseLevel?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
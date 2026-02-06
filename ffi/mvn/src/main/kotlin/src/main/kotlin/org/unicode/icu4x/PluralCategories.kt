package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PluralCategoriesLib: Library {
}

internal class PluralCategoriesNative: Structure(), Structure.ByValue {
    @JvmField
    internal var zero: Byte = 0;
    @JvmField
    internal var one: Byte = 0;
    @JvmField
    internal var two: Byte = 0;
    @JvmField
    internal var few: Byte = 0;
    @JvmField
    internal var many: Byte = 0;
    @JvmField
    internal var other: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("zero", "one", "two", "few", "many", "other")
    }
}




internal class OptionPluralCategoriesNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: PluralCategoriesNative = PluralCategoriesNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): PluralCategoriesNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: PluralCategoriesNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: PluralCategoriesNative): OptionPluralCategoriesNative {
            return OptionPluralCategoriesNative(value, 1)
        }

        internal fun none(): OptionPluralCategoriesNative {
            return OptionPluralCategoriesNative(PluralCategoriesNative(), 0)
        }
    }

}

/** See the [Rust documentation for `categories`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.categories) for more information.
*/
class PluralCategories (var zero: Boolean, var one: Boolean, var two: Boolean, var few: Boolean, var many: Boolean, var other: Boolean) {
    companion object {

        internal val libClass: Class<PluralCategoriesLib> = PluralCategoriesLib::class.java
        internal val lib: PluralCategoriesLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(PluralCategoriesNative::class.java).toLong()

        internal fun fromNative(nativeStruct: PluralCategoriesNative): PluralCategories {
            val zero: Boolean = nativeStruct.zero > 0
            val one: Boolean = nativeStruct.one > 0
            val two: Boolean = nativeStruct.two > 0
            val few: Boolean = nativeStruct.few > 0
            val many: Boolean = nativeStruct.many > 0
            val other: Boolean = nativeStruct.other > 0

            return PluralCategories(zero, one, two, few, many, other)
        }

    }
}
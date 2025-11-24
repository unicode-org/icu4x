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

/** See the [Rust documentation for `categories`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.categories) for more information.
*/
class PluralCategories internal constructor (
    internal val nativeStruct: PluralCategoriesNative) {
    val zero: Boolean = nativeStruct.zero > 0
    val one: Boolean = nativeStruct.one > 0
    val two: Boolean = nativeStruct.two > 0
    val few: Boolean = nativeStruct.few > 0
    val many: Boolean = nativeStruct.many > 0
    val other: Boolean = nativeStruct.other > 0

    companion object {
        internal val libClass: Class<PluralCategoriesLib> = PluralCategoriesLib::class.java
        internal val lib: PluralCategoriesLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(PluralCategoriesNative::class.java).toLong()
    }

}

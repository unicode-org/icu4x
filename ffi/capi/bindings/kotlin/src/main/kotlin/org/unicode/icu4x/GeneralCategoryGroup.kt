package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface GeneralCategoryGroupLib: Library {
    fun icu4x_GeneralCategoryGroup_contains_mv1(nativeStruct: GeneralCategoryGroupNative, val_: Int): Byte
    fun icu4x_GeneralCategoryGroup_complement_mv1(nativeStruct: GeneralCategoryGroupNative): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_all_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_empty_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_union_mv1(nativeStruct: GeneralCategoryGroupNative, other: GeneralCategoryGroupNative): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_intersection_mv1(nativeStruct: GeneralCategoryGroupNative, other: GeneralCategoryGroupNative): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_cased_letter_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_letter_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_mark_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_number_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_separator_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_other_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_punctuation_mv1(): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryGroup_symbol_mv1(): GeneralCategoryGroupNative
}

internal class GeneralCategoryGroupNative: Structure(), Structure.ByValue {
    @JvmField
    internal var mask: FFIUint32 = FFIUint32();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("mask")
    }
}

/** A mask that is capable of representing groups of `General_Category` values.
*
*See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
*/
class GeneralCategoryGroup internal constructor (
    internal val nativeStruct: GeneralCategoryGroupNative) {
    val mask: UInt = nativeStruct.mask.toUInt()

    companion object {
        internal val libClass: Class<GeneralCategoryGroupLib> = GeneralCategoryGroupLib::class.java
        internal val lib: GeneralCategoryGroupLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(GeneralCategoryGroupNative::class.java).toLong()
        @JvmStatic
        
        /** See the [Rust documentation for `all`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#method.all) for more information.
        */
        fun all(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_all_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `empty`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#method.empty) for more information.
        */
        fun empty(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_empty_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `CasedLetter`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.CasedLetter) for more information.
        */
        fun casedLetter(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_cased_letter_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Letter`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Letter) for more information.
        */
        fun letter(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_letter_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Mark`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Mark) for more information.
        */
        fun mark(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_mark_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Number`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Number) for more information.
        */
        fun number(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_number_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Other`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Other) for more information.
        */
        fun separator(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_separator_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Letter`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Letter) for more information.
        */
        fun other(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_other_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Punctuation`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Punctuation) for more information.
        */
        fun punctuation(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_punctuation_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** See the [Rust documentation for `Symbol`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Symbol) for more information.
        */
        fun symbol(): GeneralCategoryGroup {
            
            val returnVal = lib.icu4x_GeneralCategoryGroup_symbol_mv1();
            
            val returnStruct = GeneralCategoryGroup(returnVal)
            return returnStruct
        }
    }
    
    /** See the [Rust documentation for `contains`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#method.contains) for more information.
    */
    fun contains(val_: GeneralCategory): Boolean {
        
        val returnVal = lib.icu4x_GeneralCategoryGroup_contains_mv1(nativeStruct, val_.toNative());
        return (returnVal > 0)
    }
    
    /** See the [Rust documentation for `complement`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#method.complement) for more information.
    */
    fun complement(): GeneralCategoryGroup {
        
        val returnVal = lib.icu4x_GeneralCategoryGroup_complement_mv1(nativeStruct);
        
        val returnStruct = GeneralCategoryGroup(returnVal)
        return returnStruct
    }
    
    /** See the [Rust documentation for `union`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#method.union) for more information.
    */
    fun union(other: GeneralCategoryGroup): GeneralCategoryGroup {
        
        val returnVal = lib.icu4x_GeneralCategoryGroup_union_mv1(nativeStruct, other.nativeStruct);
        
        val returnStruct = GeneralCategoryGroup(returnVal)
        return returnStruct
    }
    
    /** See the [Rust documentation for `intersection`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html#method.intersection) for more information.
    */
    fun intersection(other: GeneralCategoryGroup): GeneralCategoryGroup {
        
        val returnVal = lib.icu4x_GeneralCategoryGroup_intersection_mv1(nativeStruct, other.nativeStruct);
        
        val returnStruct = GeneralCategoryGroup(returnVal)
        return returnStruct
    }

}

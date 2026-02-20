package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleLib: Library {
    fun icu4x_Locale_destroy_mv1(handle: Pointer)
    fun icu4x_Locale_from_string_mv1(name: Slice): ResultPointerInt
    fun icu4x_Locale_unknown_mv1(): Pointer
    fun icu4x_Locale_clone_mv1(handle: Pointer): Pointer
    fun icu4x_Locale_basename_mv1(handle: Pointer, write: Pointer): Unit
    fun icu4x_Locale_get_unicode_extension_mv1(handle: Pointer, s: Slice, write: Pointer): OptionUnit
    fun icu4x_Locale_set_unicode_extension_mv1(handle: Pointer, k: Slice, v: Slice): OptionUnit
    fun icu4x_Locale_language_mv1(handle: Pointer, write: Pointer): Unit
    fun icu4x_Locale_set_language_mv1(handle: Pointer, s: Slice): ResultUnitInt
    fun icu4x_Locale_region_mv1(handle: Pointer, write: Pointer): OptionUnit
    fun icu4x_Locale_set_region_mv1(handle: Pointer, s: Slice): ResultUnitInt
    fun icu4x_Locale_script_mv1(handle: Pointer, write: Pointer): OptionUnit
    fun icu4x_Locale_set_script_mv1(handle: Pointer, s: Slice): ResultUnitInt
    fun icu4x_Locale_variants_mv1(handle: Pointer, write: Pointer): Unit
    fun icu4x_Locale_variant_count_mv1(handle: Pointer): FFISizet
    fun icu4x_Locale_variant_at_mv1(handle: Pointer, index: FFISizet, write: Pointer): OptionUnit
    fun icu4x_Locale_has_variant_mv1(handle: Pointer, s: Slice): Byte
    fun icu4x_Locale_add_variant_mv1(handle: Pointer, s: Slice): ResultByteInt
    fun icu4x_Locale_remove_variant_mv1(handle: Pointer, s: Slice): Byte
    fun icu4x_Locale_clear_variants_mv1(handle: Pointer): Unit
    fun icu4x_Locale_normalize_mv1(s: Slice, write: Pointer): ResultUnitInt
    fun icu4x_Locale_to_string_mv1(handle: Pointer, write: Pointer): Unit
    fun icu4x_Locale_normalizing_eq_mv1(handle: Pointer, other: Slice): Byte
    fun icu4x_Locale_compare_to_string_mv1(handle: Pointer, other: Slice): Byte
    fun icu4x_Locale_compare_to_mv1(handle: Pointer, other: Pointer): Byte
}
/** An ICU4X Locale, capable of representing strings like `"en-US"`.
*
*See the [Rust documentation for `Locale`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html) for more information.
*/
class Locale internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LocaleCleaner(val handle: Pointer, val lib: LocaleLib) : Runnable {
        override fun run() {
            lib.icu4x_Locale_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleLib> = LocaleLib::class.java
        internal val lib: LocaleLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct an [Locale] from an locale identifier.
        *
        *This will run the complete locale parsing algorithm. If code size and
        *performance are critical and the locale is of a known shape (such as
        *`aa-BB`) use `create_und`, `set_language`, `set_script`, and `set_region`.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.try_from_str) for more information.
        */
        fun fromString(name: String): Result<Locale> {
            val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
            
            val returnVal = lib.icu4x_Locale_from_string_mv1(nameSliceMemory.slice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Locale(handle, selfEdges)
                CLEANER.register(returnOpaque, Locale.LocaleCleaner(handle, Locale.lib));
                nameSliceMemory?.close()
                return returnOpaque.ok()
            } else {
                return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a unknown [Locale] "und".
        *
        *See the [Rust documentation for `UNKNOWN`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#associatedconstant.UNKNOWN) for more information.
        */
        fun unknown(): Locale {
            
            val returnVal = lib.icu4x_Locale_unknown_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Locale(handle, selfEdges)
            CLEANER.register(returnOpaque, Locale.LocaleCleaner(handle, Locale.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Normalizes a locale string.
        *
        *See the [Rust documentation for `normalize`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.normalize) for more information.
        */
        fun normalize(s: String): Result<String> {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_Locale_normalize_mv1(sSliceMemory.slice, write);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Clones the [Locale].
    *
    *See the [Rust documentation for `Locale`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html) for more information.
    */
    fun clone(): Locale {
        
        val returnVal = lib.icu4x_Locale_clone_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Locale(handle, selfEdges)
        CLEANER.register(returnOpaque, Locale.LocaleCleaner(handle, Locale.lib));
        return returnOpaque
    }
    
    /** Returns a string representation of the `LanguageIdentifier` part of
    *[Locale].
    *
    *See the [Rust documentation for `id`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#structfield.id) for more information.
    */
    fun basename(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_basename_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Returns a string representation of the unicode extension.
    *
    *See the [Rust documentation for `extensions`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#structfield.extensions) for more information.
    */
    fun getUnicodeExtension(s: String): String? {
        val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_get_unicode_extension_mv1(handle, sSliceMemory.slice, write);
        
        returnVal.option() ?: return null

        val returnString = DW.writeToString(write)
        return returnString
                                
    }
    
    /** Set a Unicode extension.
    *
    *See the [Rust documentation for `extensions`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#structfield.extensions) for more information.
    */
    fun setUnicodeExtension(k: String, v: String): Unit? {
        val kSliceMemory = PrimitiveArrayTools.borrowUtf8(k)
        val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
        
        val returnVal = lib.icu4x_Locale_set_unicode_extension_mv1(handle, kSliceMemory.slice, vSliceMemory.slice);
        return returnVal.option()
    }
    
    /** Returns a string representation of [Locale] language.
    *
    *See the [Rust documentation for `id`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#structfield.id) for more information.
    */
    fun language(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_language_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Set the language part of the [Locale].
    *
    *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.try_from_str) for more information.
    */
    fun setLanguage(s: String): Result<Unit> {
        val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_Locale_set_language_mv1(handle, sSliceMemory.slice);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** Returns a string representation of [Locale] region.
    *
    *See the [Rust documentation for `id`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#structfield.id) for more information.
    */
    fun region(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_region_mv1(handle, write);
        
        returnVal.option() ?: return null

        val returnString = DW.writeToString(write)
        return returnString
                                
    }
    
    /** Set the region part of the [Locale].
    *
    *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.try_from_str) for more information.
    */
    fun setRegion(s: String): Result<Unit> {
        val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_Locale_set_region_mv1(handle, sSliceMemory.slice);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** Returns a string representation of [Locale] script.
    *
    *See the [Rust documentation for `id`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#structfield.id) for more information.
    */
    fun script(): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_script_mv1(handle, write);
        
        returnVal.option() ?: return null

        val returnString = DW.writeToString(write)
        return returnString
                                
    }
    
    /** Set the script part of the [Locale]. Pass an empty string to remove the script.
    *
    *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.try_from_str) for more information.
    */
    fun setScript(s: String): Result<Unit> {
        val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_Locale_set_script_mv1(handle, sSliceMemory.slice);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** Returns a string representation of the [Locale] variants.
    *
    *See the [Rust documentation for `Variants`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html) for more information.
    */
    fun variants(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_variants_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Returns the number of variants in this [Locale].
    *
    *See the [Rust documentation for `Variants`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html) for more information.
    */
    fun variantCount(): ULong {
        
        val returnVal = lib.icu4x_Locale_variant_count_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** Returns the variant at the given index, or nothing if the index is out of bounds.
    *
    *See the [Rust documentation for `Variants`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html) for more information.
    */
    fun variantAt(index: ULong): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_variant_at_mv1(handle, FFISizet(index), write);
        
        returnVal.option() ?: return null

        val returnString = DW.writeToString(write)
        return returnString
                                
    }
    
    /** Returns whether the [Locale] has a specific variant.
    *
    *See the [Rust documentation for `Variants`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html) for more information.
    */
    fun hasVariant(s: String): Boolean {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_Locale_has_variant_mv1(handle, sSlice);
        return (returnVal > 0)
    }
    
    /** Adds a variant to the [Locale].
    *
    *Returns an error if the variant string is invalid.
    *Returns `true` if the variant was added, `false` if already present.
    *
    *See the [Rust documentation for `push`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html#method.push) for more information.
    */
    fun addVariant(s: String): Result<Boolean> {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_Locale_add_variant_mv1(handle, sSlice);
        if (returnVal.isOk == 1.toByte()) {
            return (returnVal.union.ok > 0).ok()
        } else {
            return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** Removes a variant from the [Locale].
    *
    *Returns `true` if the variant was removed, `false` if not present.
    *Returns `false` for invalid variant strings (they cannot exist in the locale).
    *
    *See the [Rust documentation for `remove`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html#method.remove) for more information.
    */
    fun removeVariant(s: String): Boolean {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_Locale_remove_variant_mv1(handle, sSlice);
        return (returnVal > 0)
    }
    
    /** Clears all variants from the [Locale].
    *
    *See the [Rust documentation for `clear`](https://docs.rs/icu/2.1.1/icu/locale/struct.Variants.html#method.clear) for more information.
    */
    fun clearVariants(): Unit {
        
        val returnVal = lib.icu4x_Locale_clear_variants_mv1(handle);
        
    }
    
    /** Returns a string representation of [Locale].
    *
    *See the [Rust documentation for `write_to`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.write_to) for more information.
    */
    override fun toString(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Locale_to_string_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** See the [Rust documentation for `normalizing_eq`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.normalizing_eq) for more information.
    */
    fun normalizingEq(other: String): Boolean {
        val otherSliceMemory = PrimitiveArrayTools.borrowUtf8(other)
        
        val returnVal = lib.icu4x_Locale_normalizing_eq_mv1(handle, otherSliceMemory.slice);
        return (returnVal > 0)
    }
    
    /** See the [Rust documentation for `strict_cmp`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.strict_cmp) for more information.
    */
    fun compareToString(other: String): Byte {
        val otherSliceMemory = PrimitiveArrayTools.borrowUtf8(other)
        
        val returnVal = lib.icu4x_Locale_compare_to_string_mv1(handle, otherSliceMemory.slice);
        return (returnVal)
    }
    
    /** See the [Rust documentation for `total_cmp`](https://docs.rs/icu/2.1.1/icu/locale/struct.Locale.html#method.total_cmp) for more information.
    */
    fun compareTo(other: Locale): Byte {
        
        val returnVal = lib.icu4x_Locale_compare_to_mv1(handle, other.handle);
        return (returnVal)
    }

}
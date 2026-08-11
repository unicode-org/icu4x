package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleNamesUnstableLib: Library {
    fun icu4x_LocaleNamesUnstable_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleNamesUnstable_for_region_light_with_compiled_data_mv1(locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_light_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_tiny_with_compiled_data_mv1(locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_short_tiny_with_compiled_data_mv1(locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_short_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_short_light_with_compiled_data_mv1(locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_short_light_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_light_with_compiled_data_mv1(locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_light_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_tiny_with_compiled_data_mv1(locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_heavy_with_compiled_data_mv1(locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_short_heavy_with_compiled_data_mv1(locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_short_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_variant_heavy_with_compiled_data_mv1(locale: Pointer, variant: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_variant_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, variant: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_light_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_tiny_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_light_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_light_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_heavy_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_with_compiled_data_mv1(locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Slice, languageDisplay: Int, write: Pointer): ResultUnitInt
}
/** See the [Rust documentation for `RegionDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html) for more information.
*
*See the [Rust documentation for `ScriptDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html) for more information.
*
*See the [Rust documentation for `VariantDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.VariantDisplayName.html) for more information.
*
*See the [Rust documentation for `LanguageIdentifierDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html) for more information.
*/
class LocaleNamesUnstable internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class LocaleNamesUnstableCleaner(val handle: Pointer, val lib: LocaleNamesUnstableLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleNamesUnstable_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, LocaleNamesUnstable.LocaleNamesUnstableCleaner(handle, LocaleNamesUnstable.lib));
    }

    companion object {
        internal val libClass: Class<LocaleNamesUnstableLib> = LocaleNamesUnstableLib::class.java
        internal val lib: LocaleNamesUnstableLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_light) for more information.
        */
        fun forRegionLightWithCompiledData(locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_light_with_compiled_data_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_light) for more information.
        */
        fun forRegionLightWithProvider(provider: DataProvider, locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_light_with_provider_mv1(provider.handle, locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forRegionTinyWithCompiledData(locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_tiny_with_compiled_data_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forRegionTinyWithProvider(provider: DataProvider, locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_tiny_with_provider_mv1(provider.handle, locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_tiny) for more information.
        */
        fun forRegionShortTinyWithCompiledData(locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_short_tiny_with_compiled_data_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_tiny) for more information.
        */
        fun forRegionShortTinyWithProvider(provider: DataProvider, locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_short_tiny_with_provider_mv1(provider.handle, locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_light) for more information.
        */
        fun forRegionShortLightWithCompiledData(locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_short_light_with_compiled_data_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_light) for more information.
        */
        fun forRegionShortLightWithProvider(provider: DataProvider, locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_short_light_with_provider_mv1(provider.handle, locale.handle, regionSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_light) for more information.
        */
        fun forScriptLightWithCompiledData(locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_light_with_compiled_data_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_light) for more information.
        */
        fun forScriptLightWithProvider(provider: DataProvider, locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_light_with_provider_mv1(provider.handle, locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forScriptTinyWithCompiledData(locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_tiny_with_compiled_data_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forScriptTinyWithProvider(provider: DataProvider, locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_tiny_with_provider_mv1(provider.handle, locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forScriptHeavyWithCompiledData(locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_heavy_with_compiled_data_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forScriptHeavyWithProvider(provider: DataProvider, locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_heavy_with_provider_mv1(provider.handle, locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_short_heavy) for more information.
        */
        fun forScriptShortHeavyWithCompiledData(locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_short_heavy_with_compiled_data_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_short_heavy) for more information.
        */
        fun forScriptShortHeavyWithProvider(provider: DataProvider, locale: Locale, script: String): Result<String> {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_short_heavy_with_provider_mv1(provider.handle, locale.handle, scriptSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.VariantDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forVariantHeavyWithCompiledData(locale: Locale, variant: String): Result<String> {
            val variantSliceMemory = PrimitiveArrayTools.borrowUtf8(variant)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_variant_heavy_with_compiled_data_mv1(locale.handle, variantSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                variantSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.VariantDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forVariantHeavyWithProvider(provider: DataProvider, locale: Locale, variant: String): Result<String> {
            val variantSliceMemory = PrimitiveArrayTools.borrowUtf8(variant)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_variant_heavy_with_provider_mv1(provider.handle, locale.handle, variantSliceMemory.slice, write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                variantSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_light) for more information.
        */
        fun forLanguageIdentifierLightWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_light_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_light) for more information.
        */
        fun forLanguageIdentifierLightWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_light_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forLanguageIdentifierTinyWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_tiny_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forLanguageIdentifierTinyWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_tiny_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_light) for more information.
        */
        fun forLanguageIdentifierShortLightWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_light_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_light) for more information.
        */
        fun forLanguageIdentifierShortLightWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_light_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_long_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_light) for more information.
        */
        fun forLanguageIdentifierLongLightWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_light_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_long_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_light) for more information.
        */
        fun forLanguageIdentifierLongLightWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_light_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_light) for more information.
        */
        fun forLanguageIdentifierMenuLightWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_light) for more information.
        */
        fun forLanguageIdentifierMenuLightWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_light) for more information.
        */
        fun forLanguageIdentifierShortMenuLightWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_light) for more information.
        */
        fun forLanguageIdentifierShortMenuLightWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forLanguageIdentifierHeavyWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_heavy_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forLanguageIdentifierHeavyWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_heavy_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_heavy) for more information.
        */
        fun forLanguageIdentifierShortHeavyWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_heavy) for more information.
        */
        fun forLanguageIdentifierShortHeavyWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_long_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_heavy) for more information.
        */
        fun forLanguageIdentifierLongHeavyWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_long_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_heavy) for more information.
        */
        fun forLanguageIdentifierLongHeavyWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_heavy) for more information.
        */
        fun forLanguageIdentifierMenuHeavyWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_heavy) for more information.
        */
        fun forLanguageIdentifierMenuHeavyWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_heavy) for more information.
        */
        fun forLanguageIdentifierShortMenuHeavyWithCompiledData(locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_with_compiled_data_mv1(locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new_short_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_heavy) for more information.
        */
        fun forLanguageIdentifierShortMenuHeavyWithProvider(provider: DataProvider, locale: Locale, langid: String, languageDisplay: LanguageDisplay): Result<String> {
            val langidSliceMemory = PrimitiveArrayTools.borrowUtf8(langid)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_with_provider_mv1(provider.handle, locale.handle, langidSliceMemory.slice, languageDisplay.toNative(), write);
            try {
                val nativeOkVal = returnVal.getNativeOk();
                if (nativeOkVal != null) {
                    
                    val returnString = DW.writeToString(write)
                    return returnString.ok()
                } else {
                    return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
                }
            } finally {
                langidSliceMemory.close()
            }
        }
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleNamesUnstableLib: Library {
    fun icu4x_LocaleNamesUnstable_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleNamesUnstable_for_region_light_mv1(locale: Pointer, region: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_region_light_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_tiny_mv1(locale: Pointer, region: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_region_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_short_tiny_mv1(locale: Pointer, region: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_region_short_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_short_light_mv1(locale: Pointer, region: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_region_short_light_with_provider_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_light_mv1(locale: Pointer, script: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_script_light_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_tiny_mv1(locale: Pointer, script: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_script_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_heavy_mv1(locale: Pointer, script: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_script_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_script_short_heavy_mv1(locale: Pointer, script: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_script_short_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, script: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_variant_heavy_mv1(locale: Pointer, variant: Slice, write: Pointer): Unit
    fun icu4x_LocaleNamesUnstable_for_variant_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, variant: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_light_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_tiny_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_tiny_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_light_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_light_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_heavy_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_mv1(locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_with_provider_mv1(provider: Pointer, locale: Pointer, langid: Pointer, languageDisplay: Int, write: Pointer): ResultUnitInt
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*This struct holds free functions for loading display names for languages, scripts,
*regions, and language identifiers.
*
*See the [Rust documentation for `RegionDisplayName`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html) for more information.
*
*See the [Rust documentation for `ScriptDisplayName`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html) for more information.
*
*See the [Rust documentation for `VariantDisplayName`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.VariantDisplayName.html) for more information.
*
*See the [Rust documentation for `LanguageIdentifierDisplayName`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_light_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_light_with_fallback) for more information.
        */
        fun forRegionLight(locale: Locale, region: String): String {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_light_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_light_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_light_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_tiny_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_tiny_with_fallback) for more information.
        */
        fun forRegionTiny(locale: Locale, region: String): String {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_tiny_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_tiny_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_tiny_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_short_tiny_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_short_tiny_with_fallback) for more information.
        */
        fun forRegionShortTiny(locale: Locale, region: String): String {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_short_tiny_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_short_tiny_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_short_tiny_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_short_light_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_short_light_with_fallback) for more information.
        */
        fun forRegionShortLight(locale: Locale, region: String): String {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_short_light_mv1(locale.handle, regionSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                regionSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_short_light_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.RegionDisplayName.html#method.new_short_light_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_light_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_light_with_fallback) for more information.
        */
        fun forScriptLight(locale: Locale, script: String): String {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_light_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_light_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_light_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_tiny_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_tiny_with_fallback) for more information.
        */
        fun forScriptTiny(locale: Locale, script: String): String {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_tiny_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_tiny_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_tiny_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_heavy_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_heavy_with_fallback) for more information.
        */
        fun forScriptHeavy(locale: Locale, script: String): String {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_heavy_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_heavy_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_heavy_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_short_heavy_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_short_heavy_with_fallback) for more information.
        */
        fun forScriptShortHeavy(locale: Locale, script: String): String {
            val scriptSliceMemory = PrimitiveArrayTools.borrowUtf8(script)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_script_short_heavy_mv1(locale.handle, scriptSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                scriptSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_short_heavy_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.ScriptDisplayName.html#method.new_short_heavy_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_heavy_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.VariantDisplayName.html#method.new_heavy_with_fallback) for more information.
        */
        fun forVariantHeavy(locale: Locale, variant: String): String {
            val variantSliceMemory = PrimitiveArrayTools.borrowUtf8(variant)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_variant_heavy_mv1(locale.handle, variantSliceMemory.slice, write);
            try {
                
                val returnString = DW.writeToString(write)
                return returnString
            } finally {
                variantSliceMemory.close()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `new_heavy_with_fallback`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.VariantDisplayName.html#method.new_heavy_with_fallback) for more information.
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
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_light) for more information.
        */
        fun forLanguageIdentifierLight(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_light_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_light) for more information.
        */
        fun forLanguageIdentifierLightWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_light_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forLanguageIdentifierTiny(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_tiny_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_tiny) for more information.
        */
        fun forLanguageIdentifierTinyWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_tiny_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_light) for more information.
        */
        fun forLanguageIdentifierShortLight(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_light_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_light) for more information.
        */
        fun forLanguageIdentifierShortLightWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_light_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_long_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_light) for more information.
        */
        fun forLanguageIdentifierLongLight(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_light_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_long_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_light) for more information.
        */
        fun forLanguageIdentifierLongLightWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_light_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_menu_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_light) for more information.
        */
        fun forLanguageIdentifierMenuLight(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_menu_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_light) for more information.
        */
        fun forLanguageIdentifierMenuLightWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_light_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_menu_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_light) for more information.
        */
        fun forLanguageIdentifierShortMenuLight(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_menu_light`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_light) for more information.
        */
        fun forLanguageIdentifierShortMenuLightWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_light_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forLanguageIdentifierHeavy(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_heavy_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_heavy) for more information.
        */
        fun forLanguageIdentifierHeavyWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_heavy_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_heavy) for more information.
        */
        fun forLanguageIdentifierShortHeavy(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_heavy) for more information.
        */
        fun forLanguageIdentifierShortHeavyWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_heavy_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_long_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_heavy) for more information.
        */
        fun forLanguageIdentifierLongHeavy(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_long_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_heavy) for more information.
        */
        fun forLanguageIdentifierLongHeavyWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_long_heavy_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_menu_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_heavy) for more information.
        */
        fun forLanguageIdentifierMenuHeavy(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_menu_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_heavy) for more information.
        */
        fun forLanguageIdentifierMenuHeavyWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_menu_heavy_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_menu_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_heavy) for more information.
        */
        fun forLanguageIdentifierShortMenuHeavy(locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_mv1(locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *See the [Rust documentation for `try_new_short_menu_heavy`](https://docs.rs/icu/2.3.1/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_heavy) for more information.
        */
        fun forLanguageIdentifierShortMenuHeavyWithProvider(provider: DataProvider, locale: Locale, langid: Locale, languageDisplay: LanguageDisplayUnstable): Result<String> {
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_language_identifier_short_menu_heavy_with_provider_mv1(provider.handle, locale.handle, langid.handle, languageDisplay.toNative(), write);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }

}
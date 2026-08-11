package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleNamesUnstableLib: Library {
    fun icu4x_LocaleNamesUnstable_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1(locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
    fun icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1(provider: Pointer, locale: Pointer, region: Slice, write: Pointer): ResultUnitInt
}

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
        
        fun forRegionWithCompiledDataLight(locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1(locale.handle, regionSliceMemory.slice, write);
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
        
        fun forRegionWithProviderLight(provider: DataProvider, locale: Locale, region: String): Result<String> {
            val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1(provider.handle, locale.handle, regionSliceMemory.slice, write);
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
    }

}
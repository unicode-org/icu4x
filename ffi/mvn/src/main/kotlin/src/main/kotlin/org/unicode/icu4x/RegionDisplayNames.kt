package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RegionDisplayNamesLib: Library {
    fun icu4x_RegionDisplayNames_destroy_mv1(handle: Pointer)
    fun icu4x_RegionDisplayNames_create_v1_mv1(locale: Pointer, options: DisplayNamesOptionsNative): ResultPointerInt
    fun icu4x_RegionDisplayNames_create_v1_with_provider_mv1(provider: Pointer, locale: Pointer, options: DisplayNamesOptionsNative): ResultPointerInt
    fun icu4x_RegionDisplayNames_of_mv1(handle: Pointer, region: Slice, write: Pointer): ResultUnitInt
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `RegionDisplayNames`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.RegionDisplayNames.html) for more information.
*/
class RegionDisplayNames internal constructor (
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

    private class RegionDisplayNamesCleaner(val handle: Pointer, val lib: RegionDisplayNamesLib) : Runnable {
        override fun run() {
            lib.icu4x_RegionDisplayNames_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RegionDisplayNames.RegionDisplayNamesCleaner(handle, RegionDisplayNames.lib));
    }

    companion object {
        internal val libClass: Class<RegionDisplayNamesLib> = RegionDisplayNamesLib::class.java
        internal val lib: RegionDisplayNamesLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *Creates a new `RegionDisplayNames` from locale data and an options bag using compiled data.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.RegionDisplayNames.html#method.try_new) for more information.
        */
        fun create(locale: Locale, options: DisplayNamesOptions): Result<RegionDisplayNames> {
            
            val returnVal = lib.icu4x_RegionDisplayNames_create_v1_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RegionDisplayNames(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *Creates a new `RegionDisplayNames` from locale data and an options bag using a particular data source.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.RegionDisplayNames.html#method.try_new) for more information.
        */
        fun create_with_provider(provider: DataProvider, locale: Locale, options: DisplayNamesOptions): Result<RegionDisplayNames> {
            
            val returnVal = lib.icu4x_RegionDisplayNames_create_v1_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RegionDisplayNames(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** 🚧 This API is unstable and may experience breaking changes outside major releases.
    *
    *Returns the locale specific display name of a region.
    *Note that the function returns an empty string in case the display name for a given
    *region code is not found.
    *
    *See the [Rust documentation for `of`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.RegionDisplayNames.html#method.of) for more information.
    */
    fun of(region: String): Result<String> {
        val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_RegionDisplayNames_of_mv1(handle, regionSliceMemory.slice, write);
        try {
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                
                val returnString = DW.writeToString(write)
                return returnString.ok()
            } else {
                return LocaleParseErrorError(LocaleParseError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        } finally {
            regionSliceMemory.close()
        }
    }

}
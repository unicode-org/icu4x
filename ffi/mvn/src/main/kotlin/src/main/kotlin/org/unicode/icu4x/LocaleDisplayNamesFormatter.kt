package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleDisplayNamesFormatterLib: Library {
    fun icu4x_LocaleDisplayNamesFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleDisplayNamesFormatter_create_v1_mv1(locale: Pointer, options: DisplayNamesOptionsNative): ResultPointerInt
    fun icu4x_LocaleDisplayNamesFormatter_create_v1_with_provider_mv1(provider: Pointer, locale: Pointer, options: DisplayNamesOptionsNative): ResultPointerInt
    fun icu4x_LocaleDisplayNamesFormatter_of_mv1(handle: Pointer, locale: Pointer, write: Pointer): Unit
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `LocaleDisplayNamesFormatter`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.LocaleDisplayNamesFormatter.html) for more information.
*/
class LocaleDisplayNamesFormatter internal constructor (
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

    private class LocaleDisplayNamesFormatterCleaner(val handle: Pointer, val lib: LocaleDisplayNamesFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleDisplayNamesFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, LocaleDisplayNamesFormatter.LocaleDisplayNamesFormatterCleaner(handle, LocaleDisplayNamesFormatter.lib));
    }

    companion object {
        internal val libClass: Class<LocaleDisplayNamesFormatterLib> = LocaleDisplayNamesFormatterLib::class.java
        internal val lib: LocaleDisplayNamesFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *Creates a new `LocaleDisplayNamesFormatter` from locale data and an options bag using compiled data.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.LocaleDisplayNamesFormatter.html#method.try_new) for more information.
        */
        fun create(locale: Locale, options: DisplayNamesOptions): Result<LocaleDisplayNamesFormatter> {
            
            val returnVal = lib.icu4x_LocaleDisplayNamesFormatter_create_v1_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = LocaleDisplayNamesFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** 🚧 This API is unstable and may experience breaking changes outside major releases.
        *
        *Creates a new `LocaleDisplayNamesFormatter` from locale data and an options bag using a particular data source.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.LocaleDisplayNamesFormatter.html#method.try_new) for more information.
        */
        fun create_with_provider(provider: DataProvider, locale: Locale, options: DisplayNamesOptions): Result<LocaleDisplayNamesFormatter> {
            
            val returnVal = lib.icu4x_LocaleDisplayNamesFormatter_create_v1_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = LocaleDisplayNamesFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** 🚧 This API is unstable and may experience breaking changes outside major releases.
    *
    *Returns the locale-specific display name of a locale.
    *🚧 This API is unstable and may experience breaking changes outside major releases.
    *
    *See the [Rust documentation for `of`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.LocaleDisplayNamesFormatter.html#method.of) for more information.
    */
    fun of(locale: Locale): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_LocaleDisplayNamesFormatter_of_mv1(handle, locale.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
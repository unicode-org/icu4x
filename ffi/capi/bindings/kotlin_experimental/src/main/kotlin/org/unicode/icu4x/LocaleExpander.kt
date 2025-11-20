package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleExpanderLib: Library {
    fun icu4x_LocaleExpander_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleExpander_create_common_mv1(): Pointer
    fun icu4x_LocaleExpander_create_common_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleExpander_create_extended_mv1(): Pointer
    fun icu4x_LocaleExpander_create_extended_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleExpander_maximize_mv1(handle: Pointer, locale: Pointer): Int
    fun icu4x_LocaleExpander_minimize_mv1(handle: Pointer, locale: Pointer): Int
    fun icu4x_LocaleExpander_minimize_favor_script_mv1(handle: Pointer, locale: Pointer): Int
}
/** A locale expander.
*
*See the [Rust documentation for `LocaleExpander`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html) for more information.
*/
class LocaleExpander internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LocaleExpanderCleaner(val handle: Pointer, val lib: LocaleExpanderLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleExpander_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleExpanderLib> = LocaleExpanderLib::class.java
        internal val lib: LocaleExpanderLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a new [LocaleExpander] using compiled data.
        *
        *See the [Rust documentation for `new_common`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.new_common) for more information.
        */
        fun createCommon(): LocaleExpander {
            
            val returnVal = lib.icu4x_LocaleExpander_create_common_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleExpander(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleExpander.LocaleExpanderCleaner(handle, LocaleExpander.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [LocaleExpander] using a new_common data source.
        *
        *See the [Rust documentation for `new_common`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.new_common) for more information.
        */
        fun createCommonWithProvider(provider: DataProvider): Result<LocaleExpander> {
            
            val returnVal = lib.icu4x_LocaleExpander_create_common_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleExpander(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleExpander.LocaleExpanderCleaner(handle, LocaleExpander.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a new [LocaleExpander] with extended data using compiled data.
        *
        *See the [Rust documentation for `new_extended`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.new_extended) for more information.
        */
        fun createExtended(): LocaleExpander {
            
            val returnVal = lib.icu4x_LocaleExpander_create_extended_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleExpander(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleExpander.LocaleExpanderCleaner(handle, LocaleExpander.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [LocaleExpander] with extended data using a particular data source.
        *
        *See the [Rust documentation for `new_extended`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.new_extended) for more information.
        */
        fun createExtendedWithProvider(provider: DataProvider): Result<LocaleExpander> {
            
            val returnVal = lib.icu4x_LocaleExpander_create_extended_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleExpander(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleExpander.LocaleExpanderCleaner(handle, LocaleExpander.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `maximize`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.maximize) for more information.
    */
    fun maximize(locale: Locale): TransformResult {
        
        val returnVal = lib.icu4x_LocaleExpander_maximize_mv1(handle, locale.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        return (TransformResult.fromNative(returnVal))
    }
    
    /** See the [Rust documentation for `minimize`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.minimize) for more information.
    */
    fun minimize(locale: Locale): TransformResult {
        
        val returnVal = lib.icu4x_LocaleExpander_minimize_mv1(handle, locale.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        return (TransformResult.fromNative(returnVal))
    }
    
    /** See the [Rust documentation for `minimize_favor_script`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleExpander.html#method.minimize_favor_script) for more information.
    */
    fun minimizeFavorScript(locale: Locale): TransformResult {
        
        val returnVal = lib.icu4x_LocaleExpander_minimize_favor_script_mv1(handle, locale.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        return (TransformResult.fromNative(returnVal))
    }

}
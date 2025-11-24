package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleCanonicalizerLib: Library {
    fun icu4x_LocaleCanonicalizer_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleCanonicalizer_create_common_mv1(): Pointer
    fun icu4x_LocaleCanonicalizer_create_common_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleCanonicalizer_create_extended_mv1(): Pointer
    fun icu4x_LocaleCanonicalizer_create_extended_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleCanonicalizer_canonicalize_mv1(handle: Pointer, locale: Pointer): Int
}
/** A locale canonicalizer.
*
*See the [Rust documentation for `LocaleCanonicalizer`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleCanonicalizer.html) for more information.
*/
class LocaleCanonicalizer internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LocaleCanonicalizerCleaner(val handle: Pointer, val lib: LocaleCanonicalizerLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleCanonicalizer_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleCanonicalizerLib> = LocaleCanonicalizerLib::class.java
        internal val lib: LocaleCanonicalizerLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a new [LocaleCanonicalizer] using compiled data.
        *
        *See the [Rust documentation for `new_common`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleCanonicalizer.html#method.new_common) for more information.
        */
        fun createCommon(): LocaleCanonicalizer {
            
            val returnVal = lib.icu4x_LocaleCanonicalizer_create_common_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleCanonicalizer(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleCanonicalizer.LocaleCanonicalizerCleaner(handle, LocaleCanonicalizer.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [LocaleCanonicalizer].
        *
        *See the [Rust documentation for `new_common`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleCanonicalizer.html#method.new_common) for more information.
        */
        fun createCommonWithProvider(provider: DataProvider): Result<LocaleCanonicalizer> {
            
            val returnVal = lib.icu4x_LocaleCanonicalizer_create_common_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleCanonicalizer(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleCanonicalizer.LocaleCanonicalizerCleaner(handle, LocaleCanonicalizer.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create a new [LocaleCanonicalizer] with extended data using compiled data.
        *
        *See the [Rust documentation for `new_extended`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleCanonicalizer.html#method.new_extended) for more information.
        */
        fun createExtended(): LocaleCanonicalizer {
            
            val returnVal = lib.icu4x_LocaleCanonicalizer_create_extended_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleCanonicalizer(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleCanonicalizer.LocaleCanonicalizerCleaner(handle, LocaleCanonicalizer.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [LocaleCanonicalizer] with extended data.
        *
        *See the [Rust documentation for `new_extended`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleCanonicalizer.html#method.new_extended) for more information.
        */
        fun createExtendedWithProvider(provider: DataProvider): Result<LocaleCanonicalizer> {
            
            val returnVal = lib.icu4x_LocaleCanonicalizer_create_extended_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleCanonicalizer(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleCanonicalizer.LocaleCanonicalizerCleaner(handle, LocaleCanonicalizer.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `canonicalize`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleCanonicalizer.html#method.canonicalize) for more information.
    */
    fun canonicalize(locale: Locale): TransformResult {
        
        val returnVal = lib.icu4x_LocaleCanonicalizer_canonicalize_mv1(handle, locale.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        return (TransformResult.fromNative(returnVal))
    }

}
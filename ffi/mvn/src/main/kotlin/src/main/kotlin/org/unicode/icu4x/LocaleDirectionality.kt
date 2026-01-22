package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleDirectionalityLib: Library {
    fun icu4x_LocaleDirectionality_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleDirectionality_create_common_mv1(): Pointer
    fun icu4x_LocaleDirectionality_create_common_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleDirectionality_create_extended_mv1(): Pointer
    fun icu4x_LocaleDirectionality_create_extended_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleDirectionality_get_mv1(handle: Pointer, locale: Pointer): Int
    fun icu4x_LocaleDirectionality_is_left_to_right_mv1(handle: Pointer, locale: Pointer): Byte
    fun icu4x_LocaleDirectionality_is_right_to_left_mv1(handle: Pointer, locale: Pointer): Byte
}
/** See the [Rust documentation for `LocaleDirectionality`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html) for more information.
*/
class LocaleDirectionality internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LocaleDirectionalityCleaner(val handle: Pointer, val lib: LocaleDirectionalityLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleDirectionality_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleDirectionalityLib> = LocaleDirectionalityLib::class.java
        internal val lib: LocaleDirectionalityLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `LocaleDirectionality` instance using compiled data.
        *
        *See the [Rust documentation for `new_common`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.new_common) for more information.
        */
        fun createCommon(): LocaleDirectionality {
            
            val returnVal = lib.icu4x_LocaleDirectionality_create_common_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleDirectionality(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleDirectionality.LocaleDirectionalityCleaner(handle, LocaleDirectionality.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `LocaleDirectionality` instance using a particular data source.
        *
        *See the [Rust documentation for `new_common`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.new_common) for more information.
        */
        fun createCommonWithProvider(provider: DataProvider): Result<LocaleDirectionality> {
            
            val returnVal = lib.icu4x_LocaleDirectionality_create_common_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleDirectionality(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleDirectionality.LocaleDirectionalityCleaner(handle, LocaleDirectionality.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `LocaleDirectionality` instance using compiled data.
        *
        *See the [Rust documentation for `new_extended`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.new_extended) for more information.
        */
        fun createExtended(): LocaleDirectionality {
            
            val returnVal = lib.icu4x_LocaleDirectionality_create_extended_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleDirectionality(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleDirectionality.LocaleDirectionalityCleaner(handle, LocaleDirectionality.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `LocaleDirectionality` instance using a particular data source.
        *
        *See the [Rust documentation for `new_extended`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.new_extended) for more information.
        */
        fun createExtendedWithProvider(provider: DataProvider): Result<LocaleDirectionality> {
            
            val returnVal = lib.icu4x_LocaleDirectionality_create_extended_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleDirectionality(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleDirectionality.LocaleDirectionalityCleaner(handle, LocaleDirectionality.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.get) for more information.
    */
    fun get(locale: Locale): LocaleDirection {
        
        val returnVal = lib.icu4x_LocaleDirectionality_get_mv1(handle, locale.handle);
        return (LocaleDirection.fromNative(returnVal))
    }
    
    /** See the [Rust documentation for `is_left_to_right`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.is_left_to_right) for more information.
    */
    fun isLeftToRight(locale: Locale): Boolean {
        
        val returnVal = lib.icu4x_LocaleDirectionality_is_left_to_right_mv1(handle, locale.handle);
        return (returnVal > 0)
    }
    
    /** See the [Rust documentation for `is_right_to_left`](https://docs.rs/icu/2.1.1/icu/locale/struct.LocaleDirectionality.html#method.is_right_to_left) for more information.
    */
    fun isRightToLeft(locale: Locale): Boolean {
        
        val returnVal = lib.icu4x_LocaleDirectionality_is_right_to_left_mv1(handle, locale.handle);
        return (returnVal > 0)
    }

}
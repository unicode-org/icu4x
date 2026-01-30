package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PluralRulesWithRangesLib: Library {
    fun icu4x_PluralRulesWithRanges_destroy_mv1(handle: Pointer)
    fun icu4x_PluralRulesWithRanges_create_cardinal_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_PluralRulesWithRanges_create_cardinal_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_PluralRulesWithRanges_create_ordinal_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_PluralRulesWithRanges_create_ordinal_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_PluralRulesWithRanges_category_for_range_mv1(handle: Pointer, start: Pointer, end: Pointer): Int
}
/** See the [Rust documentation for `PluralRulesWithRanges`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRulesWithRanges.html) for more information.
*/
class PluralRulesWithRanges internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class PluralRulesWithRangesCleaner(val handle: Pointer, val lib: PluralRulesWithRangesLib) : Runnable {
        override fun run() {
            lib.icu4x_PluralRulesWithRanges_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<PluralRulesWithRangesLib> = PluralRulesWithRangesLib::class.java
        internal val lib: PluralRulesWithRangesLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** construct a [PluralRulesWithRanges] for the given locale, for cardinal numbers, using compiled data.
        *
        *See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_cardinal) for more information.
        */
        fun createCardinal(locale: Locale): Result<PluralRulesWithRanges> {
            
            val returnVal = lib.icu4x_PluralRulesWithRanges_create_cardinal_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRulesWithRanges(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRulesWithRanges.PluralRulesWithRangesCleaner(handle, PluralRulesWithRanges.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** construct a [PluralRulesWithRanges] for the given locale, for cardinal numbers, using a particular data source.
        *
        *See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_cardinal) for more information.
        */
        fun createCardinalWithProvider(provider: DataProvider, locale: Locale): Result<PluralRulesWithRanges> {
            
            val returnVal = lib.icu4x_PluralRulesWithRanges_create_cardinal_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRulesWithRanges(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRulesWithRanges.PluralRulesWithRangesCleaner(handle, PluralRulesWithRanges.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [PluralRulesWithRanges] for the given locale, for ordinal numbers, using compiled data.
        *
        *See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_ordinal) for more information.
        */
        fun createOrdinal(locale: Locale): Result<PluralRulesWithRanges> {
            
            val returnVal = lib.icu4x_PluralRulesWithRanges_create_ordinal_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRulesWithRanges(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRulesWithRanges.PluralRulesWithRangesCleaner(handle, PluralRulesWithRanges.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a [PluralRulesWithRanges] for the given locale, for ordinal numbers, using a particular data source.
        *
        *See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_ordinal) for more information.
        */
        fun createOrdinalWithProvider(provider: DataProvider, locale: Locale): Result<PluralRulesWithRanges> {
            
            val returnVal = lib.icu4x_PluralRulesWithRanges_create_ordinal_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRulesWithRanges(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRulesWithRanges.PluralRulesWithRangesCleaner(handle, PluralRulesWithRanges.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Get the category for a given number represented as operands
    *
    *See the [Rust documentation for `category_for_range`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for_range) for more information.
    */
    fun categoryForRange(start: PluralOperands, end: PluralOperands): PluralCategory {
        
        val returnVal = lib.icu4x_PluralRulesWithRanges_category_for_range_mv1(handle, start.handle, end.handle);
        return (PluralCategory.fromNative(returnVal))
    }

}
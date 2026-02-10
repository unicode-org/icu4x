package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PluralRulesLib: Library {
    fun icu4x_PluralRules_destroy_mv1(handle: Pointer)
    fun icu4x_PluralRules_create_cardinal_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_PluralRules_create_cardinal_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_PluralRules_create_ordinal_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_PluralRules_create_ordinal_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_PluralRules_category_for_mv1(handle: Pointer, op: Pointer): Int
    fun icu4x_PluralRules_categories_mv1(handle: Pointer): PluralCategoriesNative
}
/** See the [Rust documentation for `PluralRules`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html) for more information.
*/
class PluralRules internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class PluralRulesCleaner(val handle: Pointer, val lib: PluralRulesLib) : Runnable {
        override fun run() {
            lib.icu4x_PluralRules_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<PluralRulesLib> = PluralRulesLib::class.java
        internal val lib: PluralRulesLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct an [PluralRules] for the given locale, for cardinal numbers, using compiled data.
        *
        *See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.try_new_cardinal) for more information.
        */
        fun createCardinal(locale: Locale): Result<PluralRules> {
            
            val returnVal = lib.icu4x_PluralRules_create_cardinal_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRules(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRules.PluralRulesCleaner(handle, PluralRules.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct an [PluralRules] for the given locale, for cardinal numbers, using a particular data source.
        *
        *See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.try_new_cardinal) for more information.
        */
        fun createCardinalWithProvider(provider: DataProvider, locale: Locale): Result<PluralRules> {
            
            val returnVal = lib.icu4x_PluralRules_create_cardinal_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRules(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRules.PluralRulesCleaner(handle, PluralRules.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct an [PluralRules] for the given locale, for ordinal numbers, using compiled data.
        *
        *See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.try_new_ordinal) for more information.
        */
        fun createOrdinal(locale: Locale): Result<PluralRules> {
            
            val returnVal = lib.icu4x_PluralRules_create_ordinal_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRules(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRules.PluralRulesCleaner(handle, PluralRules.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct an [PluralRules] for the given locale, for ordinal numbers, using a particular data source.
        *
        *See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.try_new_ordinal) for more information.
        */
        fun createOrdinalWithProvider(provider: DataProvider, locale: Locale): Result<PluralRules> {
            
            val returnVal = lib.icu4x_PluralRules_create_ordinal_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralRules(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralRules.PluralRulesCleaner(handle, PluralRules.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Get the category for a given number represented as operands
    *
    *See the [Rust documentation for `category_for`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.category_for) for more information.
    */
    fun categoryFor(op: PluralOperands): PluralCategory {
        
        val returnVal = lib.icu4x_PluralRules_category_for_mv1(handle, op.handle);
        return (PluralCategory.fromNative(returnVal))
    }
    
    /** Get all of the categories needed in the current locale
    *
    *See the [Rust documentation for `categories`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralRules.html#method.categories) for more information.
    */
    fun categories(): PluralCategories {
        
        val returnVal = lib.icu4x_PluralRules_categories_mv1(handle);
        val returnStruct = PluralCategories.fromNative(returnVal)
        return returnStruct
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ExemplarCharactersLib: Library {
    fun icu4x_ExemplarCharacters_destroy_mv1(handle: Pointer)
    fun icu4x_ExemplarCharacters_contains_str_mv1(handle: Pointer, s: Slice): Byte
    fun icu4x_ExemplarCharacters_contains_mv1(handle: Pointer, cp: Int): Byte
    fun icu4x_ExemplarCharacters_create_main_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_main_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_auxiliary_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_auxiliary_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_punctuation_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_punctuation_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_numbers_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_numbers_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_index_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_ExemplarCharacters_create_index_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
}
/** A set of "exemplar characters" for a given locale.
*
*See the [Rust documentation for `locale`](https://docs.rs/icu/2.1.1/icu/locale/index.html) for more information.
*
*See the [Rust documentation for `ExemplarCharacters`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html) for more information.
*
*See the [Rust documentation for `ExemplarCharactersBorrowed`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharactersBorrowed.html) for more information.
*/
class ExemplarCharacters internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class ExemplarCharactersCleaner(val handle: Pointer, val lib: ExemplarCharactersLib) : Runnable {
        override fun run() {
            lib.icu4x_ExemplarCharacters_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<ExemplarCharactersLib> = ExemplarCharactersLib::class.java
        internal val lib: ExemplarCharactersLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "main" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_main`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_main) for more information.
        */
        fun createMain(locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_main_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "main" set of exemplar characters for a given locale, using a particular data source
        *
        *See the [Rust documentation for `try_new_main`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_main) for more information.
        */
        fun createMainWithProvider(provider: DataProvider, locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_main_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "auxiliary" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_auxiliary`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_auxiliary) for more information.
        */
        fun createAuxiliary(locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_auxiliary_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "auxiliary" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_auxiliary`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_auxiliary) for more information.
        */
        fun createAuxiliaryWithProvider(provider: DataProvider, locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_auxiliary_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "punctuation" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_punctuation`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_punctuation) for more information.
        */
        fun createPunctuation(locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_punctuation_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "punctuation" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_punctuation`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_punctuation) for more information.
        */
        fun createPunctuationWithProvider(provider: DataProvider, locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_punctuation_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "numbers" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_numbers`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_numbers) for more information.
        */
        fun createNumbers(locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_numbers_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "numbers" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_numbers`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_numbers) for more information.
        */
        fun createNumbersWithProvider(provider: DataProvider, locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_numbers_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "index" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_index`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_index) for more information.
        */
        fun createIndex(locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_index_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Create an [ExemplarCharacters] for the "index" set of exemplar characters for a given locale, using compiled data.
        *
        *See the [Rust documentation for `try_new_index`](https://docs.rs/icu/2.1.1/icu/locale/exemplar_chars/struct.ExemplarCharacters.html#method.try_new_index) for more information.
        */
        fun createIndexWithProvider(provider: DataProvider, locale: Locale): Result<ExemplarCharacters> {
            
            val returnVal = lib.icu4x_ExemplarCharacters_create_index_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ExemplarCharacters(handle, selfEdges)
                CLEANER.register(returnOpaque, ExemplarCharacters.ExemplarCharactersCleaner(handle, ExemplarCharacters.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Checks whether the string is in the set.
    *
    *See the [Rust documentation for `contains_str`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvliststringlist/struct.CodePointInversionListAndStringList.html#method.contains_str) for more information.
    */
    fun contains(s: String): Boolean {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_ExemplarCharacters_contains_str_mv1(handle, sSlice);
        return (returnVal > 0)
    }
    
    /** Checks whether the code point is in the set.
    *
    *See the [Rust documentation for `contains`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvliststringlist/struct.CodePointInversionListAndStringList.html#method.contains) for more information.
    */
    fun contains(cp: Int): Boolean {
        
        val returnVal = lib.icu4x_ExemplarCharacters_contains_mv1(handle, cp);
        return (returnVal > 0)
    }

}
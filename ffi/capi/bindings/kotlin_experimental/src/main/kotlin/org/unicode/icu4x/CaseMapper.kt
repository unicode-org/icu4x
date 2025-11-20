package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CaseMapperLib: Library {
    fun icu4x_CaseMapper_destroy_mv1(handle: Pointer)
    fun icu4x_CaseMapper_create_mv1(): Pointer
    fun icu4x_CaseMapper_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CaseMapper_lowercase_mv1(handle: Pointer, s: Slice, locale: Pointer, write: Pointer): Unit
    fun icu4x_CaseMapper_uppercase_mv1(handle: Pointer, s: Slice, locale: Pointer, write: Pointer): Unit
    fun icu4x_CaseMapper_lowercase_with_compiled_data_mv1(s: Slice, locale: Pointer, write: Pointer): Unit
    fun icu4x_CaseMapper_uppercase_with_compiled_data_mv1(s: Slice, locale: Pointer, write: Pointer): Unit
    fun icu4x_CaseMapper_fold_mv1(handle: Pointer, s: Slice, write: Pointer): Unit
    fun icu4x_CaseMapper_fold_turkic_mv1(handle: Pointer, s: Slice, write: Pointer): Unit
    fun icu4x_CaseMapper_add_case_closure_to_mv1(handle: Pointer, c: Int, builder: Pointer): Unit
    fun icu4x_CaseMapper_simple_lowercase_mv1(handle: Pointer, ch: Int): Int
    fun icu4x_CaseMapper_simple_lowercase_with_compiled_data_mv1(ch: Int): Int
    fun icu4x_CaseMapper_simple_uppercase_mv1(handle: Pointer, ch: Int): Int
    fun icu4x_CaseMapper_simple_uppercase_with_compiled_data_mv1(ch: Int): Int
    fun icu4x_CaseMapper_simple_titlecase_mv1(handle: Pointer, ch: Int): Int
    fun icu4x_CaseMapper_simple_titlecase_with_compiled_data_mv1(ch: Int): Int
    fun icu4x_CaseMapper_simple_fold_mv1(handle: Pointer, ch: Int): Int
    fun icu4x_CaseMapper_simple_fold_with_compiled_data_mv1(ch: Int): Int
    fun icu4x_CaseMapper_simple_fold_turkic_mv1(handle: Pointer, ch: Int): Int
    fun icu4x_CaseMapper_simple_fold_turkic_with_compiled_data_mv1(ch: Int): Int
}
/** See the [Rust documentation for `CaseMapper`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapper.html) for more information.
*/
class CaseMapper internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CaseMapperCleaner(val handle: Pointer, val lib: CaseMapperLib) : Runnable {
        override fun run() {
            lib.icu4x_CaseMapper_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CaseMapperLib> = CaseMapperLib::class.java
        internal val lib: CaseMapperLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new CaseMapper instance using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapper.html#method.new) for more information.
        */
        fun create(): CaseMapper {
            
            val returnVal = lib.icu4x_CaseMapper_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CaseMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, CaseMapper.CaseMapperCleaner(handle, CaseMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new CaseMapper instance using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapper.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<CaseMapper> {
            
            val returnVal = lib.icu4x_CaseMapper_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CaseMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, CaseMapper.CaseMapperCleaner(handle, CaseMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Returns the full lowercase mapping of the given string, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `lowercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.lowercase) for more information.
        */
        fun lowercaseWithCompiledData(s: String, locale: Locale): String {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_CaseMapper_lowercase_with_compiled_data_mv1(sSlice, locale.handle, write);
            
            val returnString = DW.writeToString(write)
            return returnString
        }
        @JvmStatic
        
        /** Returns the full uppercase mapping of the given string, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `uppercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.uppercase) for more information.
        */
        fun uppercaseWithCompiledData(s: String, locale: Locale): String {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            val write = DW.lib.diplomat_buffer_write_create(0)
            val returnVal = lib.icu4x_CaseMapper_uppercase_with_compiled_data_mv1(sSlice, locale.handle, write);
            
            val returnString = DW.writeToString(write)
            return returnString
        }
        @JvmStatic
        
        /** Returns the simple lowercase mapping of the given character, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `simple_lowercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_lowercase) for more information.
        */
        fun simpleLowercaseWithCompiledData(ch: Int): Int {
            
            val returnVal = lib.icu4x_CaseMapper_simple_lowercase_with_compiled_data_mv1(ch);
            return (returnVal)
        }
        @JvmStatic
        
        /** Returns the simple uppercase mapping of the given character, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `simple_uppercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_uppercase) for more information.
        */
        fun simpleUppercaseWithCompiledData(ch: Int): Int {
            
            val returnVal = lib.icu4x_CaseMapper_simple_uppercase_with_compiled_data_mv1(ch);
            return (returnVal)
        }
        @JvmStatic
        
        /** Returns the simple titlecase mapping of the given character, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `simple_titlecase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_titlecase) for more information.
        */
        fun simpleTitlecaseWithCompiledData(ch: Int): Int {
            
            val returnVal = lib.icu4x_CaseMapper_simple_titlecase_with_compiled_data_mv1(ch);
            return (returnVal)
        }
        @JvmStatic
        
        /** Returns the simple casefolding of the given character, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `simple_fold`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_fold) for more information.
        */
        fun simpleFoldWithCompiledData(ch: Int): Int {
            
            val returnVal = lib.icu4x_CaseMapper_simple_fold_with_compiled_data_mv1(ch);
            return (returnVal)
        }
        @JvmStatic
        
        /** Returns the simple Turkic casefolding of the given character, using compiled data (avoids having to allocate a CaseMapper object)
        *
        *See the [Rust documentation for `simple_fold_turkic`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_fold_turkic) for more information.
        */
        fun simpleFoldTurkicWithCompiledData(ch: Int): Int {
            
            val returnVal = lib.icu4x_CaseMapper_simple_fold_turkic_with_compiled_data_mv1(ch);
            return (returnVal)
        }
    }
    
    /** Returns the full lowercase mapping of the given string
    *
    *See the [Rust documentation for `lowercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.lowercase) for more information.
    */
    fun lowercase(s: String, locale: Locale): String {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_CaseMapper_lowercase_mv1(handle, sSlice, locale.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Returns the full uppercase mapping of the given string
    *
    *See the [Rust documentation for `uppercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.uppercase) for more information.
    */
    fun uppercase(s: String, locale: Locale): String {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_CaseMapper_uppercase_mv1(handle, sSlice, locale.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Case-folds the characters in the given string
    *
    *See the [Rust documentation for `fold`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.fold) for more information.
    */
    fun fold(s: String): String {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_CaseMapper_fold_mv1(handle, sSlice, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Case-folds the characters in the given string
    *using Turkic (T) mappings for dotted/dotless I.
    *
    *See the [Rust documentation for `fold_turkic`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.fold_turkic) for more information.
    */
    fun foldTurkic(s: String): String {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_CaseMapper_fold_turkic_mv1(handle, sSlice, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Adds all simple case mappings and the full case folding for `c` to `builder`.
    *Also adds special case closure mappings.
    *
    *In other words, this adds all characters that this casemaps to, as
    *well as all characters that may casemap to this one.
    *
    *Note that since CodePointSetBuilder does not contain strings, this will
    *ignore string mappings.
    *
    *Identical to the similarly named method on `CaseMapCloser`, use that if you
    *plan on using string case closure mappings too.
    *
    *See the [Rust documentation for `add_case_closure_to`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.add_case_closure_to) for more information.
    */
    fun addCaseClosureTo(c: Int, builder: CodePointSetBuilder): Unit {
        
        val returnVal = lib.icu4x_CaseMapper_add_case_closure_to_mv1(handle, c, builder.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        
    }
    
    /** Returns the simple lowercase mapping of the given character.
    *
    *This function only implements simple and common mappings.
    *Full mappings, which can map one char to a string, are not included.
    *For full mappings, use `CaseMapperBorrowed::lowercase`.
    *
    *See the [Rust documentation for `simple_lowercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_lowercase) for more information.
    */
    fun simpleLowercase(ch: Int): Int {
        
        val returnVal = lib.icu4x_CaseMapper_simple_lowercase_mv1(handle, ch);
        return (returnVal)
    }
    
    /** Returns the simple uppercase mapping of the given character.
    *
    *This function only implements simple and common mappings.
    *Full mappings, which can map one char to a string, are not included.
    *For full mappings, use `CaseMapperBorrowed::uppercase`.
    *
    *See the [Rust documentation for `simple_uppercase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_uppercase) for more information.
    */
    fun simpleUppercase(ch: Int): Int {
        
        val returnVal = lib.icu4x_CaseMapper_simple_uppercase_mv1(handle, ch);
        return (returnVal)
    }
    
    /** Returns the simple titlecase mapping of the given character.
    *
    *This function only implements simple and common mappings.
    *Full mappings, which can map one char to a string, are not included.
    *For full mappings, use `CaseMapperBorrowed::titlecase_segment`.
    *
    *See the [Rust documentation for `simple_titlecase`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_titlecase) for more information.
    */
    fun simpleTitlecase(ch: Int): Int {
        
        val returnVal = lib.icu4x_CaseMapper_simple_titlecase_mv1(handle, ch);
        return (returnVal)
    }
    
    /** Returns the simple casefolding of the given character.
    *
    *This function only implements simple folding.
    *For full folding, use `CaseMapperBorrowed::fold`.
    *
    *See the [Rust documentation for `simple_fold`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_fold) for more information.
    */
    fun simpleFold(ch: Int): Int {
        
        val returnVal = lib.icu4x_CaseMapper_simple_fold_mv1(handle, ch);
        return (returnVal)
    }
    
    /** Returns the simple casefolding of the given character in the Turkic locale.
    *
    *This function only implements simple folding.
    *For full folding, use `CaseMapperBorrowed::fold_turkic`.
    *
    *See the [Rust documentation for `simple_fold_turkic`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapperBorrowed.html#method.simple_fold_turkic) for more information.
    */
    fun simpleFoldTurkic(ch: Int): Int {
        
        val returnVal = lib.icu4x_CaseMapper_simple_fold_turkic_mv1(handle, ch);
        return (returnVal)
    }

}
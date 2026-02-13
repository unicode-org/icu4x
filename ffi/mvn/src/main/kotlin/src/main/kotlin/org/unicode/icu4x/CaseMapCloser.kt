package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CaseMapCloserLib: Library {
    fun icu4x_CaseMapCloser_destroy_mv1(handle: Pointer)
    fun icu4x_CaseMapCloser_create_mv1(): ResultPointerInt
    fun icu4x_CaseMapCloser_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CaseMapCloser_add_case_closure_to_mv1(handle: Pointer, c: Int, builder: Pointer): Unit
    fun icu4x_CaseMapCloser_add_string_case_closure_to_mv1(handle: Pointer, s: Slice, builder: Pointer): Byte
}
/** See the [Rust documentation for `CaseMapCloser`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapCloser.html) for more information.
*/
class CaseMapCloser internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CaseMapCloserCleaner(val handle: Pointer, val lib: CaseMapCloserLib) : Runnable {
        override fun run() {
            lib.icu4x_CaseMapCloser_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CaseMapCloserLib> = CaseMapCloserLib::class.java
        internal val lib: CaseMapCloserLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `CaseMapCloser` instance using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapCloser.html#method.new) for more information.
        */
        fun create(): Result<CaseMapCloser> {
            
            val returnVal = lib.icu4x_CaseMapCloser_create_mv1();
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CaseMapCloser(handle, selfEdges)
                CLEANER.register(returnOpaque, CaseMapCloser.CaseMapCloserCleaner(handle, CaseMapCloser.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `CaseMapCloser` instance using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapCloser.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<CaseMapCloser> {
            
            val returnVal = lib.icu4x_CaseMapCloser_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CaseMapCloser(handle, selfEdges)
                CLEANER.register(returnOpaque, CaseMapCloser.CaseMapCloserCleaner(handle, CaseMapCloser.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Adds all simple case mappings and the full case folding for `c` to `builder`.
    *Also adds special case closure mappings.
    *
    *See the [Rust documentation for `add_case_closure_to`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapCloserBorrowed.html#method.add_case_closure_to) for more information.
    */
    fun addCaseClosureTo(c: Int, builder: CodePointSetBuilder): Unit {
        
        val returnVal = lib.icu4x_CaseMapCloser_add_case_closure_to_mv1(handle, c, builder.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        
    }
    
    /** Finds all characters and strings which may casemap to `s` as their full case folding string
    *and adds them to the set.
    *
    *Returns true if the string was found
    *
    *See the [Rust documentation for `add_string_case_closure_to`](https://docs.rs/icu/2.1.1/icu/casemap/struct.CaseMapCloserBorrowed.html#method.add_string_case_closure_to) for more information.
    */
    fun addStringCaseClosureTo(s: String, builder: CodePointSetBuilder): Boolean {
        val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_CaseMapCloser_add_string_case_closure_to_mv1(handle, sSliceMemory.slice, builder.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        return (returnVal > 0)
    }

}
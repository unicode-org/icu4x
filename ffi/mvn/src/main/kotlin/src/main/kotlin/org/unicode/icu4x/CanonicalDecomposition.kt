package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CanonicalDecompositionLib: Library {
    fun icu4x_CanonicalDecomposition_destroy_mv1(handle: Pointer)
    fun icu4x_CanonicalDecomposition_create_mv1(): Pointer
    fun icu4x_CanonicalDecomposition_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CanonicalDecomposition_decompose_mv1(handle: Pointer, c: Int): DecomposedNative
}
/** The raw (non-recursive) canonical decomposition operation.
*
*Callers should generally use `DecomposingNormalizer` unless they specifically need raw composition operations
*
*See the [Rust documentation for `CanonicalDecomposition`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalDecomposition.html) for more information.
*/
class CanonicalDecomposition internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CanonicalDecompositionCleaner(val handle: Pointer, val lib: CanonicalDecompositionLib) : Runnable {
        override fun run() {
            lib.icu4x_CanonicalDecomposition_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CanonicalDecompositionLib> = CanonicalDecompositionLib::class.java
        internal val lib: CanonicalDecompositionLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `CanonicalDecomposition` instance for NFC using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.new) for more information.
        */
        fun create(): CanonicalDecomposition {
            
            val returnVal = lib.icu4x_CanonicalDecomposition_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CanonicalDecomposition(handle, selfEdges)
            CLEANER.register(returnOpaque, CanonicalDecomposition.CanonicalDecompositionCleaner(handle, CanonicalDecomposition.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `CanonicalDecomposition` instance for NFC using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<CanonicalDecomposition> {
            
            val returnVal = lib.icu4x_CanonicalDecomposition_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CanonicalDecomposition(handle, selfEdges)
                CLEANER.register(returnOpaque, CanonicalDecomposition.CanonicalDecompositionCleaner(handle, CanonicalDecomposition.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Performs non-recursive canonical decomposition (including for Hangul).
    *
    *See the [Rust documentation for `decompose`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalDecompositionBorrowed.html#method.decompose) for more information.
    */
    fun decompose(c: Int): Decomposed {
        
        val returnVal = lib.icu4x_CanonicalDecomposition_decompose_mv1(handle, c);
        val returnStruct = Decomposed.fromNative(returnVal)
        return returnStruct
    }

}
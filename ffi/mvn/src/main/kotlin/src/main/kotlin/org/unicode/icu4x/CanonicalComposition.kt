package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CanonicalCompositionLib: Library {
    fun icu4x_CanonicalComposition_destroy_mv1(handle: Pointer)
    fun icu4x_CanonicalComposition_create_mv1(): Pointer
    fun icu4x_CanonicalComposition_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CanonicalComposition_compose_mv1(handle: Pointer, starter: Int, second: Int): Int
}
/** The raw canonical composition operation.
*
*Callers should generally use `ComposingNormalizer` unless they specifically need raw composition operations
*
*See the [Rust documentation for `CanonicalComposition`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalComposition.html) for more information.
*/
class CanonicalComposition internal constructor (
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

    private class CanonicalCompositionCleaner(val handle: Pointer, val lib: CanonicalCompositionLib) : Runnable {
        override fun run() {
            lib.icu4x_CanonicalComposition_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, CanonicalComposition.CanonicalCompositionCleaner(handle, CanonicalComposition.lib));
    }

    companion object {
        internal val libClass: Class<CanonicalCompositionLib> = CanonicalCompositionLib::class.java
        internal val lib: CanonicalCompositionLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `CanonicalComposition` instance for NFC using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalComposition.html#method.new) for more information.
        */
        fun create(): CanonicalComposition {
            
            val returnVal = lib.icu4x_CanonicalComposition_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CanonicalComposition(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `CanonicalComposition` instance for NFC using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalComposition.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<CanonicalComposition> {
            
            val returnVal = lib.icu4x_CanonicalComposition_create_with_provider_mv1(provider.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = CanonicalComposition(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** Performs canonical composition (including Hangul) on a pair of characters
    *or returns NUL if these characters don’t compose. Composition exclusions are taken into account.
    *
    *See the [Rust documentation for `compose`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalCompositionBorrowed.html#method.compose) for more information.
    */
    fun compose(starter: Int, second: Int): Int {
        
        val returnVal = lib.icu4x_CanonicalComposition_compose_mv1(handle, starter, second);
        return (returnVal)
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecomposingNormalizerLib: Library {
    fun icu4x_DecomposingNormalizer_destroy_mv1(handle: Pointer)
    fun icu4x_DecomposingNormalizer_create_nfd_mv1(): Pointer
    fun icu4x_DecomposingNormalizer_create_nfd_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_DecomposingNormalizer_create_nfkd_mv1(): Pointer
    fun icu4x_DecomposingNormalizer_create_nfkd_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_DecomposingNormalizer_normalize_mv1(handle: Pointer, s: Slice, write: Pointer): Unit
    fun icu4x_DecomposingNormalizer_is_normalized_utf16_mv1(handle: Pointer, s: Slice): Byte
    fun icu4x_DecomposingNormalizer_is_normalized_utf16_up_to_mv1(handle: Pointer, s: Slice): FFISizet
}
/** See the [Rust documentation for `DecomposingNormalizer`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizer.html) for more information.
*/
class DecomposingNormalizer internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class DecomposingNormalizerCleaner(val handle: Pointer, val lib: DecomposingNormalizerLib) : Runnable {
        override fun run() {
            lib.icu4x_DecomposingNormalizer_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<DecomposingNormalizerLib> = DecomposingNormalizerLib::class.java
        internal val lib: DecomposingNormalizerLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `DecomposingNormalizer` instance for NFD using compiled data.
        *
        *See the [Rust documentation for `new_nfd`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfd) for more information.
        */
        fun createNfd(): DecomposingNormalizer {
            
            val returnVal = lib.icu4x_DecomposingNormalizer_create_nfd_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = DecomposingNormalizer(handle, selfEdges)
            CLEANER.register(returnOpaque, DecomposingNormalizer.DecomposingNormalizerCleaner(handle, DecomposingNormalizer.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `DecomposingNormalizer` instance for NFD using a particular data source.
        *
        *See the [Rust documentation for `new_nfd`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfd) for more information.
        */
        fun createNfdWithProvider(provider: DataProvider): Result<DecomposingNormalizer> {
            
            val returnVal = lib.icu4x_DecomposingNormalizer_create_nfd_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = DecomposingNormalizer(handle, selfEdges)
                CLEANER.register(returnOpaque, DecomposingNormalizer.DecomposingNormalizerCleaner(handle, DecomposingNormalizer.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `DecomposingNormalizer` instance for NFKD using compiled data.
        *
        *See the [Rust documentation for `new_nfkd`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfkd) for more information.
        */
        fun createNfkd(): DecomposingNormalizer {
            
            val returnVal = lib.icu4x_DecomposingNormalizer_create_nfkd_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = DecomposingNormalizer(handle, selfEdges)
            CLEANER.register(returnOpaque, DecomposingNormalizer.DecomposingNormalizerCleaner(handle, DecomposingNormalizer.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `DecomposingNormalizer` instance for NFKD using a particular data source.
        *
        *See the [Rust documentation for `new_nfkd`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfkd) for more information.
        */
        fun createNfkdWithProvider(provider: DataProvider): Result<DecomposingNormalizer> {
            
            val returnVal = lib.icu4x_DecomposingNormalizer_create_nfkd_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = DecomposingNormalizer(handle, selfEdges)
                CLEANER.register(returnOpaque, DecomposingNormalizer.DecomposingNormalizerCleaner(handle, DecomposingNormalizer.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Normalize a string
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizerBorrowed.html#method.normalize_utf8) for more information.
    */
    fun normalize(s: String): String {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DecomposingNormalizer_normalize_mv1(handle, sSlice, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Check if a string is normalized
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `is_normalized_utf16`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizerBorrowed.html#method.is_normalized_utf16) for more information.
    */
    fun is_normalized(s: String): Boolean {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf16(s)
        
        val returnVal = lib.icu4x_DecomposingNormalizer_is_normalized_utf16_mv1(handle, sSlice);
        return (returnVal > 0)
    }
    
    /** Return the index a slice of potentially-invalid UTF-16 is normalized up to
    *
    *See the [Rust documentation for `split_normalized_utf16`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.DecomposingNormalizerBorrowed.html#method.split_normalized_utf16) for more information.
    */
    fun is_normalized_up_to(s: String): ULong {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf16(s)
        
        val returnVal = lib.icu4x_DecomposingNormalizer_is_normalized_utf16_up_to_mv1(handle, sSlice);
        return (returnVal.toULong())
    }

}
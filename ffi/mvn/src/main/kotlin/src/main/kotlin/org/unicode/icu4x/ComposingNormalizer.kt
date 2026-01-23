package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ComposingNormalizerLib: Library {
    fun icu4x_ComposingNormalizer_destroy_mv1(handle: Pointer)
    fun icu4x_ComposingNormalizer_create_nfc_mv1(): Pointer
    fun icu4x_ComposingNormalizer_create_nfc_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_ComposingNormalizer_create_nfkc_mv1(): Pointer
    fun icu4x_ComposingNormalizer_create_nfkc_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_ComposingNormalizer_normalize_mv1(handle: Pointer, s: Slice, write: Pointer): Unit
    fun icu4x_ComposingNormalizer_is_normalized_utf16_mv1(handle: Pointer, s: Slice): Byte
    fun icu4x_ComposingNormalizer_is_normalized_utf16_up_to_mv1(handle: Pointer, s: Slice): FFISizet
}
/** See the [Rust documentation for `ComposingNormalizer`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizer.html) for more information.
*/
class ComposingNormalizer internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class ComposingNormalizerCleaner(val handle: Pointer, val lib: ComposingNormalizerLib) : Runnable {
        override fun run() {
            lib.icu4x_ComposingNormalizer_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<ComposingNormalizerLib> = ComposingNormalizerLib::class.java
        internal val lib: ComposingNormalizerLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `ComposingNormalizer` instance for NFC using compiled data.
        *
        *See the [Rust documentation for `new_nfc`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfc) for more information.
        */
        fun createNfc(): ComposingNormalizer {
            
            val returnVal = lib.icu4x_ComposingNormalizer_create_nfc_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ComposingNormalizer(handle, selfEdges)
            CLEANER.register(returnOpaque, ComposingNormalizer.ComposingNormalizerCleaner(handle, ComposingNormalizer.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `ComposingNormalizer` instance for NFC using a particular data source.
        *
        *See the [Rust documentation for `new_nfc`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfc) for more information.
        */
        fun createNfcWithProvider(provider: DataProvider): Result<ComposingNormalizer> {
            
            val returnVal = lib.icu4x_ComposingNormalizer_create_nfc_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ComposingNormalizer(handle, selfEdges)
                CLEANER.register(returnOpaque, ComposingNormalizer.ComposingNormalizerCleaner(handle, ComposingNormalizer.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `ComposingNormalizer` instance for NFKC using compiled data.
        *
        *See the [Rust documentation for `new_nfkc`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfkc) for more information.
        */
        fun createNfkc(): ComposingNormalizer {
            
            val returnVal = lib.icu4x_ComposingNormalizer_create_nfkc_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ComposingNormalizer(handle, selfEdges)
            CLEANER.register(returnOpaque, ComposingNormalizer.ComposingNormalizerCleaner(handle, ComposingNormalizer.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new `ComposingNormalizer` instance for NFKC using a particular data source.
        *
        *See the [Rust documentation for `new_nfkc`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfkc) for more information.
        */
        fun createNfkcWithProvider(provider: DataProvider): Result<ComposingNormalizer> {
            
            val returnVal = lib.icu4x_ComposingNormalizer_create_nfkc_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ComposingNormalizer(handle, selfEdges)
                CLEANER.register(returnOpaque, ComposingNormalizer.ComposingNormalizerCleaner(handle, ComposingNormalizer.lib));
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
    *See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizerBorrowed.html#method.normalize_utf8) for more information.
    */
    fun normalize(s: String): String {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_ComposingNormalizer_normalize_mv1(handle, sSlice, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Check if a string is normalized
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `is_normalized_utf16`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizerBorrowed.html#method.is_normalized_utf16) for more information.
    */
    fun is_normalized(s: String): Boolean {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf16(s)
        
        val returnVal = lib.icu4x_ComposingNormalizer_is_normalized_utf16_mv1(handle, sSlice);
        return (returnVal > 0)
    }
    
    /** Return the index a slice of potentially-invalid UTF-16 is normalized up to
    *
    *See the [Rust documentation for `split_normalized_utf16`](https://docs.rs/icu/2.1.1/icu/normalizer/struct.ComposingNormalizerBorrowed.html#method.split_normalized_utf16) for more information.
    */
    fun is_normalized_up_to(s: String): ULong {
        val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf16(s)
        
        val returnVal = lib.icu4x_ComposingNormalizer_is_normalized_utf16_up_to_mv1(handle, sSlice);
        return (returnVal.toULong())
    }

}
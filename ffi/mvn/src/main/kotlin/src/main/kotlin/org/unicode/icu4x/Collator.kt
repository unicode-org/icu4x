package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CollatorLib: Library {
    fun icu4x_Collator_destroy_mv1(handle: Pointer)
    fun icu4x_Collator_create_v1_mv1(locale: Pointer, options: CollatorOptionsNative): ResultPointerInt
    fun icu4x_Collator_create_v1_with_provider_mv1(provider: Pointer, locale: Pointer, options: CollatorOptionsNative): ResultPointerInt
    fun icu4x_Collator_compare_utf16_mv1(handle: Pointer, left: Slice, right: Slice): Byte
    fun icu4x_Collator_resolved_options_v1_mv1(handle: Pointer): CollatorResolvedOptionsNative
}
/** See the [Rust documentation for `Collator`](https://docs.rs/icu/2.1.1/icu/collator/struct.Collator.html) for more information.
*/
class Collator internal constructor (
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

    private class CollatorCleaner(val handle: Pointer, val lib: CollatorLib) : Runnable {
        override fun run() {
            lib.icu4x_Collator_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, Collator.CollatorCleaner(handle, Collator.lib));
    }

    companion object {
        internal val libClass: Class<CollatorLib> = CollatorLib::class.java
        internal val lib: CollatorLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new Collator instance using compiled data.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/collator/struct.Collator.html#method.try_new) for more information.
        */
        fun create(locale: Locale, options: CollatorOptions): Result<Collator> {
            
            val returnVal = lib.icu4x_Collator_create_v1_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = Collator(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new Collator instance using a particular data source.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/collator/struct.Collator.html#method.try_new) for more information.
        */
        fun create_with_provider(provider: DataProvider, locale: Locale, options: CollatorOptions): Result<Collator> {
            
            val returnVal = lib.icu4x_Collator_create_v1_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = Collator(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** Compare two strings.
    *
    *Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
    *to the WHATWG Encoding Standard.
    *
    *See the [Rust documentation for `compare_utf16`](https://docs.rs/icu/2.1.1/icu/collator/struct.CollatorBorrowed.html#method.compare_utf16) for more information.
    */
    fun compare(left: String, right: String): Byte {
        val leftSliceMemory = PrimitiveArrayTools.borrowUtf16(left)
        val rightSliceMemory = PrimitiveArrayTools.borrowUtf16(right)
        
        val returnVal = lib.icu4x_Collator_compare_utf16_mv1(handle, leftSliceMemory.slice, rightSliceMemory.slice);
        try {
            return (returnVal)
        } finally {
            leftSliceMemory.close()
            rightSliceMemory.close()
        }
    }
    
    /** The resolved options showing how the default options, the requested options,
    *and the options from locale data were combined. None of the struct fields
    *will have `Auto` as the value.
    *
    *See the [Rust documentation for `resolved_options`](https://docs.rs/icu/2.1.1/icu/collator/struct.CollatorBorrowed.html#method.resolved_options) for more information.
    */
    fun resolved_options(): CollatorResolvedOptions {
        
        val returnVal = lib.icu4x_Collator_resolved_options_v1_mv1(handle);
        val returnStruct = CollatorResolvedOptions.fromNative(returnVal)
        return returnStruct
    }

}
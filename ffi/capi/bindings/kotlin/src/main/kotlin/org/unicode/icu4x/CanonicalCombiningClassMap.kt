package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CanonicalCombiningClassMapLib: Library {
    fun icu4x_CanonicalCombiningClassMap_destroy_mv1(handle: Pointer)
    fun icu4x_CanonicalCombiningClassMap_create_mv1(): Pointer
    fun icu4x_CanonicalCombiningClassMap_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_CanonicalCombiningClassMap_get_mv1(handle: Pointer, ch: Int): FFIUint8
}
/** Lookup of the Canonical_Combining_Class Unicode property
*
*See the [Rust documentation for `CanonicalCombiningClassMap`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html) for more information.
*/
class CanonicalCombiningClassMap internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CanonicalCombiningClassMapCleaner(val handle: Pointer, val lib: CanonicalCombiningClassMapLib) : Runnable {
        override fun run() {
            lib.icu4x_CanonicalCombiningClassMap_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CanonicalCombiningClassMapLib> = CanonicalCombiningClassMapLib::class.java
        internal val lib: CanonicalCombiningClassMapLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new CanonicalCombiningClassMap instance for NFC using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html#method.new) for more information.
        */
        fun create(): CanonicalCombiningClassMap {
            
            val returnVal = lib.icu4x_CanonicalCombiningClassMap_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CanonicalCombiningClassMap(handle, selfEdges)
            CLEANER.register(returnOpaque, CanonicalCombiningClassMap.CanonicalCombiningClassMapCleaner(handle, CanonicalCombiningClassMap.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct a new CanonicalCombiningClassMap instance for NFC using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<CanonicalCombiningClassMap> {
            
            val returnVal = lib.icu4x_CanonicalCombiningClassMap_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = CanonicalCombiningClassMap(handle, selfEdges)
                CLEANER.register(returnOpaque, CanonicalCombiningClassMap.CanonicalCombiningClassMapCleaner(handle, CanonicalCombiningClassMap.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/normalizer/properties/struct.CanonicalCombiningClassMapBorrowed.html#method.get) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html)
    */
    fun get(ch: Int): UByte {
        
        val returnVal = lib.icu4x_CanonicalCombiningClassMap_get_mv1(handle, ch);
        return (returnVal.toUByte())
    }

}
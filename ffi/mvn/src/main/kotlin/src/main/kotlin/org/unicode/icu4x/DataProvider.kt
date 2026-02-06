package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DataProviderLib: Library {
    fun icu4x_DataProvider_destroy_mv1(handle: Pointer)
    fun icu4x_DataProvider_from_fs_mv1(path: Slice): ResultPointerInt
    fun icu4x_DataProvider_from_byte_slice_mv1(blob: Slice): ResultPointerInt
    fun icu4x_DataProvider_fork_by_marker_mv1(handle: Pointer, other: Pointer): ResultUnitInt
    fun icu4x_DataProvider_fork_by_locale_mv1(handle: Pointer, other: Pointer): ResultUnitInt
    fun icu4x_DataProvider_enable_locale_fallback_with_mv1(handle: Pointer, fallbacker: Pointer): ResultUnitInt
}
/** An ICU4X data provider, capable of loading ICU4X data keys from some source.
*
*Currently the only source supported is loading from "blob" formatted data from a bytes buffer or the file system.
*
*If you wish to use ICU4X's builtin "compiled data", use the version of the constructors that do not have `_with_provider`
*in their names.
*
*See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/2.1.1/icu_provider/index.html) for more information.
*/
class DataProvider internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class DataProviderCleaner(val handle: Pointer, val lib: DataProviderLib) : Runnable {
        override fun run() {
            lib.icu4x_DataProvider_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<DataProviderLib> = DataProviderLib::class.java
        internal val lib: DataProviderLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Constructs an `FsDataProvider` and returns it as an [DataProvider].
        *Requires the `provider_fs` Cargo feature.
        *Not supported in WASM.
        *
        *See the [Rust documentation for `FsDataProvider`](https://docs.rs/icu_provider_fs/2.1.1/icu_provider_fs/struct.FsDataProvider.html) for more information.
        */
        fun fromFs(path: String): Result<DataProvider> {
            val pathSliceMemory = PrimitiveArrayTools.borrowUtf8(path)
            
            val returnVal = lib.icu4x_DataProvider_from_fs_mv1(pathSliceMemory.slice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = DataProvider(handle, selfEdges)
                CLEANER.register(returnOpaque, DataProvider.DataProviderCleaner(handle, DataProvider.lib));
                pathSliceMemory?.close()
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Constructs a `BlobDataProvider` and returns it as an [DataProvider].
        *
        *See the [Rust documentation for `try_new_from_static_blob`](https://docs.rs/icu_provider_blob/2.1.1/icu_provider_blob/struct.BlobDataProvider.html#method.try_new_from_static_blob) for more information.
        */
        fun fromByteSlice(blob: ByteArray): Result<DataProvider> {
            val blobSliceMemory = PrimitiveArrayTools.borrow(blob)
            
            val returnVal = lib.icu4x_DataProvider_from_byte_slice_mv1(blobSliceMemory.slice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = DataProvider(handle, selfEdges)
                CLEANER.register(returnOpaque, DataProvider.DataProviderCleaner(handle, DataProvider.lib));
                blobSliceMemory?.close()
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Creates a provider that tries the current provider and then, if the current provider
    *doesn't support the data key, another provider `other`.
    *
    *This takes ownership of the `other` provider, leaving an empty provider in its place.
    *
    *See the [Rust documentation for `ForkByMarkerProvider`](https://docs.rs/icu_provider_adapters/2.1.1/icu_provider_adapters/fork/type.ForkByMarkerProvider.html) for more information.
    */
    fun forkByMarker(other: DataProvider): Result<Unit> {
        
        val returnVal = lib.icu4x_DataProvider_fork_by_marker_mv1(handle, other.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** Same as `fork_by_key` but forks by locale instead of key.
    *
    *See the [Rust documentation for `IdentifierNotFoundPredicate`](https://docs.rs/icu_provider_adapters/2.1.1/icu_provider_adapters/fork/predicates/struct.IdentifierNotFoundPredicate.html) for more information.
    */
    fun forkByLocale(other: DataProvider): Result<Unit> {
        
        val returnVal = lib.icu4x_DataProvider_fork_by_locale_mv1(handle, other.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
        }
    }
    
    /** See the [Rust documentation for `new`](https://docs.rs/icu_provider_adapters/2.1.1/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.new) for more information.
    *
    *Additional information: [1](https://docs.rs/icu_provider_adapters/2.1.1/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html)
    */
    fun enableLocaleFallbackWith(fallbacker: LocaleFallbacker): Result<Unit> {
        
        val returnVal = lib.icu4x_DataProvider_enable_locale_fallback_with_mv1(handle, fallbacker.handle);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
        }
    }

}
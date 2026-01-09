package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TitlecaseMapperLib: Library {
    fun icu4x_TitlecaseMapper_destroy_mv1(handle: Pointer)
    fun icu4x_TitlecaseMapper_create_mv1(): ResultPointerInt
    fun icu4x_TitlecaseMapper_create_with_provider_mv1(provider: Pointer): ResultPointerInt
}
/** See the [Rust documentation for `TitlecaseMapper`](https://docs.rs/icu/2.1.1/icu/casemap/struct.TitlecaseMapper.html) for more information.
*/
class TitlecaseMapper internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class TitlecaseMapperCleaner(val handle: Pointer, val lib: TitlecaseMapperLib) : Runnable {
        override fun run() {
            lib.icu4x_TitlecaseMapper_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<TitlecaseMapperLib> = TitlecaseMapperLib::class.java
        internal val lib: TitlecaseMapperLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `TitlecaseMapper` instance using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/casemap/struct.TitlecaseMapper.html#method.new) for more information.
        */
        fun create(): Result<TitlecaseMapper> {
            
            val returnVal = lib.icu4x_TitlecaseMapper_create_mv1();
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = TitlecaseMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, TitlecaseMapper.TitlecaseMapperCleaner(handle, TitlecaseMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `TitlecaseMapper` instance using a particular data source.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/casemap/struct.TitlecaseMapper.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<TitlecaseMapper> {
            
            val returnVal = lib.icu4x_TitlecaseMapper_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = TitlecaseMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, TitlecaseMapper.TitlecaseMapperCleaner(handle, TitlecaseMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }

}
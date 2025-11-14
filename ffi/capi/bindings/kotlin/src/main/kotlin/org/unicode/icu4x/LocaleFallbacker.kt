package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleFallbackerLib: Library {
    fun icu4x_LocaleFallbacker_destroy_mv1(handle: Pointer)
    fun icu4x_LocaleFallbacker_create_mv1(): Pointer
    fun icu4x_LocaleFallbacker_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_LocaleFallbacker_without_data_mv1(): Pointer
    fun icu4x_LocaleFallbacker_for_config_mv1(handle: Pointer, config: LocaleFallbackConfigNative): Pointer
}
/** An object that runs the ICU4X locale fallback algorithm.
*
*See the [Rust documentation for `LocaleFallbacker`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html) for more information.
*/
class LocaleFallbacker internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LocaleFallbackerCleaner(val handle: Pointer, val lib: LocaleFallbackerLib) : Runnable {
        override fun run() {
            lib.icu4x_LocaleFallbacker_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LocaleFallbackerLib> = LocaleFallbackerLib::class.java
        internal val lib: LocaleFallbackerLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new `LocaleFallbacker` from compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html#method.new) for more information.
        */
        fun create(): LocaleFallbacker {
            
            val returnVal = lib.icu4x_LocaleFallbacker_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleFallbacker(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleFallbacker.LocaleFallbackerCleaner(handle, LocaleFallbacker.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Creates a new `LocaleFallbacker` from a data provider.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<LocaleFallbacker> {
            
            val returnVal = lib.icu4x_LocaleFallbacker_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = LocaleFallbacker(handle, selfEdges)
                CLEANER.register(returnOpaque, LocaleFallbacker.LocaleFallbackerCleaner(handle, LocaleFallbacker.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new `LocaleFallbacker` without data for limited functionality.
        *
        *See the [Rust documentation for `new_without_data`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html#method.new_without_data) for more information.
        */
        fun withoutData(): LocaleFallbacker {
            
            val returnVal = lib.icu4x_LocaleFallbacker_without_data_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = LocaleFallbacker(handle, selfEdges)
            CLEANER.register(returnOpaque, LocaleFallbacker.LocaleFallbackerCleaner(handle, LocaleFallbacker.lib));
            return returnOpaque
        }
    }
    
    /** Associates this `LocaleFallbacker` with configuration options.
    *
    *See the [Rust documentation for `for_config`](https://docs.rs/icu_locale/2.1.1/icu_locale/struct.LocaleFallbacker.html#method.for_config) for more information.
    */
    fun forConfig(config: LocaleFallbackConfig): LocaleFallbackerWithConfig {
        
        val returnVal = lib.icu4x_LocaleFallbacker_for_config_mv1(handle, config.nativeStruct);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = LocaleFallbackerWithConfig(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, LocaleFallbackerWithConfig.LocaleFallbackerWithConfigCleaner(handle, LocaleFallbackerWithConfig.lib));
        return returnOpaque
    }

}
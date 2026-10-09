package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PartsSinkLib: Library {
    fun icu4x_PartsSink_destroy_mv1(handle: Pointer)
    fun icu4x_PartsSink_create_mv1(): Pointer
    fun icu4x_PartsSink_create_with_capacity_mv1(capacity: FFISizet): Pointer
    fun icu4x_PartsSink_drain_mv1(handle: Pointer): Pointer
}
/**
 * A mutable sink that collects [PartSpan]s produced during `*_to_parts` formatting methods,
 * and can be drained into an immutable [Parts] collection via [PartsSink::drain].
 *
 * See the [Rust documentation for `PartsWrite`](https://docs.rs/writeable/0.6.4/writeable/trait.PartsWrite.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class PartsSink internal constructor (
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

    private class PartsSinkCleaner(val handle: Pointer, val lib: PartsSinkLib) : Runnable {
        override fun run() {
            lib.icu4x_PartsSink_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, PartsSink.PartsSinkCleaner(handle, PartsSink.lib));
    }

    companion object {
        internal val libClass: Class<PartsSinkLib> = PartsSinkLib::class.java
        internal val lib: PartsSinkLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /**
         * Construct a new empty [PartsSink].
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun create(): PartsSink {
            
            val returnVal = lib.icu4x_PartsSink_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PartsSink(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /**
         * Construct a new empty [PartsSink] with pre-allocated capacity.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createWithCapacity(capacity: ULong): PartsSink {
            
            val returnVal = lib.icu4x_PartsSink_create_with_capacity_mv1(FFISizet(capacity));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PartsSink(handle, selfEdges, true)
            return returnOpaque
        }
    }
    
    /**
     * Drains the collected [PartSpan]s into an immutable [Parts] collection without
     * copying the underlying span buffer, leaving `self` empty.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun drain(): Parts {
        
        val returnVal = lib.icu4x_PartsSink_drain_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Parts(handle, selfEdges, true)
        return returnOpaque
    }

}
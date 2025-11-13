package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LoggerLib: Library {
    fun icu4x_Logger_destroy_mv1(handle: Pointer)
    fun icu4x_Logger_init_simple_logger_mv1(): Byte
}
/** An object allowing control over the logging used
*/
class Logger internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class LoggerCleaner(val handle: Pointer, val lib: LoggerLib) : Runnable {
        override fun run() {
            lib.icu4x_Logger_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<LoggerLib> = LoggerLib::class.java
        internal val lib: LoggerLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Initialize the logger using `simple_logger`
        *
        *Requires the `simple_logger` Cargo feature.
        *
        *Returns `false` if there was already a logger set.
        */
        fun initSimpleLogger(): Boolean {
            
            val returnVal = lib.icu4x_Logger_init_simple_logger_mv1();
            return (returnVal > 0)
        }
    }

}
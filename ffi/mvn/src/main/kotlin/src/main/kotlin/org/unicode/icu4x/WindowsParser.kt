package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface WindowsParserLib: Library {
    fun icu4x_WindowsParser_destroy_mv1(handle: Pointer)
    fun icu4x_WindowsParser_create_mv1(): Pointer
    fun icu4x_WindowsParser_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_WindowsParser_parse_mv1(handle: Pointer, value: Slice, region: Slice): Pointer?
}
/** A mapper between Windows time zone identifiers and BCP-47 time zone identifiers.
*
*This mapper supports two-way mapping, but it is optimized for the case of Windows to BCP-47.
*It also supports normalizing and canonicalizing the Windows strings.
*
*See the [Rust documentation for `WindowsParser`](https://docs.rs/icu/2.1.1/icu/time/zone/windows/struct.WindowsParser.html) for more information.
*/
class WindowsParser internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class WindowsParserCleaner(val handle: Pointer, val lib: WindowsParserLib) : Runnable {
        override fun run() {
            lib.icu4x_WindowsParser_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<WindowsParserLib> = WindowsParserLib::class.java
        internal val lib: WindowsParserLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a new [WindowsParser] using compiled data
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/windows/struct.WindowsParser.html#method.new) for more information.
        */
        fun create(): WindowsParser {
            
            val returnVal = lib.icu4x_WindowsParser_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = WindowsParser(handle, selfEdges)
            CLEANER.register(returnOpaque, WindowsParser.WindowsParserCleaner(handle, WindowsParser.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [WindowsParser] using a particular data source
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/windows/struct.WindowsParser.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<WindowsParser> {
            
            val returnVal = lib.icu4x_WindowsParser_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WindowsParser(handle, selfEdges)
                CLEANER.register(returnOpaque, WindowsParser.WindowsParserCleaner(handle, WindowsParser.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `parse`](https://docs.rs/icu/2.1.1/icu/time/zone/windows/struct.WindowsParserBorrowed.html#method.parse) for more information.
    */
    fun parse(value: String, region: String): TimeZone? {
        val valueSliceMemory = PrimitiveArrayTools.borrowUtf8(value)
        val regionSliceMemory = PrimitiveArrayTools.borrowUtf8(region)
        
        val returnVal = lib.icu4x_WindowsParser_parse_mv1(handle, valueSliceMemory.slice, regionSliceMemory.slice);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal ?: return null
        val returnOpaque = TimeZone(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZone.TimeZoneCleaner(handle, TimeZone.lib));
        valueSliceMemory?.close()
        regionSliceMemory?.close()
        return returnOpaque
    }

}
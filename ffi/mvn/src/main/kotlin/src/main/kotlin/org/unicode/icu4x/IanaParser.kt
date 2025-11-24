package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface IanaParserLib: Library {
    fun icu4x_IanaParser_destroy_mv1(handle: Pointer)
    fun icu4x_IanaParser_create_mv1(): Pointer
    fun icu4x_IanaParser_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_IanaParser_parse_mv1(handle: Pointer, value: Slice): Pointer
    fun icu4x_IanaParser_iter_mv1(handle: Pointer): Pointer
}
/** A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
*
*This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
*It also supports normalizing and canonicalizing the IANA strings.
*
*See the [Rust documentation for `IanaParser`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParser.html) for more information.
*/
class IanaParser internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class IanaParserCleaner(val handle: Pointer, val lib: IanaParserLib) : Runnable {
        override fun run() {
            lib.icu4x_IanaParser_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<IanaParserLib> = IanaParserLib::class.java
        internal val lib: IanaParserLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a new [IanaParser] using compiled data
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParser.html#method.new) for more information.
        */
        fun create(): IanaParser {
            
            val returnVal = lib.icu4x_IanaParser_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = IanaParser(handle, selfEdges)
            CLEANER.register(returnOpaque, IanaParser.IanaParserCleaner(handle, IanaParser.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [IanaParser] using a particular data source
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParser.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<IanaParser> {
            
            val returnVal = lib.icu4x_IanaParser_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = IanaParser(handle, selfEdges)
                CLEANER.register(returnOpaque, IanaParser.IanaParserCleaner(handle, IanaParser.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `parse`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserBorrowed.html#method.parse) for more information.
    */
    fun parse(value: String): TimeZone {
        val (valueMem, valueSlice) = PrimitiveArrayTools.borrowUtf8(value)
        
        val returnVal = lib.icu4x_IanaParser_parse_mv1(handle, valueSlice);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = TimeZone(handle, selfEdges)
        CLEANER.register(returnOpaque, TimeZone.TimeZoneCleaner(handle, TimeZone.lib));
        if (valueMem != null) valueMem.close()
        return returnOpaque
    }
    
    /** See the [Rust documentation for `iter`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserBorrowed.html#method.iter) for more information.
    */
    fun iter(): TimeZoneIterator {
        
        val returnVal = lib.icu4x_IanaParser_iter_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = TimeZoneIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, TimeZoneIterator.TimeZoneIteratorCleaner(handle, TimeZoneIterator.lib));
        return returnOpaque
    }

}
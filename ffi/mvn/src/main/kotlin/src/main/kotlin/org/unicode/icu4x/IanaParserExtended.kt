package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface IanaParserExtendedLib: Library {
    fun icu4x_IanaParserExtended_destroy_mv1(handle: Pointer)
    fun icu4x_IanaParserExtended_create_mv1(): Pointer
    fun icu4x_IanaParserExtended_create_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_IanaParserExtended_parse_mv1(handle: Pointer, value: Slice): TimeZoneAndCanonicalAndNormalizedNative
    fun icu4x_IanaParserExtended_iter_mv1(handle: Pointer): Pointer
    fun icu4x_IanaParserExtended_iter_all_mv1(handle: Pointer): Pointer
}
/** A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
*
*This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
*It also supports normalizing and canonicalizing the IANA strings.
*
*See the [Rust documentation for `IanaParserExtended`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserExtended.html) for more information.
*/
class IanaParserExtended internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class IanaParserExtendedCleaner(val handle: Pointer, val lib: IanaParserExtendedLib) : Runnable {
        override fun run() {
            lib.icu4x_IanaParserExtended_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<IanaParserExtendedLib> = IanaParserExtendedLib::class.java
        internal val lib: IanaParserExtendedLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a new [IanaParserExtended] using compiled data
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserExtended.html#method.new) for more information.
        */
        fun create(): IanaParserExtended {
            
            val returnVal = lib.icu4x_IanaParserExtended_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = IanaParserExtended(handle, selfEdges)
            CLEANER.register(returnOpaque, IanaParserExtended.IanaParserExtendedCleaner(handle, IanaParserExtended.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a new [IanaParserExtended] using a particular data source
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserExtended.html#method.new) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<IanaParserExtended> {
            
            val returnVal = lib.icu4x_IanaParserExtended_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = IanaParserExtended(handle, selfEdges)
                CLEANER.register(returnOpaque, IanaParserExtended.IanaParserExtendedCleaner(handle, IanaParserExtended.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `parse`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserExtendedBorrowed.html#method.parse) for more information.
    */
    fun parse(value: String): TimeZoneAndCanonicalAndNormalized {
        val (valueMem, valueSlice) = PrimitiveArrayTools.borrowUtf8(value)
        
        val returnVal = lib.icu4x_IanaParserExtended_parse_mv1(handle, valueSlice);
        
        val aEdges: List<Any?> = listOf(this)
        val returnStruct = TimeZoneAndCanonicalAndNormalized.fromNative(returnVal, aEdges)
        if (valueMem != null) valueMem.close()
        return returnStruct
    }
    
    /** See the [Rust documentation for `iter`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserExtendedBorrowed.html#method.iter) for more information.
    */
    fun iter(): TimeZoneAndCanonicalIterator {
        
        val returnVal = lib.icu4x_IanaParserExtended_iter_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = TimeZoneAndCanonicalIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, TimeZoneAndCanonicalIterator.TimeZoneAndCanonicalIteratorCleaner(handle, TimeZoneAndCanonicalIterator.lib));
        return returnOpaque
    }
    
    /** See the [Rust documentation for `iter_all`](https://docs.rs/icu/2.1.1/icu/time/zone/iana/struct.IanaParserExtendedBorrowed.html#method.iter_all) for more information.
    */
    fun iterAll(): TimeZoneAndCanonicalAndNormalizedIterator {
        
        val returnVal = lib.icu4x_IanaParserExtended_iter_all_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val aEdges: List<Any?> = listOf(this)
        val handle = returnVal 
        val returnOpaque = TimeZoneAndCanonicalAndNormalizedIterator(handle, selfEdges, aEdges)
        CLEANER.register(returnOpaque, TimeZoneAndCanonicalAndNormalizedIterator.TimeZoneAndCanonicalAndNormalizedIteratorCleaner(handle, TimeZoneAndCanonicalAndNormalizedIterator.lib));
        return returnOpaque
    }

}
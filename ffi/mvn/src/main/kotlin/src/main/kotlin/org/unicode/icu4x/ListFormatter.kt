package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ListFormatterLib: Library {
    fun icu4x_ListFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_ListFormatter_create_and_with_length_mv1(locale: Pointer, length: Int): ResultPointerInt
    fun icu4x_ListFormatter_create_and_with_length_and_provider_mv1(provider: Pointer, locale: Pointer, length: Int): ResultPointerInt
    fun icu4x_ListFormatter_create_or_with_length_mv1(locale: Pointer, length: Int): ResultPointerInt
    fun icu4x_ListFormatter_create_or_with_length_and_provider_mv1(provider: Pointer, locale: Pointer, length: Int): ResultPointerInt
    fun icu4x_ListFormatter_create_unit_with_length_mv1(locale: Pointer, length: Int): ResultPointerInt
    fun icu4x_ListFormatter_create_unit_with_length_and_provider_mv1(provider: Pointer, locale: Pointer, length: Int): ResultPointerInt
    fun icu4x_ListFormatter_format_utf16_mv1(handle: Pointer, list: Slice, write: Pointer): Unit
}
/** See the [Rust documentation for `ListFormatter`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html) for more information.
*/
class ListFormatter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class ListFormatterCleaner(val handle: Pointer, val lib: ListFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_ListFormatter_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<ListFormatterLib> = ListFormatterLib::class.java
        internal val lib: ListFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct a new `ListFormatter` instance for And patterns from compiled data.
        *
        *See the [Rust documentation for `try_new_and`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.try_new_and) for more information.
        */
        fun createAndWithLength(locale: Locale, length: ListLength): Result<ListFormatter> {
            
            val returnVal = lib.icu4x_ListFormatter_create_and_with_length_mv1(locale.handle, length.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ListFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, ListFormatter.ListFormatterCleaner(handle, ListFormatter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `ListFormatter` instance for And patterns
        *
        *See the [Rust documentation for `try_new_and`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.try_new_and) for more information.
        */
        fun createAndWithLengthAndProvider(provider: DataProvider, locale: Locale, length: ListLength): Result<ListFormatter> {
            
            val returnVal = lib.icu4x_ListFormatter_create_and_with_length_and_provider_mv1(provider.handle, locale.handle, length.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ListFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, ListFormatter.ListFormatterCleaner(handle, ListFormatter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `ListFormatter` instance for And patterns from compiled data.
        *
        *See the [Rust documentation for `try_new_or`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.try_new_or) for more information.
        */
        fun createOrWithLength(locale: Locale, length: ListLength): Result<ListFormatter> {
            
            val returnVal = lib.icu4x_ListFormatter_create_or_with_length_mv1(locale.handle, length.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ListFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, ListFormatter.ListFormatterCleaner(handle, ListFormatter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `ListFormatter` instance for And patterns
        *
        *See the [Rust documentation for `try_new_or`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.try_new_or) for more information.
        */
        fun createOrWithLengthAndProvider(provider: DataProvider, locale: Locale, length: ListLength): Result<ListFormatter> {
            
            val returnVal = lib.icu4x_ListFormatter_create_or_with_length_and_provider_mv1(provider.handle, locale.handle, length.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ListFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, ListFormatter.ListFormatterCleaner(handle, ListFormatter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `ListFormatter` instance for And patterns from compiled data.
        *
        *See the [Rust documentation for `try_new_unit`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.try_new_unit) for more information.
        */
        fun createUnitWithLength(locale: Locale, length: ListLength): Result<ListFormatter> {
            
            val returnVal = lib.icu4x_ListFormatter_create_unit_with_length_mv1(locale.handle, length.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ListFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, ListFormatter.ListFormatterCleaner(handle, ListFormatter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct a new `ListFormatter` instance for And patterns
        *
        *See the [Rust documentation for `try_new_unit`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.try_new_unit) for more information.
        */
        fun createUnitWithLengthAndProvider(provider: DataProvider, locale: Locale, length: ListLength): Result<ListFormatter> {
            
            val returnVal = lib.icu4x_ListFormatter_create_unit_with_length_and_provider_mv1(provider.handle, locale.handle, length.toNative());
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ListFormatter(handle, selfEdges)
                CLEANER.register(returnOpaque, ListFormatter.ListFormatterCleaner(handle, ListFormatter.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/list/struct.ListFormatter.html#method.format) for more information.
    */
    fun format(list: Array<String>): String {
        val listSliceMemory = PrimitiveArrayTools.borrowUtf16s(list)
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_ListFormatter_format_utf16_mv1(handle, listSliceMemory.slice, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
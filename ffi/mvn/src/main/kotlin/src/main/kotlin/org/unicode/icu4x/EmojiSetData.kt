package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface EmojiSetDataLib: Library {
    fun icu4x_EmojiSetData_destroy_mv1(handle: Pointer)
    fun icu4x_EmojiSetData_contains_str_mv1(handle: Pointer, s: Slice): Byte
    fun icu4x_EmojiSetData_contains_mv1(handle: Pointer, cp: Int): Byte
    fun icu4x_EmojiSetData_create_basic_mv1(): Pointer
    fun icu4x_EmojiSetData_create_basic_with_provider_mv1(provider: Pointer): ResultPointerInt
    fun icu4x_EmojiSetData_basic_emoji_for_char_mv1(ch: Int): Byte
    fun icu4x_EmojiSetData_basic_emoji_for_str_mv1(s: Slice): Byte
}
/** An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
*
*See the [Rust documentation for `properties`](https://docs.rs/icu/2.1.1/icu/properties/index.html) for more information.
*
*See the [Rust documentation for `EmojiSetData`](https://docs.rs/icu/2.1.1/icu/properties/struct.EmojiSetData.html) for more information.
*
*See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/properties/struct.EmojiSetData.html#method.new) for more information.
*
*See the [Rust documentation for `EmojiSetDataBorrowed`](https://docs.rs/icu/2.1.1/icu/properties/struct.EmojiSetDataBorrowed.html) for more information.
*/
class EmojiSetData internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class EmojiSetDataCleaner(val handle: Pointer, val lib: EmojiSetDataLib) : Runnable {
        override fun run() {
            lib.icu4x_EmojiSetData_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<EmojiSetDataLib> = EmojiSetDataLib::class.java
        internal val lib: EmojiSetDataLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a map for the `Basic_Emoji` property, using compiled data.
        *
        *See the [Rust documentation for `BasicEmoji`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BasicEmoji.html) for more information.
        */
        fun createBasic(): EmojiSetData {
            
            val returnVal = lib.icu4x_EmojiSetData_create_basic_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = EmojiSetData(handle, selfEdges)
            CLEANER.register(returnOpaque, EmojiSetData.EmojiSetDataCleaner(handle, EmojiSetData.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a map for the `Basic_Emoji` property, using a particular data source.
        *
        *See the [Rust documentation for `BasicEmoji`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BasicEmoji.html) for more information.
        */
        fun createBasicWithProvider(provider: DataProvider): Result<EmojiSetData> {
            
            val returnVal = lib.icu4x_EmojiSetData_create_basic_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = EmojiSetData(handle, selfEdges)
                CLEANER.register(returnOpaque, EmojiSetData.EmojiSetDataCleaner(handle, EmojiSetData.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Get the `Basic_Emoji` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EmojiSet.html#tymethod.for_char) for more information.
        */
        fun basicEmojiForChar(ch: Int): Boolean {
            
            val returnVal = lib.icu4x_EmojiSetData_basic_emoji_for_char_mv1(ch);
            return (returnVal > 0)
        }
        @JvmStatic
        
        /** Get the `Basic_Emoji` value for a given character, using compiled data
        *
        *See the [Rust documentation for `for_str`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EmojiSet.html#tymethod.for_str) for more information.
        */
        fun basicEmojiForStr(s: String): Boolean {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_EmojiSetData_basic_emoji_for_str_mv1(sSliceMemory.slice);
            return (returnVal > 0)
        }
    }
    
    /** Checks whether the string is in the set.
    *
    *See the [Rust documentation for `contains_str`](https://docs.rs/icu/2.1.1/icu/properties/struct.EmojiSetDataBorrowed.html#method.contains_str) for more information.
    */
    fun contains(s: String): Boolean {
        val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
        
        val returnVal = lib.icu4x_EmojiSetData_contains_str_mv1(handle, sSliceMemory.slice);
        return (returnVal > 0)
    }
    
    /** Checks whether the code point is in the set.
    *
    *See the [Rust documentation for `contains`](https://docs.rs/icu/2.1.1/icu/properties/struct.EmojiSetDataBorrowed.html#method.contains) for more information.
    */
    fun contains(cp: Int): Boolean {
        
        val returnVal = lib.icu4x_EmojiSetData_contains_mv1(handle, cp);
        return (returnVal > 0)
    }

}
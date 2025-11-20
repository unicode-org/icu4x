package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface SegmenterWordTypeLib: Library {
    fun icu4x_SegmenterWordType_is_word_like_mv1(inner: Int): Byte
}
/** See the [Rust documentation for `WordType`](https://docs.rs/icu/2.1.1/icu/segmenter/options/enum.WordType.html) for more information.
*/
enum class SegmenterWordType {
    None,
    Number,
    Letter;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<SegmenterWordTypeLib> = SegmenterWordTypeLib::class.java
        internal val lib: SegmenterWordTypeLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): SegmenterWordType {
            return SegmenterWordType.entries[native]
        }

        fun default(): SegmenterWordType {
            return None
        }
    }
    
    /** See the [Rust documentation for `is_word_like`](https://docs.rs/icu/2.1.1/icu/segmenter/options/enum.WordType.html#method.is_word_like) for more information.
    */
    fun isWordLike(): Boolean {
        
        val returnVal = lib.icu4x_SegmenterWordType_is_word_like_mv1(this.toNative());
        return (returnVal > 0)
    }
}
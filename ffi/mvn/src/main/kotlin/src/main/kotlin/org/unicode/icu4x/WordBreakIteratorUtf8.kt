package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface WordBreakIteratorUtf8Lib: Library {
    fun icu4x_WordBreakIteratorUtf8_destroy_mv1(handle: Pointer)
    fun icu4x_WordBreakIteratorUtf8_next_mv1(handle: Pointer): Int
    fun icu4x_WordBreakIteratorUtf8_word_type_mv1(handle: Pointer): Int
    fun icu4x_WordBreakIteratorUtf8_is_word_like_mv1(handle: Pointer): Byte
}
/** See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/2.1.1/icu/segmenter/iterators/struct.WordBreakIterator.html) for more information.
*/
class WordBreakIteratorUtf8 internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val aEdges: List<Any?>,
)  {

    internal class WordBreakIteratorUtf8Cleaner(val handle: Pointer, val lib: WordBreakIteratorUtf8Lib) : Runnable {
        override fun run() {
            lib.icu4x_WordBreakIteratorUtf8_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<WordBreakIteratorUtf8Lib> = WordBreakIteratorUtf8Lib::class.java
        internal val lib: WordBreakIteratorUtf8Lib = Native.load("icu4x", libClass)
    }
    
    /** Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
    *out of range of a 32-bit signed integer.
    *
    *See the [Rust documentation for `next`](https://docs.rs/icu/2.1.1/icu/segmenter/iterators/struct.WordBreakIterator.html#method.next) for more information.
    */
    fun next(): Int {
        
        val returnVal = lib.icu4x_WordBreakIteratorUtf8_next_mv1(handle);
        return (returnVal)
    }
    
    /** Return the status value of break boundary.
    *
    *See the [Rust documentation for `word_type`](https://docs.rs/icu/2.1.1/icu/segmenter/iterators/struct.WordBreakIterator.html#method.word_type) for more information.
    */
    fun wordType(): SegmenterWordType {
        
        val returnVal = lib.icu4x_WordBreakIteratorUtf8_word_type_mv1(handle);
        return (SegmenterWordType.fromNative(returnVal))
    }
    
    /** Return true when break boundary is word-like such as letter/number/CJK
    *
    *See the [Rust documentation for `is_word_like`](https://docs.rs/icu/2.1.1/icu/segmenter/iterators/struct.WordBreakIterator.html#method.is_word_like) for more information.
    */
    fun isWordLike(): Boolean {
        
        val returnVal = lib.icu4x_WordBreakIteratorUtf8_is_word_like_mv1(handle);
        return (returnVal > 0)
    }

}
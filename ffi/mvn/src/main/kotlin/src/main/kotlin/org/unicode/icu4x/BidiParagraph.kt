package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BidiParagraphLib: Library {
    fun icu4x_BidiParagraph_destroy_mv1(handle: Pointer)
    fun icu4x_BidiParagraph_set_paragraph_in_text_mv1(handle: Pointer, n: FFISizet): Byte
    fun icu4x_BidiParagraph_direction_mv1(handle: Pointer): Int
    fun icu4x_BidiParagraph_size_mv1(handle: Pointer): FFISizet
    fun icu4x_BidiParagraph_range_start_mv1(handle: Pointer): FFISizet
    fun icu4x_BidiParagraph_range_end_mv1(handle: Pointer): FFISizet
    fun icu4x_BidiParagraph_reorder_line_mv1(handle: Pointer, rangeStart: FFISizet, rangeEnd: FFISizet, write: Pointer): OptionUnit
    fun icu4x_BidiParagraph_level_at_mv1(handle: Pointer, pos: FFISizet): FFIUint8
}
/** Bidi information for a single processed paragraph
*/
class BidiParagraph internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val infoEdges: List<Any?>,
)  {

    internal class BidiParagraphCleaner(val handle: Pointer, val lib: BidiParagraphLib) : Runnable {
        override fun run() {
            lib.icu4x_BidiParagraph_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<BidiParagraphLib> = BidiParagraphLib::class.java
        internal val lib: BidiParagraphLib = Native.load("icu4x", libClass)
    }
    
    /** Given a paragraph index `n` within the surrounding text, this sets this
    *object to the paragraph at that index. Returns nothing when out of bounds.
    *
    *This is equivalent to calling `paragraph_at()` on `BidiInfo` but doesn't
    *create a new object
    */
    fun setParagraphInText(n: ULong): Boolean {
        
        val returnVal = lib.icu4x_BidiParagraph_set_paragraph_in_text_mv1(handle, FFISizet(n));
        return (returnVal > 0)
    }
    
    /** The primary direction of this paragraph
    *
    *See the [Rust documentation for `level_at`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
    */
    fun direction(): BidiDirection {
        
        val returnVal = lib.icu4x_BidiParagraph_direction_mv1(handle);
        return (BidiDirection.fromNative(returnVal))
    }
    
    /** The number of bytes in this paragraph
    *
    *See the [Rust documentation for `len`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/struct.ParagraphInfo.html#method.len) for more information.
    */
    fun size(): ULong {
        
        val returnVal = lib.icu4x_BidiParagraph_size_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** The start index of this paragraph within the source text
    */
    fun rangeStart(): ULong {
        
        val returnVal = lib.icu4x_BidiParagraph_range_start_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** The end index of this paragraph within the source text
    */
    fun rangeEnd(): ULong {
        
        val returnVal = lib.icu4x_BidiParagraph_range_end_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
    *within this paragraph's range.
    *
    *See the [Rust documentation for `level_at`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
    */
    fun reorderLine(rangeStart: ULong, rangeEnd: ULong): String? {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_BidiParagraph_reorder_line_mv1(handle, FFISizet(rangeStart), FFISizet(rangeEnd), write);
        
        returnVal.option() ?: return null

        val returnString = DW.writeToString(write)
        return returnString
                                
    }
    
    /** Get the BIDI level at a particular byte index in this paragraph.
    *This integer is conceptually a `unicode_bidi::Level`,
    *and can be further inspected using the static methods on Bidi.
    *
    *Returns 0 (equivalent to `Level::ltr()`) on error
    *
    *See the [Rust documentation for `level_at`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
    */
    fun levelAt(pos: ULong): UByte {
        
        val returnVal = lib.icu4x_BidiParagraph_level_at_mv1(handle, FFISizet(pos));
        return (returnVal.toUByte())
    }

}
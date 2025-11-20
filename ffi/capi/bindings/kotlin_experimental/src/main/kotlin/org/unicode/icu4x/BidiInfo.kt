package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BidiInfoLib: Library {
    fun icu4x_BidiInfo_destroy_mv1(handle: Pointer)
    fun icu4x_BidiInfo_paragraph_count_mv1(handle: Pointer): FFISizet
    fun icu4x_BidiInfo_paragraph_at_mv1(handle: Pointer, n: FFISizet): Pointer?
    fun icu4x_BidiInfo_size_mv1(handle: Pointer): FFISizet
    fun icu4x_BidiInfo_level_at_mv1(handle: Pointer, pos: FFISizet): FFIUint8
}
/** An object containing bidi information for a given string, produced by `for_text()` on `Bidi`
*
*See the [Rust documentation for `BidiInfo`](https://docs.rs/unicode_bidi/0.3.11/unicode_bidi/struct.BidiInfo.html) for more information.
*/
class BidiInfo internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal val textEdges: List<Any?>,
)  {

    internal class BidiInfoCleaner(val handle: Pointer, val lib: BidiInfoLib) : Runnable {
        override fun run() {
            lib.icu4x_BidiInfo_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<BidiInfoLib> = BidiInfoLib::class.java
        internal val lib: BidiInfoLib = Native.load("icu4x", libClass)
    }
    
    /** The number of paragraphs contained here
    */
    fun paragraphCount(): ULong {
        
        val returnVal = lib.icu4x_BidiInfo_paragraph_count_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** Get the nth paragraph, returning `None` if out of bounds
    */
    fun paragraphAt(n: ULong): BidiParagraph? {
        
        val returnVal = lib.icu4x_BidiInfo_paragraph_at_mv1(handle, FFISizet(n));
        val selfEdges: List<Any> = listOf()
        val textEdges: List<Any?> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = BidiParagraph(handle, selfEdges, textEdges)
        CLEANER.register(returnOpaque, BidiParagraph.BidiParagraphCleaner(handle, BidiParagraph.lib));
        return returnOpaque
    }
    
    /** The number of bytes in this full text
    */
    fun size(): ULong {
        
        val returnVal = lib.icu4x_BidiInfo_size_mv1(handle);
        return (returnVal.toULong())
    }
    
    /** Get the BIDI level at a particular byte index in the full text.
    *This integer is conceptually a `unicode_bidi::Level`,
    *and can be further inspected using the static methods on Bidi.
    *
    *Returns 0 (equivalent to `Level::ltr()`) on error
    */
    fun levelAt(pos: ULong): UByte {
        
        val returnVal = lib.icu4x_BidiInfo_level_at_mv1(handle, FFISizet(pos));
        return (returnVal.toUByte())
    }

}
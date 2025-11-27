package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CodePointSetBuilderLib: Library {
    fun icu4x_CodePointSetBuilder_destroy_mv1(handle: Pointer)
    fun icu4x_CodePointSetBuilder_create_mv1(): Pointer
    fun icu4x_CodePointSetBuilder_build_mv1(handle: Pointer): Pointer
    fun icu4x_CodePointSetBuilder_complement_mv1(handle: Pointer): Unit
    fun icu4x_CodePointSetBuilder_is_empty_mv1(handle: Pointer): Byte
    fun icu4x_CodePointSetBuilder_add_char_mv1(handle: Pointer, ch: Int): Unit
    fun icu4x_CodePointSetBuilder_add_inclusive_range_mv1(handle: Pointer, start: Int, end: Int): Unit
    fun icu4x_CodePointSetBuilder_add_set_mv1(handle: Pointer, data: Pointer): Unit
    fun icu4x_CodePointSetBuilder_remove_char_mv1(handle: Pointer, ch: Int): Unit
    fun icu4x_CodePointSetBuilder_remove_inclusive_range_mv1(handle: Pointer, start: Int, end: Int): Unit
    fun icu4x_CodePointSetBuilder_remove_set_mv1(handle: Pointer, data: Pointer): Unit
    fun icu4x_CodePointSetBuilder_retain_char_mv1(handle: Pointer, ch: Int): Unit
    fun icu4x_CodePointSetBuilder_retain_inclusive_range_mv1(handle: Pointer, start: Int, end: Int): Unit
    fun icu4x_CodePointSetBuilder_retain_set_mv1(handle: Pointer, data: Pointer): Unit
    fun icu4x_CodePointSetBuilder_complement_char_mv1(handle: Pointer, ch: Int): Unit
    fun icu4x_CodePointSetBuilder_complement_inclusive_range_mv1(handle: Pointer, start: Int, end: Int): Unit
    fun icu4x_CodePointSetBuilder_complement_set_mv1(handle: Pointer, data: Pointer): Unit
}
/** See the [Rust documentation for `CodePointInversionListBuilder`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html) for more information.
*/
class CodePointSetBuilder internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class CodePointSetBuilderCleaner(val handle: Pointer, val lib: CodePointSetBuilderLib) : Runnable {
        override fun run() {
            lib.icu4x_CodePointSetBuilder_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<CodePointSetBuilderLib> = CodePointSetBuilderLib::class.java
        internal val lib: CodePointSetBuilderLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Make a new set builder containing nothing
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.new) for more information.
        */
        fun create(): CodePointSetBuilder {
            
            val returnVal = lib.icu4x_CodePointSetBuilder_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = CodePointSetBuilder(handle, selfEdges)
            CLEANER.register(returnOpaque, CodePointSetBuilder.CodePointSetBuilderCleaner(handle, CodePointSetBuilder.lib));
            return returnOpaque
        }
    }
    
    /** Build this into a set
    *
    *This object is repopulated with an empty builder
    *
    *See the [Rust documentation for `build`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.build) for more information.
    */
    fun build(): CodePointSetData {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_build_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = CodePointSetData(handle, selfEdges)
        CLEANER.register(returnOpaque, CodePointSetData.CodePointSetDataCleaner(handle, CodePointSetData.lib));
        return returnOpaque
    }
    
    /** Complements this set
    *
    *(Elements in this set are removed and vice versa)
    *
    *See the [Rust documentation for `complement`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement) for more information.
    */
    fun complement(): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_complement_mv1(handle);
        
    }
    
    /** Returns whether this set is empty
    *
    *See the [Rust documentation for `is_empty`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.is_empty) for more information.
    */
    fun isEmpty(): Boolean {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_is_empty_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Add a single character to the set
    *
    *See the [Rust documentation for `add_char`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_char) for more information.
    */
    fun addChar(ch: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_add_char_mv1(handle, ch);
        
    }
    
    /** Add an inclusive range of characters to the set
    *
    *See the [Rust documentation for `add_range`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_range) for more information.
    */
    fun addInclusiveRange(start: Int, end: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_add_inclusive_range_mv1(handle, start, end);
        
    }
    
    /** Add all elements that belong to the provided set to the set
    *
    *See the [Rust documentation for `add_set`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_set) for more information.
    */
    fun addSet(data: CodePointSetData): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_add_set_mv1(handle, data.handle);
        
    }
    
    /** Remove a single character to the set
    *
    *See the [Rust documentation for `remove_char`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.remove_char) for more information.
    */
    fun removeChar(ch: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_remove_char_mv1(handle, ch);
        
    }
    
    /** Remove an inclusive range of characters from the set
    *
    *See the [Rust documentation for `remove_range`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.remove_range) for more information.
    */
    fun removeInclusiveRange(start: Int, end: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_remove_inclusive_range_mv1(handle, start, end);
        
    }
    
    /** Remove all elements that belong to the provided set from the set
    *
    *See the [Rust documentation for `remove_set`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.remove_set) for more information.
    */
    fun removeSet(data: CodePointSetData): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_remove_set_mv1(handle, data.handle);
        
    }
    
    /** Removes all elements from the set except a single character
    *
    *See the [Rust documentation for `retain_char`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.retain_char) for more information.
    */
    fun retainChar(ch: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_retain_char_mv1(handle, ch);
        
    }
    
    /** Removes all elements from the set except an inclusive range of characters f
    *
    *See the [Rust documentation for `retain_range`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.retain_range) for more information.
    */
    fun retainInclusiveRange(start: Int, end: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_retain_inclusive_range_mv1(handle, start, end);
        
    }
    
    /** Removes all elements from the set except all elements in the provided set
    *
    *See the [Rust documentation for `retain_set`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.retain_set) for more information.
    */
    fun retainSet(data: CodePointSetData): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_retain_set_mv1(handle, data.handle);
        
    }
    
    /** Complement a single character to the set
    *
    *(Characters which are in this set are removed and vice versa)
    *
    *See the [Rust documentation for `complement_char`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement_char) for more information.
    */
    fun complementChar(ch: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_complement_char_mv1(handle, ch);
        
    }
    
    /** Complement an inclusive range of characters from the set
    *
    *(Characters which are in this set are removed and vice versa)
    *
    *See the [Rust documentation for `complement_range`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement_range) for more information.
    */
    fun complementInclusiveRange(start: Int, end: Int): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_complement_inclusive_range_mv1(handle, start, end);
        
    }
    
    /** Complement all elements that belong to the provided set from the set
    *
    *(Characters which are in this set are removed and vice versa)
    *
    *See the [Rust documentation for `complement_set`](https://docs.rs/icu/2.1.1/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement_set) for more information.
    */
    fun complementSet(data: CodePointSetData): Unit {
        
        val returnVal = lib.icu4x_CodePointSetBuilder_complement_set_mv1(handle, data.handle);
        
    }

}
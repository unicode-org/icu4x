package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface GeneralCategoryNameToGroupMapperLib: Library {
    fun icu4x_GeneralCategoryNameToGroupMapper_destroy_mv1(handle: Pointer)
    fun icu4x_GeneralCategoryNameToGroupMapper_get_strict_mv1(handle: Pointer, name: Slice): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryNameToGroupMapper_get_loose_mv1(handle: Pointer, name: Slice): GeneralCategoryGroupNative
    fun icu4x_GeneralCategoryNameToGroupMapper_create_mv1(): Pointer
    fun icu4x_GeneralCategoryNameToGroupMapper_create_with_provider_mv1(provider: Pointer): ResultPointerInt
}
/** A type capable of looking up General Category Group values from a string name.
*
*See the [Rust documentation for `PropertyParser`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParser.html) for more information.
*
*See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.GeneralCategory.html) for more information.
*/
class GeneralCategoryNameToGroupMapper internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class GeneralCategoryNameToGroupMapperCleaner(val handle: Pointer, val lib: GeneralCategoryNameToGroupMapperLib) : Runnable {
        override fun run() {
            lib.icu4x_GeneralCategoryNameToGroupMapper_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<GeneralCategoryNameToGroupMapperLib> = GeneralCategoryNameToGroupMapperLib::class.java
        internal val lib: GeneralCategoryNameToGroupMapperLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Create a name-to-mask mapper for the `General_Category` property, using compiled data.
        *
        *See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
        */
        fun create(): GeneralCategoryNameToGroupMapper {
            
            val returnVal = lib.icu4x_GeneralCategoryNameToGroupMapper_create_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = GeneralCategoryNameToGroupMapper(handle, selfEdges)
            CLEANER.register(returnOpaque, GeneralCategoryNameToGroupMapper.GeneralCategoryNameToGroupMapperCleaner(handle, GeneralCategoryNameToGroupMapper.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Create a name-to-mask mapper for the `General_Category` property, using a particular data source.
        *
        *See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
        */
        fun createWithProvider(provider: DataProvider): Result<GeneralCategoryNameToGroupMapper> {
            
            val returnVal = lib.icu4x_GeneralCategoryNameToGroupMapper_create_with_provider_mv1(provider.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = GeneralCategoryNameToGroupMapper(handle, selfEdges)
                CLEANER.register(returnOpaque, GeneralCategoryNameToGroupMapper.GeneralCategoryNameToGroupMapperCleaner(handle, GeneralCategoryNameToGroupMapper.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Get the mask value matching the given name, using strict matching
    *
    *Returns 0 if the name is unknown for this property
    *
    *See the [Rust documentation for `get_strict`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParserBorrowed.html#method.get_strict) for more information.
    */
    fun getStrict(name: String): GeneralCategoryGroup {
        val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
        
        val returnVal = lib.icu4x_GeneralCategoryNameToGroupMapper_get_strict_mv1(handle, nameSliceMemory.slice);
        val returnStruct = GeneralCategoryGroup.fromNative(returnVal)
        nameSliceMemory?.close()
        return returnStruct
    }
    
    /** Get the mask value matching the given name, using loose matching
    *
    *Returns 0 if the name is unknown for this property
    *
    *See the [Rust documentation for `get_loose`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyParserBorrowed.html#method.get_loose) for more information.
    */
    fun getLoose(name: String): GeneralCategoryGroup {
        val nameSliceMemory = PrimitiveArrayTools.borrowUtf8(name)
        
        val returnVal = lib.icu4x_GeneralCategoryNameToGroupMapper_get_loose_mv1(handle, nameSliceMemory.slice);
        val returnStruct = GeneralCategoryGroup.fromNative(returnVal)
        nameSliceMemory?.close()
        return returnStruct
    }

}
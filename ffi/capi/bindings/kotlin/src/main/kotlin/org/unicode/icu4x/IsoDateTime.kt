package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface IsoDateTimeLib: Library {
    fun icu4x_IsoDateTime_from_string_mv1(v: Slice): ResultIsoDateTimeNativeInt
}

internal class IsoDateTimeNative: Structure(), Structure.ByValue {
    @JvmField
    internal var date: Pointer = Pointer(0);
    @JvmField
    internal var time: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("date", "time")
    }
}

/** An ICU4X DateTime object capable of containing a ISO-8601 date and time.
*
*See the [Rust documentation for `DateTime`](https://docs.rs/icu/2.1.1/icu/time/struct.DateTime.html) for more information.
*/
class IsoDateTime internal constructor (
    internal val nativeStruct: IsoDateTimeNative) {
    val date: IsoDate = IsoDate(nativeStruct.date, listOf())
    val time: Time = Time(nativeStruct.time, listOf())

    companion object {
        internal val libClass: Class<IsoDateTimeLib> = IsoDateTimeLib::class.java
        internal val lib: IsoDateTimeLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(IsoDateTimeNative::class.java).toLong()
        @JvmStatic
        
        /** Creates a new [IsoDateTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.DateTime.html#method.try_from_str) for more information.
        */
        fun fromString(v: String): Result<IsoDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_IsoDateTime_from_string_mv1(vSlice);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = IsoDateTime(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }

}

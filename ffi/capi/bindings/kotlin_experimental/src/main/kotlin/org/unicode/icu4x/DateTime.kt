package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeLib: Library {
    fun icu4x_DateTime_from_string_mv1(v: Slice, calendar: Pointer): ResultDateTimeNativeInt
}

internal class DateTimeNative: Structure(), Structure.ByValue {
    @JvmField
    internal var date: Pointer = Pointer(0);
    @JvmField
    internal var time: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("date", "time")
    }
}

/** An ICU4X DateTime object capable of containing a date and time for any calendar.
*
*See the [Rust documentation for `DateTime`](https://docs.rs/icu/2.1.1/icu/time/struct.DateTime.html) for more information.
*/
class DateTime internal constructor (
    internal val nativeStruct: DateTimeNative) {
    val date: Date = Date(nativeStruct.date, listOf())
    val time: Time = Time(nativeStruct.time, listOf())

    companion object {
        internal val libClass: Class<DateTimeLib> = DateTimeLib::class.java
        internal val lib: DateTimeLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateTimeNative::class.java).toLong()
        @JvmStatic
        
        /** Creates a new [DateTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.DateTime.html#method.try_from_str) for more information.
        */
        fun fromString(v: String, calendar: Calendar): Result<DateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_DateTime_from_string_mv1(vSlice, calendar.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = DateTime(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }

}

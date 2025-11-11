package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface IsoWeekOfYearLib: Library {
}

internal class IsoWeekOfYearNative: Structure(), Structure.ByValue {
    @JvmField
    internal var weekNumber: FFIUint8 = FFIUint8();
    @JvmField
    internal var isoYear: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("weekNumber", "isoYear")
    }
}

/** See the [Rust documentation for `IsoWeekOfYear`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.IsoWeekOfYear.html) for more information.
*/
class IsoWeekOfYear internal constructor (
    internal val nativeStruct: IsoWeekOfYearNative) {
    val weekNumber: UByte = nativeStruct.weekNumber.toUByte()
    val isoYear: Int = nativeStruct.isoYear

    companion object {
        internal val libClass: Class<IsoWeekOfYearLib> = IsoWeekOfYearLib::class.java
        internal val lib: IsoWeekOfYearLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(IsoWeekOfYearNative::class.java).toLong()
    }

}

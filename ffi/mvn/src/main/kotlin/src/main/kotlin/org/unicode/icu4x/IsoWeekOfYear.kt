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




internal class OptionIsoWeekOfYearNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: IsoWeekOfYearNative = IsoWeekOfYearNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): IsoWeekOfYearNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: IsoWeekOfYearNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: IsoWeekOfYearNative): OptionIsoWeekOfYearNative {
            return OptionIsoWeekOfYearNative(value, 1)
        }

        internal fun none(): OptionIsoWeekOfYearNative {
            return OptionIsoWeekOfYearNative(IsoWeekOfYearNative(), 0)
        }
    }

}

/** See the [Rust documentation for `IsoWeekOfYear`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.IsoWeekOfYear.html) for more information.
*/
class IsoWeekOfYear (var weekNumber: UByte, var isoYear: Int) {
    companion object {

        internal val libClass: Class<IsoWeekOfYearLib> = IsoWeekOfYearLib::class.java
        internal val lib: IsoWeekOfYearLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(IsoWeekOfYearNative::class.java).toLong()

        internal fun fromNative(nativeStruct: IsoWeekOfYearNative): IsoWeekOfYear {
            val weekNumber: UByte = nativeStruct.weekNumber.toUByte()
            val isoYear: Int = nativeStruct.isoYear

            return IsoWeekOfYear(weekNumber, isoYear)
        }

    }
    internal fun toNative(): IsoWeekOfYearNative {
        var native = IsoWeekOfYearNative()
        native.weekNumber = FFIUint8(this.weekNumber)
        native.isoYear = this.isoYear
        return native
    }

}
package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateFieldsLib: Library {
}

internal class DateFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var era: OptionSlice = OptionSlice.none();
    @JvmField
    internal var eraYear: OptionInt = OptionInt.none();
    @JvmField
    internal var extendedYear: OptionInt = OptionInt.none();
    @JvmField
    internal var monthCode: OptionSlice = OptionSlice.none();
    @JvmField
    internal var ordinalMonth: OptionFFIUint8 = OptionFFIUint8.none();
    @JvmField
    internal var day: OptionFFIUint8 = OptionFFIUint8.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("era", "eraYear", "extendedYear", "monthCode", "ordinalMonth", "day")
    }
}




internal class OptionDateFieldsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DateFieldsNative = DateFieldsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DateFieldsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DateFieldsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DateFieldsNative): OptionDateFieldsNative {
            return OptionDateFieldsNative(value, 1)
        }

        internal fun none(): OptionDateFieldsNative {
            return OptionDateFieldsNative(DateFieldsNative(), 0)
        }
    }

}

/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `DateFields`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateFields.html) for more information.
*/
class DateFields (var era: String?, var eraYear: Int?, var extendedYear: Int?, var monthCode: String?, var ordinalMonth: UByte?, var day: UByte?) {
    companion object {

        internal val libClass: Class<DateFieldsLib> = DateFieldsLib::class.java
        internal val lib: DateFieldsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DateFieldsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DateFieldsNative, aEdges: List<Any?>): DateFields {
            val era: String? = nativeStruct.era.option()?.let { PrimitiveArrayTools.getUtf8(it) }
            val eraYear: Int? = nativeStruct.eraYear.option()?.let { it }
            val extendedYear: Int? = nativeStruct.extendedYear.option()?.let { it }
            val monthCode: String? = nativeStruct.monthCode.option()?.let { PrimitiveArrayTools.getUtf8(it) }
            val ordinalMonth: UByte? = nativeStruct.ordinalMonth.option()?.let { it.toUByte() }
            val day: UByte? = nativeStruct.day.option()?.let { it.toUByte() }

            return DateFields(era, eraYear, extendedYear, monthCode, ordinalMonth, day)
        }

    }
    internal fun toNative(aAppendArray: Array<MutableList<Any>>): DateFieldsNative {
        var native = DateFieldsNative()
        native.era = this.era?.let { OptionSlice.some(PrimitiveArrayTools.borrowUtf8(it).into(listOf(*aAppendArray)).slice) } ?: OptionSlice.none()
        native.eraYear = this.eraYear?.let { OptionInt.some(it) } ?: OptionInt.none()
        native.extendedYear = this.extendedYear?.let { OptionInt.some(it) } ?: OptionInt.none()
        native.monthCode = this.monthCode?.let { OptionSlice.some(PrimitiveArrayTools.borrowUtf8(it).into(listOf(*aAppendArray)).slice) } ?: OptionSlice.none()
        native.ordinalMonth = this.ordinalMonth?.let { OptionFFIUint8.some(FFIUint8(it)) } ?: OptionFFIUint8.none()
        native.day = this.day?.let { OptionFFIUint8.some(FFIUint8(it)) } ?: OptionFFIUint8.none()
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}
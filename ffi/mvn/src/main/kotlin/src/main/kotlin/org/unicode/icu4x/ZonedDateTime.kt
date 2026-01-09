package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZonedDateTimeLib: Library {
    fun icu4x_ZonedDateTime_strict_from_string_mv1(v: Slice, calendar: Pointer, ianaParser: Pointer): ResultZonedDateTimeNativeInt
    fun icu4x_ZonedDateTime_full_from_string_mv1(v: Slice, calendar: Pointer, ianaParser: Pointer, offsetCalculator: Pointer): ResultZonedDateTimeNativeInt
    fun icu4x_ZonedDateTime_location_only_from_string_mv1(v: Slice, calendar: Pointer, ianaParser: Pointer): ResultZonedDateTimeNativeInt
    fun icu4x_ZonedDateTime_offset_only_from_string_mv1(v: Slice, calendar: Pointer): ResultZonedDateTimeNativeInt
    fun icu4x_ZonedDateTime_lenient_from_string_mv1(v: Slice, calendar: Pointer, ianaParser: Pointer): ResultZonedDateTimeNativeInt
}

internal class ZonedDateTimeNative: Structure(), Structure.ByValue {
    @JvmField
    internal var date: Pointer = Pointer(0);
    @JvmField
    internal var time: Pointer = Pointer(0);
    @JvmField
    internal var zone: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("date", "time", "zone")
    }
}




internal class OptionZonedDateTimeNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: ZonedDateTimeNative = ZonedDateTimeNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): ZonedDateTimeNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: ZonedDateTimeNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: ZonedDateTimeNative): OptionZonedDateTimeNative {
            return OptionZonedDateTimeNative(value, 1)
        }

        internal fun none(): OptionZonedDateTimeNative {
            return OptionZonedDateTimeNative(ZonedDateTimeNative(), 0)
        }
    }

}

/** An ICU4X DateTime object capable of containing a date, time, and zone for any calendar.
*
*See the [Rust documentation for `ZonedDateTime`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html) for more information.
*/
class ZonedDateTime (var date: Date, var time: Time, var zone: TimeZoneInfo) {
    companion object {

        internal val libClass: Class<ZonedDateTimeLib> = ZonedDateTimeLib::class.java
        internal val lib: ZonedDateTimeLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ZonedDateTimeNative::class.java).toLong()

        internal fun fromNative(nativeStruct: ZonedDateTimeNative): ZonedDateTime {
            val date: Date = Date(nativeStruct.date, listOf())
            val time: Time = Time(nativeStruct.time, listOf())
            val zone: TimeZoneInfo = TimeZoneInfo(nativeStruct.zone, listOf())

            return ZonedDateTime(date, time, zone)
        }

        @JvmStatic
        
        /** Creates a new [ZonedIsoDateTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_strict_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_strict_from_str) for more information.
        */
        fun strictFromString(v: String, calendar: Calendar, ianaParser: IanaParser): Result<ZonedDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedDateTime_strict_from_string_mv1(vSlice, calendar.handle, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedDateTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_full_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_full_from_str) for more information.
        */
        fun fullFromString(v: String, calendar: Calendar, ianaParser: IanaParser, offsetCalculator: VariantOffsetsCalculator): Result<ZonedDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedDateTime_full_from_string_mv1(vSlice, calendar.handle, ianaParser.handle, offsetCalculator.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedDateTime] from a location-only IXDTF string.
        *
        *See the [Rust documentation for `try_location_only_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_location_only_from_str) for more information.
        */
        fun locationOnlyFromString(v: String, calendar: Calendar, ianaParser: IanaParser): Result<ZonedDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedDateTime_location_only_from_string_mv1(vSlice, calendar.handle, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedDateTime] from an offset-only IXDTF string.
        *
        *See the [Rust documentation for `try_offset_only_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_offset_only_from_str) for more information.
        */
        fun offsetOnlyFromString(v: String, calendar: Calendar): Result<ZonedDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedDateTime_offset_only_from_string_mv1(vSlice, calendar.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedDateTime] from an IXDTF string, without requiring the offset.
        *
        *See the [Rust documentation for `try_lenient_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_lenient_from_str) for more information.
        */
        fun lenientFromString(v: String, calendar: Calendar, ianaParser: IanaParser): Result<ZonedDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedDateTime_lenient_from_string_mv1(vSlice, calendar.handle, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }
}
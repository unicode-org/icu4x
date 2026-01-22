package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZonedIsoDateTimeLib: Library {
    fun icu4x_ZonedIsoDateTime_strict_from_string_mv1(v: Slice, ianaParser: Pointer): ResultZonedIsoDateTimeNativeInt
    fun icu4x_ZonedIsoDateTime_full_from_string_mv1(v: Slice, ianaParser: Pointer, offsetCalculator: Pointer): ResultZonedIsoDateTimeNativeInt
    fun icu4x_ZonedIsoDateTime_from_epoch_milliseconds_and_utc_offset_mv1(epochMilliseconds: Long, utcOffset: Pointer): ZonedIsoDateTimeNative
}

internal class ZonedIsoDateTimeNative: Structure(), Structure.ByValue {
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




internal class OptionZonedIsoDateTimeNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: ZonedIsoDateTimeNative = ZonedIsoDateTimeNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): ZonedIsoDateTimeNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: ZonedIsoDateTimeNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: ZonedIsoDateTimeNative): OptionZonedIsoDateTimeNative {
            return OptionZonedIsoDateTimeNative(value, 1)
        }

        internal fun none(): OptionZonedIsoDateTimeNative {
            return OptionZonedIsoDateTimeNative(ZonedIsoDateTimeNative(), 0)
        }
    }

}

/** An ICU4X `ZonedDateTime` object capable of containing a ISO-8601 date, time, and zone.
*
*See the [Rust documentation for `ZonedDateTime`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html) for more information.
*/
class ZonedIsoDateTime (var date: IsoDate, var time: Time, var zone: TimeZoneInfo) {
    companion object {

        internal val libClass: Class<ZonedIsoDateTimeLib> = ZonedIsoDateTimeLib::class.java
        internal val lib: ZonedIsoDateTimeLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ZonedIsoDateTimeNative::class.java).toLong()

        internal fun fromNative(nativeStruct: ZonedIsoDateTimeNative): ZonedIsoDateTime {
            val date: IsoDate = IsoDate(nativeStruct.date, listOf())
            val time: Time = Time(nativeStruct.time, listOf())
            val zone: TimeZoneInfo = TimeZoneInfo(nativeStruct.zone, listOf())

            return ZonedIsoDateTime(date, time, zone)
        }

        @JvmStatic
        
        /** Creates a new [ZonedIsoDateTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_strict_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_strict_from_str) for more information.
        */
        fun strictFromString(v: String, ianaParser: IanaParser): Result<ZonedIsoDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedIsoDateTime_strict_from_string_mv1(vSlice, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedIsoDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedIsoDateTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_full_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.try_full_from_str) for more information.
        */
        fun fullFromString(v: String, ianaParser: IanaParser, offsetCalculator: VariantOffsetsCalculator): Result<ZonedIsoDateTime> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedIsoDateTime_full_from_string_mv1(vSlice, ianaParser.handle, offsetCalculator.handle);
            if (returnVal.isOk == 1.toByte()) {
                
                val returnStruct = ZonedIsoDateTime.fromNative(returnVal.union.ok)
                if (vMem != null) vMem.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedIsoDateTime] from milliseconds since epoch (timestamp) and a UTC offset.
        *
        *Note: [ZonedIsoDateTime]s created with this constructor can only be formatted using localized offset zone styles.
        *
        *See the [Rust documentation for `from_epoch_milliseconds_and_utc_offset`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedDateTime.html#method.from_epoch_milliseconds_and_utc_offset) for more information.
        */
        fun fromEpochMillisecondsAndUtcOffset(epochMilliseconds: Long, utcOffset: UtcOffset): ZonedIsoDateTime {
            
            val returnVal = lib.icu4x_ZonedIsoDateTime_from_epoch_milliseconds_and_utc_offset_mv1(epochMilliseconds, utcOffset.handle);
            
            val returnStruct = ZonedIsoDateTime.fromNative(returnVal)
            return returnStruct
        }
    }
}
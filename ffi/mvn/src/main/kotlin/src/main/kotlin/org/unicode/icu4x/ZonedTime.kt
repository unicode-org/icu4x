package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZonedTimeLib: Library {
    fun icu4x_ZonedTime_strict_from_string_mv1(v: Slice, ianaParser: Pointer): ResultZonedTimeNativeInt
    fun icu4x_ZonedTime_location_only_from_string_mv1(v: Slice, ianaParser: Pointer): ResultZonedTimeNativeInt
    fun icu4x_ZonedTime_offset_only_from_string_mv1(v: Slice): ResultZonedTimeNativeInt
    fun icu4x_ZonedTime_lenient_from_string_mv1(v: Slice, ianaParser: Pointer): ResultZonedTimeNativeInt
}

internal class ZonedTimeNative: Structure(), Structure.ByValue {
    @JvmField
    internal var time: Pointer = Pointer(0);
    @JvmField
    internal var zone: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("time", "zone")
    }
}




internal class OptionZonedTimeNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: ZonedTimeNative = ZonedTimeNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): ZonedTimeNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: ZonedTimeNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: ZonedTimeNative): OptionZonedTimeNative {
            return OptionZonedTimeNative(value, 1)
        }

        internal fun none(): OptionZonedTimeNative {
            return OptionZonedTimeNative(ZonedTimeNative(), 0)
        }
    }

}

/** An ICU4X `ZonedTime` object capable of containing a ISO-8601 time, and zone.
*
*See the [Rust documentation for `ZonedTime`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedTime.html) for more information.
*/
class ZonedTime (var time: Time, var zone: TimeZoneInfo) {
    companion object {

        internal val libClass: Class<ZonedTimeLib> = ZonedTimeLib::class.java
        internal val lib: ZonedTimeLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ZonedTimeNative::class.java).toLong()

        internal fun fromNative(nativeStruct: ZonedTimeNative): ZonedTime {
            val time: Time = Time(nativeStruct.time, listOf())
            val zone: TimeZoneInfo = TimeZoneInfo(nativeStruct.zone, listOf())

            return ZonedTime(time, zone)
        }

        @JvmStatic
        
        /** Creates a new [ZonedTime] from an IXDTF string.
        *
        *See the [Rust documentation for `try_strict_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedTime.html#method.try_strict_from_str) for more information.
        */
        fun strictFromString(v: String, ianaParser: IanaParser): Result<ZonedTime> {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedTime_strict_from_string_mv1(vSliceMemory.slice, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                val returnStruct = ZonedTime.fromNative(returnVal.union.ok)
                vSliceMemory?.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedTime] from a location-only IXDTF string.
        *
        *See the [Rust documentation for `try_location_only_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedTime.html#method.try_location_only_from_str) for more information.
        */
        fun locationOnlyFromString(v: String, ianaParser: IanaParser): Result<ZonedTime> {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedTime_location_only_from_string_mv1(vSliceMemory.slice, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                val returnStruct = ZonedTime.fromNative(returnVal.union.ok)
                vSliceMemory?.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedTime] from an offset-only IXDTF string.
        *
        *See the [Rust documentation for `try_offset_only_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedTime.html#method.try_offset_only_from_str) for more information.
        */
        fun offsetOnlyFromString(v: String): Result<ZonedTime> {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedTime_offset_only_from_string_mv1(vSliceMemory.slice);
            if (returnVal.isOk == 1.toByte()) {
                val returnStruct = ZonedTime.fromNative(returnVal.union.ok)
                vSliceMemory?.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [ZonedTime] from an IXDTF string, without requiring the offset.
        *
        *See the [Rust documentation for `try_lenient_from_str`](https://docs.rs/icu/2.1.1/icu/time/struct.ZonedTime.html#method.try_lenient_from_str) for more information.
        */
        fun lenientFromString(v: String, ianaParser: IanaParser): Result<ZonedTime> {
            val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_ZonedTime_lenient_from_string_mv1(vSliceMemory.slice, ianaParser.handle);
            if (returnVal.isOk == 1.toByte()) {
                val returnStruct = ZonedTime.fromNative(returnVal.union.ok)
                vSliceMemory?.close()
                return returnStruct.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }
}
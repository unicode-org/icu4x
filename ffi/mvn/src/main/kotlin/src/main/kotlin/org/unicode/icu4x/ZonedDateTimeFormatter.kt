package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ZonedDateTimeFormatterLib: Library {
    fun icu4x_ZonedDateTimeFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_ZonedDateTimeFormatter_create_specific_long_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_specific_long_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_specific_short_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_specific_short_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_localized_offset_long_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_localized_offset_short_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_generic_long_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_generic_long_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_generic_short_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_generic_short_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_location_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_location_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_exemplar_city_mv1(locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_create_exemplar_city_with_provider_mv1(provider: Pointer, locale: Pointer, formatter: Pointer): ResultPointerInt
    fun icu4x_ZonedDateTimeFormatter_format_iso_mv1(handle: Pointer, isoDate: Pointer, time: Pointer, zone: Pointer, write: Pointer): ResultUnitInt
    fun icu4x_ZonedDateTimeFormatter_format_same_calendar_mv1(handle: Pointer, date: Pointer, time: Pointer, zone: Pointer, write: Pointer): ResultUnitDateTimeMismatchedCalendarErrorNative
}
/** See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html) for more information.
*/
class ZonedDateTimeFormatter internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class ZonedDateTimeFormatterCleaner(val handle: Pointer, val lib: ZonedDateTimeFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_ZonedDateTimeFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ZonedDateTimeFormatter.ZonedDateTimeFormatterCleaner(handle, ZonedDateTimeFormatter.lib));
    }

    companion object {
        internal val libClass: Class<ZonedDateTimeFormatterLib> = ZonedDateTimeFormatterLib::class.java
        internal val lib: ZonedDateTimeFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `SpecificLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.SpecificLong.html) for more information.
        */
        fun createSpecificLong(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_specific_long_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `SpecificLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.SpecificLong.html) for more information.
        */
        fun createSpecificLongWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_specific_long_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `SpecificShort`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.SpecificShort.html) for more information.
        */
        fun createSpecificShort(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_specific_short_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `SpecificShort`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.SpecificShort.html) for more information.
        */
        fun createSpecificShortWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_specific_short_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `LocalizedOffsetLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.LocalizedOffsetLong.html) for more information.
        */
        fun createLocalizedOffsetLong(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_localized_offset_long_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `LocalizedOffsetLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.LocalizedOffsetLong.html) for more information.
        */
        fun createLocalizedOffsetLongWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `LocalizedOffsetShort`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.LocalizedOffsetShort.html) for more information.
        */
        fun createLocalizedOffsetShort(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_localized_offset_short_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `LocalizedOffsetShort`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.LocalizedOffsetShort.html) for more information.
        */
        fun createLocalizedOffsetShortWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `GenericLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.GenericLong.html) for more information.
        */
        fun createGenericLong(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_generic_long_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `GenericLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.GenericLong.html) for more information.
        */
        fun createGenericLongWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_generic_long_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `GenericShort`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.GenericShort.html) for more information.
        */
        fun createGenericShort(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_generic_short_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `GenericShort`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.GenericShort.html) for more information.
        */
        fun createGenericShortWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_generic_short_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `Location`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.Location.html) for more information.
        */
        fun createLocation(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_location_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `Location`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.Location.html) for more information.
        */
        fun createLocationWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_location_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `ExemplarCity`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.ExemplarCity.html) for more information.
        */
        fun createExemplarCity(locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_exemplar_city_mv1(locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `ExemplarCity`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.ExemplarCity.html) for more information.
        */
        fun createExemplarCityWithProvider(provider: DataProvider, locale: Locale, formatter: DateTimeFormatter): Result<ZonedDateTimeFormatter> {
            
            val returnVal = lib.icu4x_ZonedDateTimeFormatter_create_exemplar_city_with_provider_mv1(provider.handle, locale.handle, formatter.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ZonedDateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
    */
    fun formatIso(isoDate: IsoDate, time: Time, zone: TimeZoneInfo): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_ZonedDateTimeFormatter_format_iso_mv1(handle, isoDate.handle, time.handle, zone.handle, write);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            return DateTimeWriteErrorError(DateTimeWriteError.fromNative(returnVal.getNativeErr()!!)).err()
        }
    }
    
    /** See the [Rust documentation for `format_same_calendar`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.format_same_calendar) for more information.
    */
    fun formatSameCalendar(date: Date, time: Time, zone: TimeZoneInfo): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_ZonedDateTimeFormatter_format_same_calendar_mv1(handle, date.handle, time.handle, zone.handle, write);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            val returnStruct = DateTimeMismatchedCalendarError.fromNative(returnVal.getNativeErr()!!)
            return returnStruct.err()
        }
    }

}
package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeZoneFormatterLib: Library {
    fun icu4x_TimeZoneFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_TimeZoneFormatter_create_specific_long_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_specific_long_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_specific_short_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_specific_short_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_localized_offset_long_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_localized_offset_long_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_localized_offset_short_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_localized_offset_short_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_generic_long_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_generic_long_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_generic_short_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_generic_short_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_location_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_location_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_exemplar_city_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_create_exemplar_city_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_TimeZoneFormatter_format_mv1(handle: Pointer, zone: Pointer, write: Pointer): ResultUnitInt
}
/** See the [Rust documentation for `NoCalendarFormatter`](https://docs.rs/icu/2.1.1/icu/datetime/type.NoCalendarFormatter.html) for more information.
*/
class TimeZoneFormatter internal constructor (
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

    private class TimeZoneFormatterCleaner(val handle: Pointer, val lib: TimeZoneFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeZoneFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, TimeZoneFormatter.TimeZoneFormatterCleaner(handle, TimeZoneFormatter.lib));
    }

    companion object {
        internal val libClass: Class<TimeZoneFormatterLib> = TimeZoneFormatterLib::class.java
        internal val lib: TimeZoneFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a zoned formatter based on a non-zoned formatter.
        *
        *Caution: The locale provided here must match the locale used to construct the non-zoned formatter,
        *or else unexpected behavior may occur!
        *
        *See the [Rust documentation for `SpecificLong`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/zone/struct.SpecificLong.html) for more information.
        */
        fun createSpecificLong(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_specific_long_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createSpecificLongWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_specific_long_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createSpecificShort(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_specific_short_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createSpecificShortWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_specific_short_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createLocalizedOffsetLong(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_localized_offset_long_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createLocalizedOffsetLongWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_localized_offset_long_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createLocalizedOffsetShort(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_localized_offset_short_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createLocalizedOffsetShortWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_localized_offset_short_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createGenericLong(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_generic_long_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createGenericLongWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_generic_long_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createGenericShort(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_generic_short_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createGenericShortWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_generic_short_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createLocation(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_location_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createLocationWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_location_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createExemplarCity(locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_exemplar_city_mv1(locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
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
        fun createExemplarCityWithProvider(provider: DataProvider, locale: Locale): Result<TimeZoneFormatter> {
            
            val returnVal = lib.icu4x_TimeZoneFormatter_create_exemplar_city_with_provider_mv1(provider.handle, locale.handle);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeZoneFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/datetime/struct.FixedCalendarDateTimeFormatter.html#method.format) for more information.
    */
    fun format(zone: TimeZoneInfo): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_TimeZoneFormatter_format_mv1(handle, zone.handle, write);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            return DateTimeWriteErrorError(DateTimeWriteError.fromNative(returnVal.getNativeErr()!!)).err()
        }
    }

}
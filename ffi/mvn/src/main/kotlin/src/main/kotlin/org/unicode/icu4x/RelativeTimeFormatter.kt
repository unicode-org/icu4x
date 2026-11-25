package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RelativeTimeFormatterLib: Library {
    fun icu4x_RelativeTimeFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_RelativeTimeFormatter_create_long_second_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_minute_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_hour_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_day_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_week_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_month_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_quarter_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_year_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_second_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_minute_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_hour_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_day_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_week_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_month_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_quarter_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_year_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_second_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_minute_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_hour_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_day_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_week_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_month_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_year_mv1(locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1(provider: Pointer, locale: Pointer, options: RelativeTimeFormatterOptionsNative): ResultPointerInt
    fun icu4x_RelativeTimeFormatter_format_mv1(handle: Pointer, value: Pointer, write: Pointer): Unit
}
/**
 * An ICU4X `RelativeTimeFormatter` object capable of formatting a [Decimal] into a
 * locale-sensitive relative time string.
 *
 * See the [Rust documentation for `RelativeTimeFormatter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class RelativeTimeFormatter internal constructor (
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

    private class RelativeTimeFormatterCleaner(val handle: Pointer, val lib: RelativeTimeFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_RelativeTimeFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, RelativeTimeFormatter.RelativeTimeFormatterCleaner(handle, RelativeTimeFormatter.lib));
    }

    companion object {
        internal val libClass: Class<RelativeTimeFormatterLib> = RelativeTimeFormatterLib::class.java
        internal val lib: RelativeTimeFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long seconds from compiled data.
         *
         * See the [Rust documentation for `try_new_long_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_second) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongSecond(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_second_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long seconds using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_second) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongSecondWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long minutes from compiled data.
         *
         * See the [Rust documentation for `try_new_long_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_minute) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongMinute(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_minute_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long minutes using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_minute) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongMinuteWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long hours from compiled data.
         *
         * See the [Rust documentation for `try_new_long_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_hour) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongHour(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_hour_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long hours using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_hour) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongHourWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long days from compiled data.
         *
         * See the [Rust documentation for `try_new_long_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_day) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongDay(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_day_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long days using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_day) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongDayWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long weeks from compiled data.
         *
         * See the [Rust documentation for `try_new_long_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_week) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongWeek(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_week_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long weeks using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_week) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongWeekWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long months from compiled data.
         *
         * See the [Rust documentation for `try_new_long_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_month) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongMonth(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_month_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long months using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_month) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongMonthWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long quarters from compiled data.
         *
         * See the [Rust documentation for `try_new_long_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_quarter) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongQuarter(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_quarter_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long quarters using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_quarter) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongQuarterWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long years from compiled data.
         *
         * See the [Rust documentation for `try_new_long_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_year) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongYear(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_year_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for long years using a particular data source.
         *
         * See the [Rust documentation for `try_new_long_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_year) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createLongYearWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short seconds from compiled data.
         *
         * See the [Rust documentation for `try_new_short_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_second) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortSecond(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_second_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short seconds using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_second) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortSecondWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short minutes from compiled data.
         *
         * See the [Rust documentation for `try_new_short_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_minute) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortMinute(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_minute_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short minutes using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_minute) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortMinuteWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short hours from compiled data.
         *
         * See the [Rust documentation for `try_new_short_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_hour) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortHour(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_hour_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short hours using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_hour) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortHourWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short days from compiled data.
         *
         * See the [Rust documentation for `try_new_short_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_day) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortDay(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_day_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short days using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_day) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortDayWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short weeks from compiled data.
         *
         * See the [Rust documentation for `try_new_short_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_week) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortWeek(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_week_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short weeks using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_week) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortWeekWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short months from compiled data.
         *
         * See the [Rust documentation for `try_new_short_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_month) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortMonth(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_month_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short months using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_month) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortMonthWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short quarters from compiled data.
         *
         * See the [Rust documentation for `try_new_short_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_quarter) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortQuarter(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_quarter_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short quarters using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_quarter) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortQuarterWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short years from compiled data.
         *
         * See the [Rust documentation for `try_new_short_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_year) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortYear(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_year_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for short years using a particular data source.
         *
         * See the [Rust documentation for `try_new_short_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_year) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createShortYearWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow seconds from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_second) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowSecond(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_second_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow seconds using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_second) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowSecondWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow minutes from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_minute) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowMinute(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_minute_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow minutes using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_minute) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowMinuteWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow hours from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_hour) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowHour(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_hour_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow hours using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_hour) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowHourWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow days from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_day) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowDay(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_day_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow days using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_day) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowDayWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow weeks from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_week) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowWeek(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_week_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow weeks using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_week) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowWeekWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow months from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_month) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowMonth(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_month_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow months using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_month) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowMonthWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow quarters from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_quarter) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowQuarter(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow quarters using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_quarter) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowQuarterWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow years from compiled data.
         *
         * See the [Rust documentation for `try_new_narrow_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_year) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowYear(locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_year_mv1(locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /**
         * Construct a new [RelativeTimeFormatter] for narrow years using a particular data source.
         *
         * See the [Rust documentation for `try_new_narrow_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_year) for more information.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        fun createNarrowYearWithProvider(provider: DataProvider, locale: Locale, options: RelativeTimeFormatterOptions): Result<RelativeTimeFormatter> {
            
            val returnVal = lib.icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1(provider.handle, locale.handle, options.toNative());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = RelativeTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /**
     * Formats a [Decimal] to a string.
     *
     * See the [Rust documentation for `format`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.format) for more information.
     *
     * 🚧 This API is unstable and may experience breaking changes outside major releases.
     */
    fun format(value: Decimal): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_RelativeTimeFormatter_format_mv1(handle, value.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
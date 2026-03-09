package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateTimeFormatterLib: Library {
    fun icu4x_DateTimeFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_DateTimeFormatter_create_dt_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_dt_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_mdt_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_mdt_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_ymdt_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_ymdt_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_det_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_det_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_mdet_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_mdet_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_ymdet_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_ymdet_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_et_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_create_et_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateTimeFormatter_format_iso_mv1(handle: Pointer, isoDate: Pointer, time: Pointer, write: Pointer): Unit
    fun icu4x_DateTimeFormatter_format_same_calendar_mv1(handle: Pointer, date: Pointer, time: Pointer, write: Pointer): ResultUnitDateTimeMismatchedCalendarErrorNative
}
/** See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html) for more information.
*/
class DateTimeFormatter internal constructor (
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

    private class DateTimeFormatterCleaner(val handle: Pointer, val lib: DateTimeFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_DateTimeFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, DateTimeFormatter.DateTimeFormatterCleaner(handle, DateTimeFormatter.lib));
    }

    companion object {
        internal val libClass: Class<DateTimeFormatterLib> = DateTimeFormatterLib::class.java
        internal val lib: DateTimeFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `DT`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html#method.for_length)
        */
        fun createDt(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_dt_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `DT`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DT.html#method.for_length)
        */
        fun createDtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_dt_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MDT`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html#method.for_length)
        */
        fun createMdt(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_mdt_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MDT`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDT.html#method.for_length)
        */
        fun createMdtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_mdt_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMDT`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.with_year_style), [4](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.for_length)
        */
        fun createYmdt(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_ymdt_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMDT`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.with_year_style), [4](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDT.html#method.for_length)
        */
        fun createYmdtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_ymdt_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `DET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html#method.for_length)
        */
        fun createDet(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_det_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `DET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DET.html#method.for_length)
        */
        fun createDetWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_det_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MDET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html#method.for_length)
        */
        fun createMdet(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_mdet_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MDET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDET.html#method.for_length)
        */
        fun createMdetWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_mdet_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMDET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.with_year_style), [4](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.for_length)
        */
        fun createYmdet(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_ymdet_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMDET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.with_year_style), [4](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDET.html#method.for_length)
        */
        fun createYmdetWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_ymdet_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `ET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html#method.for_length)
        */
        fun createEt(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_et_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `ET`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.ET.html#method.for_length)
        */
        fun createEtWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<DateTimeFormatter> {
            
            val returnVal = lib.icu4x_DateTimeFormatter_create_et_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateTimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
    */
    fun formatIso(isoDate: IsoDate, time: Time): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DateTimeFormatter_format_iso_mv1(handle, isoDate.handle, time.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** See the [Rust documentation for `format_same_calendar`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.format_same_calendar) for more information.
    */
    fun formatSameCalendar(date: Date, time: Time): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DateTimeFormatter_format_same_calendar_mv1(handle, date.handle, time.handle, write);
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
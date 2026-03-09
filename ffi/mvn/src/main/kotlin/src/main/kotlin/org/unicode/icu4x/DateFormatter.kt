package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateFormatterLib: Library {
    fun icu4x_DateFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_DateFormatter_create_d_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_d_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_md_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_md_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_ymd_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_ymd_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_de_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_de_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_mde_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_mde_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_ymde_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_ymde_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_e_mv1(locale: Pointer, length: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_e_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_m_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_m_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_ym_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_ym_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_y_mv1(locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_create_y_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, alignment: OptionInt, yearStyle: OptionInt): ResultPointerInt
    fun icu4x_DateFormatter_format_iso_mv1(handle: Pointer, isoDate: Pointer, write: Pointer): Unit
    fun icu4x_DateFormatter_format_same_calendar_mv1(handle: Pointer, date: Pointer, write: Pointer): ResultUnitDateTimeMismatchedCalendarErrorNative
}
/** See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html) for more information.
*/
class DateFormatter internal constructor (
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

    private class DateFormatterCleaner(val handle: Pointer, val lib: DateFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_DateFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, DateFormatter.DateFormatterCleaner(handle, DateFormatter.lib));
    }

    companion object {
        internal val libClass: Class<DateFormatterLib> = DateFormatterLib::class.java
        internal val lib: DateFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `D`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.D.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.D.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.D.html#method.for_length)
        */
        fun createD(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_d_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `D`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.D.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.D.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.D.html#method.for_length)
        */
        fun createDWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_d_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MD`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MD.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MD.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MD.html#method.for_length)
        */
        fun createMd(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_md_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MD`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MD.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MD.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MD.html#method.for_length)
        */
        fun createMdWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_md_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMD`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html#method.for_length)
        */
        fun createYmd(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_ymd_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMD`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMD.html#method.for_length)
        */
        fun createYmdWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_ymd_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `DE`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DE.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DE.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DE.html#method.for_length)
        */
        fun createDe(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_de_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `DE`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DE.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DE.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.DE.html#method.for_length)
        */
        fun createDeWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_de_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MDE`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDE.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDE.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDE.html#method.for_length)
        */
        fun createMde(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_mde_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `MDE`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDE.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDE.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.MDE.html#method.for_length)
        */
        fun createMdeWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_mde_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMDE`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html#method.for_length)
        */
        fun createYmde(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_ymde_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YMDE`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YMDE.html#method.for_length)
        */
        fun createYmdeWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_ymde_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `E`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.E.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.E.html#method.for_length)
        */
        fun createE(locale: Locale, length: DateTimeLength?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_e_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `E`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.E.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.E.html#method.for_length)
        */
        fun createEWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_e_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `M`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.M.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.M.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.M.html#method.for_length)
        */
        fun createM(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_m_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `M`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.M.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.M.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.M.html#method.for_length)
        */
        fun createMWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_m_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YM`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html#method.for_length)
        */
        fun createYm(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_ym_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `YM`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.YM.html#method.for_length)
        */
        fun createYmWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_ym_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `Y`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html#method.for_length)
        */
        fun createY(locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_y_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `Y`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html#method.with_alignment), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html#method.with_year_style), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.Y.html#method.for_length)
        */
        fun createYWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, alignment: DateTimeAlignment?, yearStyle: YearStyle?): Result<DateFormatter> {
            
            val returnVal = lib.icu4x_DateFormatter_create_y_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), yearStyle?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = DateFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
    */
    fun formatIso(isoDate: IsoDate): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DateFormatter_format_iso_mv1(handle, isoDate.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** See the [Rust documentation for `format_same_calendar`](https://docs.rs/icu/2.1.1/icu/datetime/struct.DateTimeFormatter.html#method.format_same_calendar) for more information.
    */
    fun formatSameCalendar(date: Date): Result<String> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_DateFormatter_format_same_calendar_mv1(handle, date.handle, write);
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
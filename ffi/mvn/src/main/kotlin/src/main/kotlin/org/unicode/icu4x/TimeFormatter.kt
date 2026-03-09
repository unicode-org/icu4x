package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TimeFormatterLib: Library {
    fun icu4x_TimeFormatter_destroy_mv1(handle: Pointer)
    fun icu4x_TimeFormatter_create_mv1(locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_TimeFormatter_create_with_provider_mv1(provider: Pointer, locale: Pointer, length: OptionInt, timePrecision: OptionInt, alignment: OptionInt): ResultPointerInt
    fun icu4x_TimeFormatter_format_mv1(handle: Pointer, time: Pointer, write: Pointer): Unit
}
/** See the [Rust documentation for `NoCalendarFormatter`](https://docs.rs/icu/2.1.1/icu/datetime/type.NoCalendarFormatter.html) for more information.
*/
class TimeFormatter internal constructor (
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

    private class TimeFormatterCleaner(val handle: Pointer, val lib: TimeFormatterLib) : Runnable {
        override fun run() {
            lib.icu4x_TimeFormatter_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, TimeFormatter.TimeFormatterCleaner(handle, TimeFormatter.lib));
    }

    companion object {
        internal val libClass: Class<TimeFormatterLib> = TimeFormatterLib::class.java
        internal val lib: TimeFormatterLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/type.NoCalendarFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `T`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html#method.for_length)
        */
        fun create(locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<TimeFormatter> {
            
            val returnVal = lib.icu4x_TimeFormatter_create_mv1(locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        /** See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/datetime/type.NoCalendarFormatter.html#method.try_new) for more information.
        *
        *See the [Rust documentation for `T`](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html) for more information.
        *
        *Additional information: [1](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html#method.with_time_precision), [2](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html#method.with_alignment), [3](https://docs.rs/icu/2.1.1/icu/datetime/fieldsets/struct.T.html#method.for_length)
        */
        fun createWithProvider(provider: DataProvider, locale: Locale, length: DateTimeLength?, timePrecision: TimePrecision?, alignment: DateTimeAlignment?): Result<TimeFormatter> {
            
            val returnVal = lib.icu4x_TimeFormatter_create_with_provider_mv1(provider.handle, locale.handle, length?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), timePrecision?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), alignment?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none());
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = TimeFormatter(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return DateTimeFormatterLoadErrorError(DateTimeFormatterLoadError.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `format`](https://docs.rs/icu/2.1.1/icu/datetime/type.NoCalendarFormatter.html#method.format) for more information.
    */
    fun format(time: Time): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_TimeFormatter_format_mv1(handle, time.handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}
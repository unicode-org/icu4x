package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface IsoDateLib: Library {
    fun icu4x_IsoDate_destroy_mv1(handle: Pointer)
    fun icu4x_IsoDate_create_mv1(year: Int, month: FFIUint8, day: FFIUint8): ResultPointerInt
    fun icu4x_IsoDate_from_rata_die_mv1(rd: Long): Pointer
    fun icu4x_IsoDate_from_string_mv1(v: Slice): ResultPointerInt
    fun icu4x_IsoDate_to_calendar_mv1(handle: Pointer, calendar: Pointer): Pointer
    fun icu4x_IsoDate_to_any_mv1(handle: Pointer): Pointer
    fun icu4x_IsoDate_to_rata_die_mv1(handle: Pointer): Long
    fun icu4x_IsoDate_day_of_year_mv1(handle: Pointer): FFIUint16
    fun icu4x_IsoDate_day_of_month_mv1(handle: Pointer): FFIUint8
    fun icu4x_IsoDate_day_of_week_mv1(handle: Pointer): Int
    fun icu4x_IsoDate_weekday_mv1(handle: Pointer): Int
    fun icu4x_IsoDate_week_of_year_mv1(handle: Pointer): IsoWeekOfYearNative
    fun icu4x_IsoDate_month_mv1(handle: Pointer): FFIUint8
    fun icu4x_IsoDate_year_mv1(handle: Pointer): Int
    fun icu4x_IsoDate_is_in_leap_year_mv1(handle: Pointer): Byte
    fun icu4x_IsoDate_months_in_year_mv1(handle: Pointer): FFIUint8
    fun icu4x_IsoDate_days_in_month_mv1(handle: Pointer): FFIUint8
    fun icu4x_IsoDate_days_in_year_mv1(handle: Pointer): FFIUint16
}
/** An ICU4X Date object capable of containing a ISO-8601 date
*
*See the [Rust documentation for `Date`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html) for more information.
*/
class IsoDate internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class IsoDateCleaner(val handle: Pointer, val lib: IsoDateLib) : Runnable {
        override fun run() {
            lib.icu4x_IsoDate_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<IsoDateLib> = IsoDateLib::class.java
        internal val lib: IsoDateLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [IsoDate] from the specified date.
        *
        *See the [Rust documentation for `try_new_iso`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.try_new_iso) for more information.
        */
        fun create(year: Int, month: UByte, day: UByte): Result<IsoDate> {
            
            val returnVal = lib.icu4x_IsoDate_create_mv1(year, FFIUint8(month), FFIUint8(day));
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = IsoDate(handle, selfEdges)
                CLEANER.register(returnOpaque, IsoDate.IsoDateCleaner(handle, IsoDate.lib));
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [IsoDate] from the given Rata Die
        *
        *See the [Rust documentation for `from_rata_die`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.from_rata_die) for more information.
        */
        fun fromRataDie(rd: Long): IsoDate {
            
            val returnVal = lib.icu4x_IsoDate_from_rata_die_mv1(rd);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = IsoDate(handle, selfEdges)
            CLEANER.register(returnOpaque, IsoDate.IsoDateCleaner(handle, IsoDate.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Creates a new [IsoDate] from an IXDTF string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.try_from_str) for more information.
        */
        fun fromString(v: String): Result<IsoDate> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_IsoDate_from_string_mv1(vSlice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = IsoDate(handle, selfEdges)
                CLEANER.register(returnOpaque, IsoDate.IsoDateCleaner(handle, IsoDate.lib));
                if (vMem != null) vMem.close()
                return returnOpaque.ok()
            } else {
                return Rfc9557ParseErrorError(Rfc9557ParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Convert this date to one in a different calendar
    *
    *See the [Rust documentation for `to_calendar`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.to_calendar) for more information.
    */
    fun toCalendar(calendar: Calendar): Date {
        
        val returnVal = lib.icu4x_IsoDate_to_calendar_mv1(handle, calendar.handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Date(handle, selfEdges)
        CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
        return returnOpaque
    }
    
    /** See the [Rust documentation for `to_any`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.to_any) for more information.
    */
    fun toAny(): Date {
        
        val returnVal = lib.icu4x_IsoDate_to_any_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Date(handle, selfEdges)
        CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
        return returnOpaque
    }
    
    /** Returns this date's Rata Die
    *
    *See the [Rust documentation for `to_rata_die`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.to_rata_die) for more information.
    */
    fun toRataDie(): Long {
        
        val returnVal = lib.icu4x_IsoDate_to_rata_die_mv1(handle);
        return (returnVal)
    }
    
    /** Returns the 1-indexed day in the year for this date
    *
    *See the [Rust documentation for `day_of_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.day_of_year) for more information.
    */
    fun dayOfYear(): UShort {
        
        val returnVal = lib.icu4x_IsoDate_day_of_year_mv1(handle);
        return (returnVal.toUShort())
    }
    
    /** Returns the 1-indexed day in the month for this date
    *
    *See the [Rust documentation for `day_of_month`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.day_of_month) for more information.
    */
    fun dayOfMonth(): UByte {
        
        val returnVal = lib.icu4x_IsoDate_day_of_month_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the day in the week for this day
    *
    *This is *not* the day of the week, an ordinal number that is locale
    *dependent.
    *
    *See the [Rust documentation for `day_of_week`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.day_of_week) for more information.
    */
    fun dayOfWeek(): Weekday {
        
        val returnVal = lib.icu4x_IsoDate_day_of_week_mv1(handle);
        return (Weekday.fromNative(returnVal))
    }
    
    /** Returns the day in the week for this day
    *
    *See the [Rust documentation for `weekday`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.weekday) for more information.
    */
    fun weekday(): Weekday {
        
        val returnVal = lib.icu4x_IsoDate_weekday_mv1(handle);
        return (Weekday.fromNative(returnVal))
    }
    
    /** Returns the week number in this year, using week data
    *
    *See the [Rust documentation for `week_of_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.week_of_year) for more information.
    */
    fun weekOfYear(): IsoWeekOfYear {
        
        val returnVal = lib.icu4x_IsoDate_week_of_year_mv1(handle);
        
        val returnStruct = IsoWeekOfYear.fromNative(returnVal)
        return returnStruct
    }
    
    /** Returns 1-indexed number of the month of this date in its year
    *
    *See the [Rust documentation for `ordinal`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.MonthInfo.html#structfield.ordinal) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.month)
    */
    fun month(): UByte {
        
        val returnVal = lib.icu4x_IsoDate_month_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the year number in the current era for this date
    *
    *For calendars without an era, returns the extended year
    *
    *See the [Rust documentation for `year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.year) for more information.
    */
    fun year(): Int {
        
        val returnVal = lib.icu4x_IsoDate_year_mv1(handle);
        return (returnVal)
    }
    
    /** Returns if the year is a leap year for this date
    *
    *See the [Rust documentation for `is_in_leap_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.is_in_leap_year) for more information.
    */
    fun isInLeapYear(): Boolean {
        
        val returnVal = lib.icu4x_IsoDate_is_in_leap_year_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Returns the number of months in the year represented by this date
    *
    *See the [Rust documentation for `months_in_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.months_in_year) for more information.
    */
    fun monthsInYear(): UByte {
        
        val returnVal = lib.icu4x_IsoDate_months_in_year_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the number of days in the month represented by this date
    *
    *See the [Rust documentation for `days_in_month`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.days_in_month) for more information.
    */
    fun daysInMonth(): UByte {
        
        val returnVal = lib.icu4x_IsoDate_days_in_month_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the number of days in the year represented by this date
    *
    *See the [Rust documentation for `days_in_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.days_in_year) for more information.
    */
    fun daysInYear(): UShort {
        
        val returnVal = lib.icu4x_IsoDate_days_in_year_mv1(handle);
        return (returnVal.toUShort())
    }

}
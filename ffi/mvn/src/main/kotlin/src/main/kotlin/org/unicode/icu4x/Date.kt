package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DateLib: Library {
    fun icu4x_Date_destroy_mv1(handle: Pointer)
    fun icu4x_Date_from_iso_in_calendar_mv1(isoYear: Int, isoMonth: FFIUint8, isoDay: FFIUint8, calendar: Pointer): ResultPointerInt
    fun icu4x_Date_from_codes_in_calendar_mv1(eraCode: Slice, year: Int, monthCode: Slice, day: FFIUint8, calendar: Pointer): ResultPointerInt
    fun icu4x_Date_from_rata_die_mv1(rd: Long, calendar: Pointer): ResultPointerInt
    fun icu4x_Date_from_string_mv1(v: Slice, calendar: Pointer): ResultPointerInt
    fun icu4x_Date_to_calendar_mv1(handle: Pointer, calendar: Pointer): Pointer
    fun icu4x_Date_to_iso_mv1(handle: Pointer): Pointer
    fun icu4x_Date_to_rata_die_mv1(handle: Pointer): Long
    fun icu4x_Date_day_of_year_mv1(handle: Pointer): FFIUint16
    fun icu4x_Date_day_of_month_mv1(handle: Pointer): FFIUint8
    fun icu4x_Date_day_of_week_mv1(handle: Pointer): Int
    fun icu4x_Date_weekday_mv1(handle: Pointer): Int
    fun icu4x_Date_ordinal_month_mv1(handle: Pointer): FFIUint8
    fun icu4x_Date_month_code_mv1(handle: Pointer, write: Pointer): Unit
    fun icu4x_Date_month_number_mv1(handle: Pointer): FFIUint8
    fun icu4x_Date_month_is_leap_mv1(handle: Pointer): Byte
    fun icu4x_Date_era_year_or_related_iso_mv1(handle: Pointer): Int
    fun icu4x_Date_extended_year_mv1(handle: Pointer): Int
    fun icu4x_Date_era_mv1(handle: Pointer, write: Pointer): Unit
    fun icu4x_Date_months_in_year_mv1(handle: Pointer): FFIUint8
    fun icu4x_Date_days_in_month_mv1(handle: Pointer): FFIUint8
    fun icu4x_Date_days_in_year_mv1(handle: Pointer): FFIUint16
    fun icu4x_Date_calendar_mv1(handle: Pointer): Pointer
}
/** An ICU4X Date object capable of containing a date for any calendar.
*
*See the [Rust documentation for `Date`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html) for more information.
*/
class Date internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class DateCleaner(val handle: Pointer, val lib: DateLib) : Runnable {
        override fun run() {
            lib.icu4x_Date_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<DateLib> = DateLib::class.java
        internal val lib: DateLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [Date] representing the ISO date
        *given but in a given calendar
        *
        *See the [Rust documentation for `new_from_iso`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.new_from_iso) for more information.
        */
        fun fromIsoInCalendar(isoYear: Int, isoMonth: UByte, isoDay: UByte, calendar: Calendar): Result<Date> {
            
            val returnVal = lib.icu4x_Date_from_iso_in_calendar_mv1(isoYear, FFIUint8(isoMonth), FFIUint8(isoDay), calendar.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Date(handle, selfEdges)
                CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [Date] from the given codes, which are interpreted in the given calendar system
        *
        *An empty era code will treat the year as an extended year
        *
        *See the [Rust documentation for `try_new_from_codes`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.try_new_from_codes) for more information.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.Month.html#method.try_from_str) for more information.
        */
        fun fromCodesInCalendar(eraCode: String, year: Int, monthCode: String, day: UByte, calendar: Calendar): Result<Date> {
            val (eraCodeMem, eraCodeSlice) = PrimitiveArrayTools.borrowUtf8(eraCode)
            val (monthCodeMem, monthCodeSlice) = PrimitiveArrayTools.borrowUtf8(monthCode)
            
            val returnVal = lib.icu4x_Date_from_codes_in_calendar_mv1(eraCodeSlice, year, monthCodeSlice, FFIUint8(day), calendar.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Date(handle, selfEdges)
                CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
                if (eraCodeMem != null) eraCodeMem.close()
                if (monthCodeMem != null) monthCodeMem.close()
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [Date] from the given Rata Die
        *
        *See the [Rust documentation for `from_rata_die`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.from_rata_die) for more information.
        */
        fun fromRataDie(rd: Long, calendar: Calendar): Result<Date> {
            
            val returnVal = lib.icu4x_Date_from_rata_die_mv1(rd, calendar.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Date(handle, selfEdges)
                CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
                return returnOpaque.ok()
            } else {
                return CalendarErrorError(CalendarError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [Date] from an IXDTF string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.try_from_str) for more information.
        */
        fun fromString(v: String, calendar: Calendar): Result<Date> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_Date_from_string_mv1(vSlice, calendar.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Date(handle, selfEdges)
                CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
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
        
        val returnVal = lib.icu4x_Date_to_calendar_mv1(handle, calendar.handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Date(handle, selfEdges)
        CLEANER.register(returnOpaque, Date.DateCleaner(handle, Date.lib));
        return returnOpaque
    }
    
    /** Converts this date to ISO
    *
    *See the [Rust documentation for `to_iso`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.to_iso) for more information.
    */
    fun toIso(): IsoDate {
        
        val returnVal = lib.icu4x_Date_to_iso_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = IsoDate(handle, selfEdges)
        CLEANER.register(returnOpaque, IsoDate.IsoDateCleaner(handle, IsoDate.lib));
        return returnOpaque
    }
    
    /** Returns this date's Rata Die
    *
    *See the [Rust documentation for `to_rata_die`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.to_rata_die) for more information.
    */
    fun toRataDie(): Long {
        
        val returnVal = lib.icu4x_Date_to_rata_die_mv1(handle);
        return (returnVal)
    }
    
    /** Returns the 1-indexed day in the year for this date
    *
    *See the [Rust documentation for `day_of_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.day_of_year) for more information.
    */
    fun dayOfYear(): UShort {
        
        val returnVal = lib.icu4x_Date_day_of_year_mv1(handle);
        return (returnVal.toUShort())
    }
    
    /** Returns the 1-indexed day in the month for this date
    *
    *See the [Rust documentation for `day_of_month`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.day_of_month) for more information.
    */
    fun dayOfMonth(): UByte {
        
        val returnVal = lib.icu4x_Date_day_of_month_mv1(handle);
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
        
        val returnVal = lib.icu4x_Date_day_of_week_mv1(handle);
        return (Weekday.fromNative(returnVal))
    }
    
    /** Returns the day in the week for this day
    *
    *See the [Rust documentation for `weekday`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.weekday) for more information.
    */
    fun weekday(): Weekday {
        
        val returnVal = lib.icu4x_Date_weekday_mv1(handle);
        return (Weekday.fromNative(returnVal))
    }
    
    /** Returns 1-indexed number of the month of this date in its year
    *
    *Note that for lunar calendars this may not lead to the same month
    *having the same ordinal month across years; use `month_code` if you care
    *about month identity.
    *
    *See the [Rust documentation for `month`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.month) for more information.
    *
    *See the [Rust documentation for `ordinal`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.MonthInfo.html#structfield.ordinal) for more information.
    */
    fun ordinalMonth(): UByte {
        
        val returnVal = lib.icu4x_Date_ordinal_month_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the month code for this date. Typically something
    *like "M01", "M02", but can be more complicated for lunar calendars.
    *
    *See the [Rust documentation for `code`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.Month.html#method.code) for more information.
    *
    *See the [Rust documentation for `standard_code`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.MonthInfo.html#structfield.standard_code) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.month)
    */
    fun monthCode(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Date_month_code_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Returns the month number of this month.
    *
    *See the [Rust documentation for `number`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.Month.html#method.number) for more information.
    */
    fun monthNumber(): UByte {
        
        val returnVal = lib.icu4x_Date_month_number_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns whether the month is a leap month.
    *
    *See the [Rust documentation for `is_leap`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.Month.html#method.is_leap) for more information.
    */
    fun monthIsLeap(): Boolean {
        
        val returnVal = lib.icu4x_Date_month_is_leap_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Returns the year number in the current era for this date
    *
    *For calendars without an era, returns the related ISO year.
    *
    *See the [Rust documentation for `era_year_or_related_iso`](https://docs.rs/icu/2.1.1/icu/calendar/types/enum.YearInfo.html#method.era_year_or_related_iso) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.EraYear.html#structfield.year), [2](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.CyclicYear.html#structfield.related_iso), [3](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.year)
    */
    fun eraYearOrRelatedIso(): Int {
        
        val returnVal = lib.icu4x_Date_era_year_or_related_iso_mv1(handle);
        return (returnVal)
    }
    
    /** Returns the extended year, which can be used for
    *
    *This year number can be used when you need a simple numeric representation
    *of the year, and can be meaningfully compared with extended years from other
    *eras or used in arithmetic.
    *
    *See the [Rust documentation for `extended_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.extended_year) for more information.
    */
    fun extendedYear(): Int {
        
        val returnVal = lib.icu4x_Date_extended_year_mv1(handle);
        return (returnVal)
    }
    
    /** Returns the era for this date, or an empty string
    *
    *See the [Rust documentation for `era`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.EraYear.html#structfield.era) for more information.
    *
    *Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.year)
    */
    fun era(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Date_era_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }
    
    /** Returns the number of months in the year represented by this date
    *
    *See the [Rust documentation for `months_in_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.months_in_year) for more information.
    */
    fun monthsInYear(): UByte {
        
        val returnVal = lib.icu4x_Date_months_in_year_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the number of days in the month represented by this date
    *
    *See the [Rust documentation for `days_in_month`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.days_in_month) for more information.
    */
    fun daysInMonth(): UByte {
        
        val returnVal = lib.icu4x_Date_days_in_month_mv1(handle);
        return (returnVal.toUByte())
    }
    
    /** Returns the number of days in the year represented by this date
    *
    *See the [Rust documentation for `days_in_year`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.days_in_year) for more information.
    */
    fun daysInYear(): UShort {
        
        val returnVal = lib.icu4x_Date_days_in_year_mv1(handle);
        return (returnVal.toUShort())
    }
    
    /** Returns the [Calendar] object backing this date
    *
    *See the [Rust documentation for `calendar`](https://docs.rs/icu/2.1.1/icu/calendar/struct.Date.html#method.calendar) for more information.
    */
    fun calendar(): Calendar {
        
        val returnVal = lib.icu4x_Date_calendar_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = Calendar(handle, selfEdges)
        CLEANER.register(returnOpaque, Calendar.CalendarCleaner(handle, Calendar.lib));
        return returnOpaque
    }

}
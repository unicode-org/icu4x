package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface WeekInformationLib: Library {
    fun icu4x_WeekInformation_destroy_mv1(handle: Pointer)
    fun icu4x_WeekInformation_create_mv1(locale: Pointer): ResultPointerInt
    fun icu4x_WeekInformation_create_with_provider_mv1(provider: Pointer, locale: Pointer): ResultPointerInt
    fun icu4x_WeekInformation_first_weekday_mv1(handle: Pointer): Int
    fun icu4x_WeekInformation_is_weekend_mv1(handle: Pointer, day: Int): Byte
    fun icu4x_WeekInformation_weekend_mv1(handle: Pointer): Pointer
}
/** A Week calculator, useful to be passed in to `week_of_year()` on Date and DateTime types
*
*See the [Rust documentation for `WeekInformation`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekInformation.html) for more information.
*/
class WeekInformation internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class WeekInformationCleaner(val handle: Pointer, val lib: WeekInformationLib) : Runnable {
        override fun run() {
            lib.icu4x_WeekInformation_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<WeekInformationLib> = WeekInformationLib::class.java
        internal val lib: WeekInformationLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Creates a new [WeekInformation] from locale data using compiled data.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekInformation.html#method.try_new) for more information.
        */
        fun create(locale: Locale): Result<WeekInformation> {
            
            val returnVal = lib.icu4x_WeekInformation_create_mv1(locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WeekInformation(handle, selfEdges)
                CLEANER.register(returnOpaque, WeekInformation.WeekInformationCleaner(handle, WeekInformation.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Creates a new [WeekInformation] from locale data using a particular data source.
        *
        *See the [Rust documentation for `try_new`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekInformation.html#method.try_new) for more information.
        */
        fun createWithProvider(provider: DataProvider, locale: Locale): Result<WeekInformation> {
            
            val returnVal = lib.icu4x_WeekInformation_create_with_provider_mv1(provider.handle, locale.handle);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = WeekInformation(handle, selfEdges)
                CLEANER.register(returnOpaque, WeekInformation.WeekInformationCleaner(handle, WeekInformation.lib));
                return returnOpaque.ok()
            } else {
                return DataErrorError(DataError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** Returns the weekday that starts the week for this object's locale
    *
    *See the [Rust documentation for `first_weekday`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekInformation.html#structfield.first_weekday) for more information.
    */
    fun firstWeekday(): Weekday {
        
        val returnVal = lib.icu4x_WeekInformation_first_weekday_mv1(handle);
        return (Weekday.fromNative(returnVal))
    }
    
    /** See the [Rust documentation for `weekend`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekInformation.html#structfield.weekend) for more information.
    *
    *See the [Rust documentation for `contains`](https://docs.rs/icu/2.1.1/icu/calendar/provider/struct.WeekdaySet.html#method.contains) for more information.
    */
    fun isWeekend(day: Weekday): Boolean {
        
        val returnVal = lib.icu4x_WeekInformation_is_weekend_mv1(handle, day.toNative());
        return (returnVal > 0)
    }
    
    /** See the [Rust documentation for `weekend`](https://docs.rs/icu/2.1.1/icu/calendar/week/struct.WeekInformation.html#method.weekend) for more information.
    */
    fun weekend(): WeekdaySetIterator {
        
        val returnVal = lib.icu4x_WeekInformation_weekend_mv1(handle);
        val selfEdges: List<Any> = listOf()
        val handle = returnVal 
        val returnOpaque = WeekdaySetIterator(handle, selfEdges)
        CLEANER.register(returnOpaque, WeekdaySetIterator.WeekdaySetIteratorCleaner(handle, WeekdaySetIterator.lib));
        return returnOpaque
    }

}
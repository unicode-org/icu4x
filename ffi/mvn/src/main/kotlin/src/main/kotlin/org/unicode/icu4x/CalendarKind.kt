package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CalendarKindLib: Library {
    fun icu4x_CalendarKind_create_mv1(locale: Pointer): Int
}
/** The various calendar types currently supported by [Calendar]
*
*See the [Rust documentation for `AnyCalendarKind`](https://docs.rs/icu/2.1.1/icu/calendar/enum.AnyCalendarKind.html) for more information.
*/
enum class CalendarKind(val inner: Int) {
    Iso(0),
    Gregorian(1),
    Buddhist(2),
    Japanese(3),
    JapaneseExtended(4),
    Ethiopian(5),
    EthiopianAmeteAlem(6),
    Indian(7),
    Coptic(8),
    Dangi(9),
    Chinese(10),
    Hebrew(11),
    HijriTabularTypeIIFriday(12),
    HijriSimulatedMecca(18),
    HijriTabularTypeIIThursday(14),
    HijriUmmAlQura(15),
    Persian(16),
    Roc(17);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<CalendarKindLib> = CalendarKindLib::class.java
        internal val lib: CalendarKindLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): CalendarKind {
            return when (native) {
                0 -> Iso
                1 -> Gregorian
                2 -> Buddhist
                3 -> Japanese
                4 -> JapaneseExtended
                5 -> Ethiopian
                6 -> EthiopianAmeteAlem
                7 -> Indian
                8 -> Coptic
                9 -> Dangi
                10 -> Chinese
                11 -> Hebrew
                12 -> HijriTabularTypeIIFriday
                18 -> HijriSimulatedMecca
                14 -> HijriTabularTypeIIThursday
                15 -> HijriUmmAlQura
                16 -> Persian
                17 -> Roc
                else -> throw RuntimeException("Failed to find variant ${native} of type CalendarKind")
            }
        }

        fun default(): CalendarKind {
            return Iso
        }
        @JvmStatic
        
        /** Creates a new [CalendarKind] for the specified locale, using compiled data.
        *
        *See the [Rust documentation for `new`](https://docs.rs/icu/2.1.1/icu/calendar/enum.AnyCalendarKind.html#method.new) for more information.
        */
        fun create(locale: Locale): CalendarKind {
            
            val returnVal = lib.icu4x_CalendarKind_create_mv1(locale.handle);
            return (CalendarKind.fromNative(returnVal))
        }
    }
}

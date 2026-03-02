package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/error/struct.MismatchedCalendarError.html)
*/
class CalendarMismatchedCalendarError (): Exception("Rust error result for CalendarMismatchedCalendarError") {
    companion object {

    }
}
package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

/** Additional information: [1](https://docs.rs/icu/2.1.1/icu/time/zone/struct.InvalidOffsetError.html)
*/
class TimeZoneInvalidOffsetError (): Exception("Rust error result for TimeZoneInvalidOffsetError") {
    companion object {

    }
}
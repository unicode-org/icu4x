package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

/** Additional information: [1](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/struct.LimitError.html)
*/
class DecimalLimitError (): Exception("Rust error result for DecimalLimitError") {
    companion object {

    }
}
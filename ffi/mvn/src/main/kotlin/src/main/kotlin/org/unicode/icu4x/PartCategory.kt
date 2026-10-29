package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PartCategoryLib: Library {
}
/**
 * The category of a [PartKind], corresponding to [writeable::Part::category].
 *
 * See the [Rust documentation for `category`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html#structfield.category) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
*/
enum class PartCategory {
    Unknown,
    Error,
    List,
    Decimal,
    DateTime,
    RelativeTime,
    Duration,
    Unit;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<PartCategoryLib> = PartCategoryLib::class.java
        internal val lib: PartCategoryLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): PartCategory {
            return PartCategory.entries[native]
        }

        fun default(): PartCategory {
            return Unknown
        }
    }
}

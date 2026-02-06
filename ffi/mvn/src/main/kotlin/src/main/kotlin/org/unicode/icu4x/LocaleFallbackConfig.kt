package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LocaleFallbackConfigLib: Library {
}

internal class LocaleFallbackConfigNative: Structure(), Structure.ByValue {
    /** Choice of priority mode.
    */
    @JvmField
    internal var priority: Int = LocaleFallbackPriority.default().toNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("priority")
    }
}




internal class OptionLocaleFallbackConfigNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: LocaleFallbackConfigNative = LocaleFallbackConfigNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): LocaleFallbackConfigNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: LocaleFallbackConfigNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: LocaleFallbackConfigNative): OptionLocaleFallbackConfigNative {
            return OptionLocaleFallbackConfigNative(value, 1)
        }

        internal fun none(): OptionLocaleFallbackConfigNative {
            return OptionLocaleFallbackConfigNative(LocaleFallbackConfigNative(), 0)
        }
    }

}

/** Collection of configurations for the ICU4X fallback algorithm.
*
*See the [Rust documentation for `LocaleFallbackConfig`](https://docs.rs/icu/2.1.1/icu/locale/fallback/struct.LocaleFallbackConfig.html) for more information.
*/
class LocaleFallbackConfig (var priority: LocaleFallbackPriority) {
    companion object {

        internal val libClass: Class<LocaleFallbackConfigLib> = LocaleFallbackConfigLib::class.java
        internal val lib: LocaleFallbackConfigLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(LocaleFallbackConfigNative::class.java).toLong()

        internal fun fromNative(nativeStruct: LocaleFallbackConfigNative): LocaleFallbackConfig {
            val priority: LocaleFallbackPriority = LocaleFallbackPriority.fromNative(nativeStruct.priority)

            return LocaleFallbackConfig(priority)
        }

    }
    internal fun toNative(): LocaleFallbackConfigNative {
        var native = LocaleFallbackConfigNative()
        native.priority = this.priority.toNative()
        return native
    }

}
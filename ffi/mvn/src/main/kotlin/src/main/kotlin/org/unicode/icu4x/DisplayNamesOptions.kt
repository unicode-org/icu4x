package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DisplayNamesOptionsLib: Library {
}

internal class DisplayNamesOptionsNative: Structure(), Structure.ByValue {
    /** The optional formatting style to use for display name.
    */
    @JvmField
    internal var style: OptionInt = OptionInt.none();
    /** The fallback return when the system does not have the
*requested display name, defaults to "code".
    */
    @JvmField
    internal var fallback: OptionInt = OptionInt.none();
    /** The language display kind, defaults to "dialect".
    */
    @JvmField
    internal var languageDisplay: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("style", "fallback", "languageDisplay")
    }
}




internal class OptionDisplayNamesOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: DisplayNamesOptionsNative = DisplayNamesOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): DisplayNamesOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: DisplayNamesOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: DisplayNamesOptionsNative): OptionDisplayNamesOptionsNative {
            return OptionDisplayNamesOptionsNative(value, 1)
        }

        internal fun none(): OptionDisplayNamesOptionsNative {
            return OptionDisplayNamesOptionsNative(DisplayNamesOptionsNative(), 0)
        }
    }

}

/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `DisplayNamesOptions`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/struct.DisplayNamesOptions.html) for more information.
*/
class DisplayNamesOptions (var style: DisplayNamesStyle?, var fallback: DisplayNamesFallback?, var languageDisplay: LanguageDisplay?) {
    companion object {

        internal val libClass: Class<DisplayNamesOptionsLib> = DisplayNamesOptionsLib::class.java
        internal val lib: DisplayNamesOptionsLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(DisplayNamesOptionsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: DisplayNamesOptionsNative): DisplayNamesOptions {
            val style: DisplayNamesStyle? = nativeStruct.style.option()?.let { DisplayNamesStyle.fromNative(it) }
            val fallback: DisplayNamesFallback? = nativeStruct.fallback.option()?.let { DisplayNamesFallback.fromNative(it) }
            val languageDisplay: LanguageDisplay? = nativeStruct.languageDisplay.option()?.let { LanguageDisplay.fromNative(it) }

            return DisplayNamesOptions(style, fallback, languageDisplay)
        }

    }
    internal fun toNative(): DisplayNamesOptionsNative {
        var native = DisplayNamesOptionsNative()
        native.style = this.style?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.fallback = this.fallback?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        native.languageDisplay = this.languageDisplay?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

}
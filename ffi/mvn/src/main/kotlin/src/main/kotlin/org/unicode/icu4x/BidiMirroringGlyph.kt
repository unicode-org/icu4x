package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BidiMirroringGlyphLib: Library {
    fun icu4x_BidiMirroringGlyph_for_char_mv1(ch: Int): BidiMirroringGlyphNative
}

internal class BidiMirroringGlyphNative: Structure(), Structure.ByValue {
    /** The mirroring glyph
    */
    @JvmField
    internal var mirroringGlyph: OptionInt = OptionInt.none();
    /** Whether the glyph is mirrored
    */
    @JvmField
    internal var mirrored: Byte = 0;
    /** The paired bracket type
    */
    @JvmField
    internal var pairedBracketType: Int = BidiPairedBracketType.default().toNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("mirroringGlyph", "mirrored", "pairedBracketType")
    }
}




internal class OptionBidiMirroringGlyphNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: BidiMirroringGlyphNative = BidiMirroringGlyphNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): BidiMirroringGlyphNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: BidiMirroringGlyphNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: BidiMirroringGlyphNative): OptionBidiMirroringGlyphNative {
            return OptionBidiMirroringGlyphNative(value, 1)
        }

        internal fun none(): OptionBidiMirroringGlyphNative {
            return OptionBidiMirroringGlyphNative(BidiMirroringGlyphNative(), 0)
        }
    }

}

/** See the [Rust documentation for `BidiMirroringGlyph`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.BidiMirroringGlyph.html) for more information.
*/
class BidiMirroringGlyph (var mirroringGlyph: Int?, var mirrored: Boolean, var pairedBracketType: BidiPairedBracketType) {
    companion object {

        internal val libClass: Class<BidiMirroringGlyphLib> = BidiMirroringGlyphLib::class.java
        internal val lib: BidiMirroringGlyphLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(BidiMirroringGlyphNative::class.java).toLong()

        internal fun fromNative(nativeStruct: BidiMirroringGlyphNative): BidiMirroringGlyph {
            val mirroringGlyph: Int? = nativeStruct.mirroringGlyph.option()?.let { it }
            val mirrored: Boolean = nativeStruct.mirrored > 0
            val pairedBracketType: BidiPairedBracketType = BidiPairedBracketType.fromNative(nativeStruct.pairedBracketType)

            return BidiMirroringGlyph(mirroringGlyph, mirrored, pairedBracketType)
        }

        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): BidiMirroringGlyph {
            
            val returnVal = lib.icu4x_BidiMirroringGlyph_for_char_mv1(ch);
            val returnStruct = BidiMirroringGlyph.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): BidiMirroringGlyphNative {
        var native = BidiMirroringGlyphNative()
        native.mirroringGlyph = this.mirroringGlyph?.let { OptionInt.some(it) } ?: OptionInt.none()
        native.mirrored = if (this.mirrored) 1 else 0
        native.pairedBracketType = this.pairedBracketType.toNative()
        return native
    }

}
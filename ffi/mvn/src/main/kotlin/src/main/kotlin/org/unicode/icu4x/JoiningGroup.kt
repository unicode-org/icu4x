package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface JoiningGroupLib: Library {
    fun icu4x_JoiningGroup_for_char_mv1(ch: Int): Int
    fun icu4x_JoiningGroup_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_JoiningGroup_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_JoiningGroup_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_JoiningGroup_from_integer_value_mv1(other: FFIUint8): OptionInt
}
/** See the [Rust documentation for `JoiningGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.JoiningGroup.html) for more information.
*/
enum class JoiningGroup {
    NoJoiningGroup,
    Ain,
    Alaph,
    Alef,
    Beh,
    Beth,
    Dal,
    DalathRish,
    E,
    Feh,
    FinalSemkath,
    Gaf,
    Gamal,
    Hah,
    TehMarbutaGoal,
    He,
    Heh,
    HehGoal,
    Heth,
    Kaf,
    Kaph,
    KnottedHeh,
    Lam,
    Lamadh,
    Meem,
    Mim,
    Noon,
    Nun,
    Pe,
    Qaf,
    Qaph,
    Reh,
    ReversedPe,
    Sad,
    Sadhe,
    Seen,
    Semkath,
    Shin,
    SwashKaf,
    SyriacWaw,
    Tah,
    Taw,
    TehMarbuta,
    Teth,
    Waw,
    Yeh,
    YehBarree,
    YehWithTail,
    Yudh,
    YudhHe,
    Zain,
    Fe,
    Khaph,
    Zhain,
    BurushaskiYehBarree,
    FarsiYeh,
    Nya,
    RohingyaYeh,
    ManichaeanAleph,
    ManichaeanAyin,
    ManichaeanBeth,
    ManichaeanDaleth,
    ManichaeanDhamedh,
    ManichaeanFive,
    ManichaeanGimel,
    ManichaeanHeth,
    ManichaeanHundred,
    ManichaeanKaph,
    ManichaeanLamedh,
    ManichaeanMem,
    ManichaeanNun,
    ManichaeanOne,
    ManichaeanPe,
    ManichaeanQoph,
    ManichaeanResh,
    ManichaeanSadhe,
    ManichaeanSamekh,
    ManichaeanTaw,
    ManichaeanTen,
    ManichaeanTeth,
    ManichaeanThamedh,
    ManichaeanTwenty,
    ManichaeanWaw,
    ManichaeanYodh,
    ManichaeanZayin,
    StraightWaw,
    AfricanFeh,
    AfricanNoon,
    AfricanQaf,
    MalayalamBha,
    MalayalamJa,
    MalayalamLla,
    MalayalamLlla,
    MalayalamNga,
    MalayalamNna,
    MalayalamNnna,
    MalayalamNya,
    MalayalamRa,
    MalayalamSsa,
    MalayalamTta,
    HanifiRohingyaKinnaYa,
    HanifiRohingyaPa,
    ThinYeh,
    VerticalTail,
    KashmiriYeh,
    ThinNoon;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<JoiningGroupLib> = JoiningGroupLib::class.java
        internal val lib: JoiningGroupLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): JoiningGroup {
            return JoiningGroup.entries[native]
        }

        fun default(): JoiningGroup {
            return NoJoiningGroup
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): JoiningGroup {
            
            val returnVal = lib.icu4x_JoiningGroup_for_char_mv1(ch);
            return (JoiningGroup.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): JoiningGroup? {
            
            val returnVal = lib.icu4x_JoiningGroup_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return JoiningGroup.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_JoiningGroup_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_JoiningGroup_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_JoiningGroup_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
}
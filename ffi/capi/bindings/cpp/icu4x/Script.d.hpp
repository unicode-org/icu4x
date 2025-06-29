#ifndef icu4x_Script_D_HPP
#define icu4x_Script_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace icu4x {
class Script;
}


namespace icu4x {
namespace capi {
    enum Script {
      Script_Adlam = 167,
      Script_Ahom = 161,
      Script_AnatolianHieroglyphs = 156,
      Script_Arabic = 2,
      Script_Armenian = 3,
      Script_Avestan = 117,
      Script_Balinese = 62,
      Script_Bamum = 130,
      Script_BassaVah = 134,
      Script_Batak = 63,
      Script_Bengali = 4,
      Script_Bhaiksuki = 168,
      Script_Bopomofo = 5,
      Script_Brahmi = 65,
      Script_Braille = 46,
      Script_Buginese = 55,
      Script_Buhid = 44,
      Script_CanadianAboriginal = 40,
      Script_Carian = 104,
      Script_CaucasianAlbanian = 159,
      Script_Chakma = 118,
      Script_Cham = 66,
      Script_Cherokee = 6,
      Script_Chorasmian = 189,
      Script_Common = 0,
      Script_Coptic = 7,
      Script_Cuneiform = 101,
      Script_Cypriot = 47,
      Script_CyproMinoan = 193,
      Script_Cyrillic = 8,
      Script_Deseret = 9,
      Script_Devanagari = 10,
      Script_DivesAkuru = 190,
      Script_Dogra = 178,
      Script_Duployan = 135,
      Script_EgyptianHieroglyphs = 71,
      Script_Elbasan = 136,
      Script_Elymaic = 185,
      Script_Ethiopian = 11,
      Script_Georgian = 12,
      Script_Glagolitic = 56,
      Script_Gothic = 13,
      Script_Grantha = 137,
      Script_Greek = 14,
      Script_Gujarati = 15,
      Script_GunjalaGondi = 179,
      Script_Gurmukhi = 16,
      Script_Han = 17,
      Script_Hangul = 18,
      Script_HanifiRohingya = 182,
      Script_Hanunoo = 43,
      Script_Hatran = 162,
      Script_Hebrew = 19,
      Script_Hiragana = 20,
      Script_ImperialAramaic = 116,
      Script_Inherited = 1,
      Script_InscriptionalPahlavi = 122,
      Script_InscriptionalParthian = 125,
      Script_Javanese = 78,
      Script_Kaithi = 120,
      Script_Kannada = 21,
      Script_Katakana = 22,
      Script_Kawi = 198,
      Script_KayahLi = 79,
      Script_Kharoshthi = 57,
      Script_KhitanSmallScript = 191,
      Script_Khmer = 23,
      Script_Khojki = 157,
      Script_Khudawadi = 145,
      Script_Lao = 24,
      Script_Latin = 25,
      Script_Lepcha = 82,
      Script_Limbu = 48,
      Script_LinearA = 83,
      Script_LinearB = 49,
      Script_Lisu = 131,
      Script_Lycian = 107,
      Script_Lydian = 108,
      Script_Mahajani = 160,
      Script_Makasar = 180,
      Script_Malayalam = 26,
      Script_Mandaic = 84,
      Script_Manichaean = 121,
      Script_Marchen = 169,
      Script_MasaramGondi = 175,
      Script_Medefaidrin = 181,
      Script_MeeteiMayek = 115,
      Script_MendeKikakui = 140,
      Script_MeroiticCursive = 141,
      Script_MeroiticHieroglyphs = 86,
      Script_Miao = 92,
      Script_Modi = 163,
      Script_Mongolian = 27,
      Script_Mro = 149,
      Script_Multani = 164,
      Script_Myanmar = 28,
      Script_Nabataean = 143,
      Script_NagMundari = 199,
      Script_Nandinagari = 187,
      Script_Nastaliq = 200,
      Script_NewTaiLue = 59,
      Script_Newa = 170,
      Script_Nko = 87,
      Script_Nushu = 150,
      Script_NyiakengPuachueHmong = 186,
      Script_Ogham = 29,
      Script_OlChiki = 109,
      Script_OldHungarian = 76,
      Script_OldItalic = 30,
      Script_OldNorthArabian = 142,
      Script_OldPermic = 89,
      Script_OldPersian = 61,
      Script_OldSogdian = 184,
      Script_OldSouthArabian = 133,
      Script_OldTurkic = 88,
      Script_OldUyghur = 194,
      Script_Oriya = 31,
      Script_Osage = 171,
      Script_Osmanya = 50,
      Script_PahawhHmong = 75,
      Script_Palmyrene = 144,
      Script_PauCinHau = 165,
      Script_PhagsPa = 90,
      Script_Phoenician = 91,
      Script_PsalterPahlavi = 123,
      Script_Rejang = 110,
      Script_Runic = 32,
      Script_Samaritan = 126,
      Script_Saurashtra = 111,
      Script_Sharada = 151,
      Script_Shavian = 51,
      Script_Siddham = 166,
      Script_SignWriting = 112,
      Script_Sinhala = 33,
      Script_Sogdian = 183,
      Script_SoraSompeng = 152,
      Script_Soyombo = 176,
      Script_Sundanese = 113,
      Script_SylotiNagri = 58,
      Script_Syriac = 34,
      Script_Tagalog = 42,
      Script_Tagbanwa = 45,
      Script_TaiLe = 52,
      Script_TaiTham = 106,
      Script_TaiViet = 127,
      Script_Takri = 153,
      Script_Tamil = 35,
      Script_Tangsa = 195,
      Script_Tangut = 154,
      Script_Telugu = 36,
      Script_Thaana = 37,
      Script_Thai = 38,
      Script_Tibetan = 39,
      Script_Tifinagh = 60,
      Script_Tirhuta = 158,
      Script_Toto = 196,
      Script_Ugaritic = 53,
      Script_Unknown = 103,
      Script_Vai = 99,
      Script_Vithkuqi = 197,
      Script_Wancho = 188,
      Script_WarangCiti = 146,
      Script_Yezidi = 192,
      Script_Yi = 41,
      Script_ZanabazarSquare = 177,
    };

    typedef struct Script_option {union { Script ok; }; bool is_ok; } Script_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `Script`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.Script.html) for more information.
 */
class Script {
public:
  enum Value {
    Adlam = 167,
    Ahom = 161,
    AnatolianHieroglyphs = 156,
    Arabic = 2,
    Armenian = 3,
    Avestan = 117,
    Balinese = 62,
    Bamum = 130,
    BassaVah = 134,
    Batak = 63,
    Bengali = 4,
    Bhaiksuki = 168,
    Bopomofo = 5,
    Brahmi = 65,
    Braille = 46,
    Buginese = 55,
    Buhid = 44,
    CanadianAboriginal = 40,
    Carian = 104,
    CaucasianAlbanian = 159,
    Chakma = 118,
    Cham = 66,
    Cherokee = 6,
    Chorasmian = 189,
    Common = 0,
    Coptic = 7,
    Cuneiform = 101,
    Cypriot = 47,
    CyproMinoan = 193,
    Cyrillic = 8,
    Deseret = 9,
    Devanagari = 10,
    DivesAkuru = 190,
    Dogra = 178,
    Duployan = 135,
    EgyptianHieroglyphs = 71,
    Elbasan = 136,
    Elymaic = 185,
    Ethiopian = 11,
    Georgian = 12,
    Glagolitic = 56,
    Gothic = 13,
    Grantha = 137,
    Greek = 14,
    Gujarati = 15,
    GunjalaGondi = 179,
    Gurmukhi = 16,
    Han = 17,
    Hangul = 18,
    HanifiRohingya = 182,
    Hanunoo = 43,
    Hatran = 162,
    Hebrew = 19,
    Hiragana = 20,
    ImperialAramaic = 116,
    Inherited = 1,
    InscriptionalPahlavi = 122,
    InscriptionalParthian = 125,
    Javanese = 78,
    Kaithi = 120,
    Kannada = 21,
    Katakana = 22,
    Kawi = 198,
    KayahLi = 79,
    Kharoshthi = 57,
    KhitanSmallScript = 191,
    Khmer = 23,
    Khojki = 157,
    Khudawadi = 145,
    Lao = 24,
    Latin = 25,
    Lepcha = 82,
    Limbu = 48,
    LinearA = 83,
    LinearB = 49,
    Lisu = 131,
    Lycian = 107,
    Lydian = 108,
    Mahajani = 160,
    Makasar = 180,
    Malayalam = 26,
    Mandaic = 84,
    Manichaean = 121,
    Marchen = 169,
    MasaramGondi = 175,
    Medefaidrin = 181,
    MeeteiMayek = 115,
    MendeKikakui = 140,
    MeroiticCursive = 141,
    MeroiticHieroglyphs = 86,
    Miao = 92,
    Modi = 163,
    Mongolian = 27,
    Mro = 149,
    Multani = 164,
    Myanmar = 28,
    Nabataean = 143,
    NagMundari = 199,
    Nandinagari = 187,
    Nastaliq = 200,
    NewTaiLue = 59,
    Newa = 170,
    Nko = 87,
    Nushu = 150,
    NyiakengPuachueHmong = 186,
    Ogham = 29,
    OlChiki = 109,
    OldHungarian = 76,
    OldItalic = 30,
    OldNorthArabian = 142,
    OldPermic = 89,
    OldPersian = 61,
    OldSogdian = 184,
    OldSouthArabian = 133,
    OldTurkic = 88,
    OldUyghur = 194,
    Oriya = 31,
    Osage = 171,
    Osmanya = 50,
    PahawhHmong = 75,
    Palmyrene = 144,
    PauCinHau = 165,
    PhagsPa = 90,
    Phoenician = 91,
    PsalterPahlavi = 123,
    Rejang = 110,
    Runic = 32,
    Samaritan = 126,
    Saurashtra = 111,
    Sharada = 151,
    Shavian = 51,
    Siddham = 166,
    SignWriting = 112,
    Sinhala = 33,
    Sogdian = 183,
    SoraSompeng = 152,
    Soyombo = 176,
    Sundanese = 113,
    SylotiNagri = 58,
    Syriac = 34,
    Tagalog = 42,
    Tagbanwa = 45,
    TaiLe = 52,
    TaiTham = 106,
    TaiViet = 127,
    Takri = 153,
    Tamil = 35,
    Tangsa = 195,
    Tangut = 154,
    Telugu = 36,
    Thaana = 37,
    Thai = 38,
    Tibetan = 39,
    Tifinagh = 60,
    Tirhuta = 158,
    Toto = 196,
    Ugaritic = 53,
    Unknown = 103,
    Vai = 99,
    Vithkuqi = 197,
    Wancho = 188,
    WarangCiti = 146,
    Yezidi = 192,
    Yi = 41,
    ZanabazarSquare = 177,
  };

  Script(): value(Value::Unknown) {}

  // Implicit conversions between enum and ::Value
  constexpr Script(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  /**
   * See the [Rust documentation for `for_char`](https://docs.rs/icu/2.0.0/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
   */
  inline static icu4x::Script for_char(char32_t ch);

  /**
   * Get the "long" name of this property value (returns empty if property value is unknown)
   *
   * See the [Rust documentation for `get`](https://docs.rs/icu/2.0.0/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
   */
  inline std::optional<std::string_view> long_name() const;

  /**
   * Get the "short" name of this property value (returns empty if property value is unknown)
   *
   * See the [Rust documentation for `get`](https://docs.rs/icu/2.0.0/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
   */
  inline std::optional<std::string_view> short_name() const;

  /**
   * Convert to an integer value usable with ICU4C and CodePointMapData
   *
   * See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.Script.html#method.to_icu4c_value) for more information.
   */
  inline uint16_t to_integer_value() const;

  /**
   * Convert from an integer value from ICU4C or CodePointMapData
   *
   * See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.Script.html#method.from_icu4c_value) for more information.
   */
  inline static std::optional<icu4x::Script> from_integer_value(uint16_t other);

  inline icu4x::capi::Script AsFFI() const;
  inline static icu4x::Script FromFFI(icu4x::capi::Script c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_Script_D_HPP

#ifndef ICU4X_JoiningGroup_D_HPP
#define ICU4X_JoiningGroup_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace icu4x {
class JoiningGroup;
} // namespace icu4x



namespace icu4x {
namespace capi {
    enum JoiningGroup {
      JoiningGroup_NoJoiningGroup = 0,
      JoiningGroup_Ain = 1,
      JoiningGroup_Alaph = 2,
      JoiningGroup_Alef = 3,
      JoiningGroup_Beh = 4,
      JoiningGroup_Beth = 5,
      JoiningGroup_Dal = 6,
      JoiningGroup_DalathRish = 7,
      JoiningGroup_E = 8,
      JoiningGroup_Feh = 9,
      JoiningGroup_FinalSemkath = 10,
      JoiningGroup_Gaf = 11,
      JoiningGroup_Gamal = 12,
      JoiningGroup_Hah = 13,
      JoiningGroup_TehMarbutaGoal = 14,
      JoiningGroup_He = 15,
      JoiningGroup_Heh = 16,
      JoiningGroup_HehGoal = 17,
      JoiningGroup_Heth = 18,
      JoiningGroup_Kaf = 19,
      JoiningGroup_Kaph = 20,
      JoiningGroup_KnottedHeh = 21,
      JoiningGroup_Lam = 22,
      JoiningGroup_Lamadh = 23,
      JoiningGroup_Meem = 24,
      JoiningGroup_Mim = 25,
      JoiningGroup_Noon = 26,
      JoiningGroup_Nun = 27,
      JoiningGroup_Pe = 28,
      JoiningGroup_Qaf = 29,
      JoiningGroup_Qaph = 30,
      JoiningGroup_Reh = 31,
      JoiningGroup_ReversedPe = 32,
      JoiningGroup_Sad = 33,
      JoiningGroup_Sadhe = 34,
      JoiningGroup_Seen = 35,
      JoiningGroup_Semkath = 36,
      JoiningGroup_Shin = 37,
      JoiningGroup_SwashKaf = 38,
      JoiningGroup_SyriacWaw = 39,
      JoiningGroup_Tah = 40,
      JoiningGroup_Taw = 41,
      JoiningGroup_TehMarbuta = 42,
      JoiningGroup_Teth = 43,
      JoiningGroup_Waw = 44,
      JoiningGroup_Yeh = 45,
      JoiningGroup_YehBarree = 46,
      JoiningGroup_YehWithTail = 47,
      JoiningGroup_Yudh = 48,
      JoiningGroup_YudhHe = 49,
      JoiningGroup_Zain = 50,
      JoiningGroup_Fe = 51,
      JoiningGroup_Khaph = 52,
      JoiningGroup_Zhain = 53,
      JoiningGroup_BurushaskiYehBarree = 54,
      JoiningGroup_FarsiYeh = 55,
      JoiningGroup_Nya = 56,
      JoiningGroup_RohingyaYeh = 57,
      JoiningGroup_ManichaeanAleph = 58,
      JoiningGroup_ManichaeanAyin = 59,
      JoiningGroup_ManichaeanBeth = 60,
      JoiningGroup_ManichaeanDaleth = 61,
      JoiningGroup_ManichaeanDhamedh = 62,
      JoiningGroup_ManichaeanFive = 63,
      JoiningGroup_ManichaeanGimel = 64,
      JoiningGroup_ManichaeanHeth = 65,
      JoiningGroup_ManichaeanHundred = 66,
      JoiningGroup_ManichaeanKaph = 67,
      JoiningGroup_ManichaeanLamedh = 68,
      JoiningGroup_ManichaeanMem = 69,
      JoiningGroup_ManichaeanNun = 70,
      JoiningGroup_ManichaeanOne = 71,
      JoiningGroup_ManichaeanPe = 72,
      JoiningGroup_ManichaeanQoph = 73,
      JoiningGroup_ManichaeanResh = 74,
      JoiningGroup_ManichaeanSadhe = 75,
      JoiningGroup_ManichaeanSamekh = 76,
      JoiningGroup_ManichaeanTaw = 77,
      JoiningGroup_ManichaeanTen = 78,
      JoiningGroup_ManichaeanTeth = 79,
      JoiningGroup_ManichaeanThamedh = 80,
      JoiningGroup_ManichaeanTwenty = 81,
      JoiningGroup_ManichaeanWaw = 82,
      JoiningGroup_ManichaeanYodh = 83,
      JoiningGroup_ManichaeanZayin = 84,
      JoiningGroup_StraightWaw = 85,
      JoiningGroup_AfricanFeh = 86,
      JoiningGroup_AfricanNoon = 87,
      JoiningGroup_AfricanQaf = 88,
      JoiningGroup_MalayalamBha = 89,
      JoiningGroup_MalayalamJa = 90,
      JoiningGroup_MalayalamLla = 91,
      JoiningGroup_MalayalamLlla = 92,
      JoiningGroup_MalayalamNga = 93,
      JoiningGroup_MalayalamNna = 94,
      JoiningGroup_MalayalamNnna = 95,
      JoiningGroup_MalayalamNya = 96,
      JoiningGroup_MalayalamRa = 97,
      JoiningGroup_MalayalamSsa = 98,
      JoiningGroup_MalayalamTta = 99,
      JoiningGroup_HanifiRohingyaKinnaYa = 100,
      JoiningGroup_HanifiRohingyaPa = 101,
      JoiningGroup_ThinYeh = 102,
      JoiningGroup_VerticalTail = 103,
      JoiningGroup_KashmiriYeh = 104,
      JoiningGroup_ThinNoon = 105,
    };

    typedef struct JoiningGroup_option {union { JoiningGroup ok; }; bool is_ok; } JoiningGroup_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `JoiningGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.JoiningGroup.html) for more information.
 */
class JoiningGroup {
public:
    enum Value {
        NoJoiningGroup = 0,
        Ain = 1,
        Alaph = 2,
        Alef = 3,
        Beh = 4,
        Beth = 5,
        Dal = 6,
        DalathRish = 7,
        E = 8,
        Feh = 9,
        FinalSemkath = 10,
        Gaf = 11,
        Gamal = 12,
        Hah = 13,
        TehMarbutaGoal = 14,
        He = 15,
        Heh = 16,
        HehGoal = 17,
        Heth = 18,
        Kaf = 19,
        Kaph = 20,
        KnottedHeh = 21,
        Lam = 22,
        Lamadh = 23,
        Meem = 24,
        Mim = 25,
        Noon = 26,
        Nun = 27,
        Pe = 28,
        Qaf = 29,
        Qaph = 30,
        Reh = 31,
        ReversedPe = 32,
        Sad = 33,
        Sadhe = 34,
        Seen = 35,
        Semkath = 36,
        Shin = 37,
        SwashKaf = 38,
        SyriacWaw = 39,
        Tah = 40,
        Taw = 41,
        TehMarbuta = 42,
        Teth = 43,
        Waw = 44,
        Yeh = 45,
        YehBarree = 46,
        YehWithTail = 47,
        Yudh = 48,
        YudhHe = 49,
        Zain = 50,
        Fe = 51,
        Khaph = 52,
        Zhain = 53,
        BurushaskiYehBarree = 54,
        FarsiYeh = 55,
        Nya = 56,
        RohingyaYeh = 57,
        ManichaeanAleph = 58,
        ManichaeanAyin = 59,
        ManichaeanBeth = 60,
        ManichaeanDaleth = 61,
        ManichaeanDhamedh = 62,
        ManichaeanFive = 63,
        ManichaeanGimel = 64,
        ManichaeanHeth = 65,
        ManichaeanHundred = 66,
        ManichaeanKaph = 67,
        ManichaeanLamedh = 68,
        ManichaeanMem = 69,
        ManichaeanNun = 70,
        ManichaeanOne = 71,
        ManichaeanPe = 72,
        ManichaeanQoph = 73,
        ManichaeanResh = 74,
        ManichaeanSadhe = 75,
        ManichaeanSamekh = 76,
        ManichaeanTaw = 77,
        ManichaeanTen = 78,
        ManichaeanTeth = 79,
        ManichaeanThamedh = 80,
        ManichaeanTwenty = 81,
        ManichaeanWaw = 82,
        ManichaeanYodh = 83,
        ManichaeanZayin = 84,
        StraightWaw = 85,
        AfricanFeh = 86,
        AfricanNoon = 87,
        AfricanQaf = 88,
        MalayalamBha = 89,
        MalayalamJa = 90,
        MalayalamLla = 91,
        MalayalamLlla = 92,
        MalayalamNga = 93,
        MalayalamNna = 94,
        MalayalamNnna = 95,
        MalayalamNya = 96,
        MalayalamRa = 97,
        MalayalamSsa = 98,
        MalayalamTta = 99,
        HanifiRohingyaKinnaYa = 100,
        HanifiRohingyaPa = 101,
        ThinYeh = 102,
        VerticalTail = 103,
        KashmiriYeh = 104,
        ThinNoon = 105,
    };

    JoiningGroup(): value(Value::NoJoiningGroup) {}

    // Implicit conversions between enum and ::Value
    constexpr JoiningGroup(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

  /**
   * See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
   */
  inline static icu4x::JoiningGroup for_char(char32_t ch);

  /**
   * Get the "long" name of this property value (returns empty if property value is unknown)
   *
   * See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
   */
  inline std::optional<std::string_view> long_name() const;

  /**
   * Get the "short" name of this property value (returns empty if property value is unknown)
   *
   * See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
   */
  inline std::optional<std::string_view> short_name() const;

  /**
   * Convert to an integer value usable with ICU4C and CodePointMapData
   *
   * See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html#method.to_icu4c_value) for more information.
   */
  inline uint8_t to_integer_value() const;

  /**
   * Convert from an integer value from ICU4C or CodePointMapData
   *
   * See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.JoiningGroup.html#method.from_icu4c_value) for more information.
   */
  inline static std::optional<icu4x::JoiningGroup> from_integer_value(uint8_t other);

    inline icu4x::capi::JoiningGroup AsFFI() const;
    inline static icu4x::JoiningGroup FromFFI(icu4x::capi::JoiningGroup c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_JoiningGroup_D_HPP

#ifndef ICU4XAnyCalendarKind_D_HPP
#define ICU4XAnyCalendarKind_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class ICU4XLocale;


namespace capi {
    typedef enum ICU4XAnyCalendarKind {
      ICU4XAnyCalendarKind_Iso = 0,
      ICU4XAnyCalendarKind_Gregorian = 1,
      ICU4XAnyCalendarKind_Buddhist = 2,
      ICU4XAnyCalendarKind_Japanese = 3,
      ICU4XAnyCalendarKind_JapaneseExtended = 4,
      ICU4XAnyCalendarKind_Ethiopian = 5,
      ICU4XAnyCalendarKind_EthiopianAmeteAlem = 6,
      ICU4XAnyCalendarKind_Indian = 7,
      ICU4XAnyCalendarKind_Coptic = 8,
      ICU4XAnyCalendarKind_Dangi = 9,
      ICU4XAnyCalendarKind_Chinese = 10,
      ICU4XAnyCalendarKind_Hebrew = 11,
      ICU4XAnyCalendarKind_IslamicCivil = 12,
      ICU4XAnyCalendarKind_IslamicObservational = 13,
      ICU4XAnyCalendarKind_IslamicTabular = 14,
      ICU4XAnyCalendarKind_IslamicUmmAlQura = 15,
      ICU4XAnyCalendarKind_Persian = 16,
      ICU4XAnyCalendarKind_Roc = 17,
    } ICU4XAnyCalendarKind;
}

class ICU4XAnyCalendarKind {
public:
  enum Value {
    Iso = 0,
    Gregorian = 1,
    Buddhist = 2,
    Japanese = 3,
    JapaneseExtended = 4,
    Ethiopian = 5,
    EthiopianAmeteAlem = 6,
    Indian = 7,
    Coptic = 8,
    Dangi = 9,
    Chinese = 10,
    Hebrew = 11,
    IslamicCivil = 12,
    IslamicObservational = 13,
    IslamicTabular = 14,
    IslamicUmmAlQura = 15,
    Persian = 16,
    Roc = 17,
  };

  ICU4XAnyCalendarKind() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XAnyCalendarKind(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline static std::optional<ICU4XAnyCalendarKind> get_for_locale(const ICU4XLocale& locale);

  inline static std::optional<ICU4XAnyCalendarKind> get_for_bcp47(std::string_view s);

  inline std::string bcp47();

  inline capi::ICU4XAnyCalendarKind AsFFI() const;
  inline static ICU4XAnyCalendarKind FromFFI(capi::ICU4XAnyCalendarKind c_enum);
private:
    Value value;
};


#endif // ICU4XAnyCalendarKind_D_HPP

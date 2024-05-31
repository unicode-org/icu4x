#ifndef ICU4XAnyCalendarKind_D_HPP
#define ICU4XAnyCalendarKind_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XAnyCalendarKind.d.h"

class ICU4XLocale;


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

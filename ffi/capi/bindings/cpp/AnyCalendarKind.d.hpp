#ifndef AnyCalendarKind_D_HPP
#define AnyCalendarKind_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Locale; }
class Locale;


namespace diplomat {
namespace capi {
    enum AnyCalendarKind {
      AnyCalendarKind_Iso = 0,
      AnyCalendarKind_Gregorian = 1,
      AnyCalendarKind_Buddhist = 2,
      AnyCalendarKind_Japanese = 3,
      AnyCalendarKind_JapaneseExtended = 4,
      AnyCalendarKind_Ethiopian = 5,
      AnyCalendarKind_EthiopianAmeteAlem = 6,
      AnyCalendarKind_Indian = 7,
      AnyCalendarKind_Coptic = 8,
      AnyCalendarKind_Dangi = 9,
      AnyCalendarKind_Chinese = 10,
      AnyCalendarKind_Hebrew = 11,
      AnyCalendarKind_IslamicCivil = 12,
      AnyCalendarKind_IslamicObservational = 13,
      AnyCalendarKind_IslamicTabular = 14,
      AnyCalendarKind_IslamicUmmAlQura = 15,
      AnyCalendarKind_Persian = 16,
      AnyCalendarKind_Roc = 17,
    };
} // namespace capi
} // namespace

class AnyCalendarKind {
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

  AnyCalendarKind() = default;
  // Implicit conversions between enum and ::Value
  constexpr AnyCalendarKind(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline static std::optional<AnyCalendarKind> get_for_locale(const Locale& locale);

  inline static std::optional<AnyCalendarKind> get_for_bcp47(std::string_view s);

  inline std::string bcp47();

  inline diplomat::capi::AnyCalendarKind AsFFI() const;
  inline static AnyCalendarKind FromFFI(diplomat::capi::AnyCalendarKind c_enum);
private:
    Value value;
};


#endif // AnyCalendarKind_D_HPP

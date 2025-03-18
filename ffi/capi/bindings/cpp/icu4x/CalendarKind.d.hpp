#ifndef icu4x_CalendarKind_D_HPP
#define icu4x_CalendarKind_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CalendarKind {
      CalendarKind_Iso = 0,
      CalendarKind_Gregorian = 1,
      CalendarKind_Buddhist = 2,
      CalendarKind_Japanese = 3,
      CalendarKind_JapaneseExtended = 4,
      CalendarKind_Ethiopian = 5,
      CalendarKind_EthiopianAmeteAlem = 6,
      CalendarKind_Indian = 7,
      CalendarKind_Coptic = 8,
      CalendarKind_Dangi = 9,
      CalendarKind_Chinese = 10,
      CalendarKind_Hebrew = 11,
      CalendarKind_HijriCivil = 12,
      CalendarKind_HijriObservational = 13,
      CalendarKind_HijriTabular = 14,
      CalendarKind_HijriUmmAlQura = 15,
      CalendarKind_Persian = 16,
      CalendarKind_Roc = 17,
    };
    
    typedef struct CalendarKind_option {union { CalendarKind ok; }; bool is_ok; } CalendarKind_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * The various calendar types currently supported by [`Calendar`]
 *
 * See the [Rust documentation for `AnyCalendarKind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendarKind.html) for more information.
 */
class CalendarKind {
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
    HijriCivil = 12,
    HijriObservational = 13,
    HijriTabular = 14,
    HijriUmmAlQura = 15,
    Persian = 16,
    Roc = 17,
  };

  CalendarKind() = default;
  // Implicit conversions between enum and ::Value
  constexpr CalendarKind(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::CalendarKind AsFFI() const;
  inline static icu4x::CalendarKind FromFFI(icu4x::capi::CalendarKind c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CalendarKind_D_HPP

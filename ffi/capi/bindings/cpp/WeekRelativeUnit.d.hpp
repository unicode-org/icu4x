#ifndef WeekRelativeUnit_D_HPP
#define WeekRelativeUnit_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum WeekRelativeUnit {
      WeekRelativeUnit_Previous = 0,
      WeekRelativeUnit_Current = 1,
      WeekRelativeUnit_Next = 2,
    } WeekRelativeUnit;
} // namespace capi
} // namespace

class WeekRelativeUnit {
public:
  enum Value {
    Previous = 0,
    Current = 1,
    Next = 2,
  };

  WeekRelativeUnit() = default;
  // Implicit conversions between enum and ::Value
  constexpr WeekRelativeUnit(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::WeekRelativeUnit AsFFI() const;
  inline static WeekRelativeUnit FromFFI(diplomat::capi::WeekRelativeUnit c_enum);
private:
    Value value;
};


#endif // WeekRelativeUnit_D_HPP

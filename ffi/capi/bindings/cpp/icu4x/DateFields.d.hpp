#ifndef icu4x_DateFields_D_HPP
#define icu4x_DateFields_D_HPP

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
    enum DateFields {
      DateFields_D = 0,
      DateFields_MD = 1,
      DateFields_YMD = 2,
      DateFields_DE = 3,
      DateFields_MDE = 4,
      DateFields_YMDE = 5,
      DateFields_E = 6,
      DateFields_M = 7,
      DateFields_YM = 8,
      DateFields_Y = 9,
    };
    
    typedef struct DateFields_option {union { DateFields ok; }; bool is_ok; } DateFields_option;
} // namespace capi
} // namespace

namespace icu4x {
class DateFields {
public:
  enum Value {
    D = 0,
    MD = 1,
    YMD = 2,
    DE = 3,
    MDE = 4,
    YMDE = 5,
    E = 6,
    M = 7,
    YM = 8,
    Y = 9,
  };

  DateFields() = default;
  // Implicit conversions between enum and ::Value
  constexpr DateFields(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::DateFields AsFFI() const;
  inline static icu4x::DateFields FromFFI(icu4x::capi::DateFields c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_DateFields_D_HPP

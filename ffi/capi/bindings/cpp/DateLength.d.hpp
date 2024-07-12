#ifndef DateLength_D_HPP
#define DateLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum DateLength {
      DateLength_Full = 0,
      DateLength_Long = 1,
      DateLength_Medium = 2,
      DateLength_Short = 3,
    };
} // namespace capi
} // namespace

class DateLength {
public:
  enum Value {
    Full = 0,
    Long = 1,
    Medium = 2,
    Short = 3,
  };

  DateLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr DateLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::DateLength AsFFI() const;
  inline static DateLength FromFFI(diplomat::capi::DateLength c_enum);
private:
    Value value;
};


#endif // DateLength_D_HPP

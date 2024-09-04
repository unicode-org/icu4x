#ifndef icu4x_DateLength_D_HPP
#define icu4x_DateLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum DateLength {
      DateLength_Full = 0,
      DateLength_Long = 1,
      DateLength_Medium = 2,
      DateLength_Short = 3,
    };
    
    typedef struct DateLength_option {union { DateLength ok; }; bool is_ok; } DateLength_option;
} // namespace capi
} // namespace

namespace icu4x {
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

  inline icu4x::capi::DateLength AsFFI() const;
  inline static icu4x::DateLength FromFFI(icu4x::capi::DateLength c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_DateLength_D_HPP

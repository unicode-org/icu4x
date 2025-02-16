#ifndef icu4x_TimePrecision_D_HPP
#define icu4x_TimePrecision_D_HPP

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
    enum TimePrecision {
      TimePrecision_Hour = 0,
      TimePrecision_Minute = 1,
      TimePrecision_MinuteOptional = 2,
      TimePrecision_Second = 3,
      TimePrecision_SecondS1 = 4,
      TimePrecision_SecondS2 = 5,
      TimePrecision_SecondS3 = 6,
      TimePrecision_SecondS4 = 7,
      TimePrecision_SecondS5 = 8,
      TimePrecision_SecondS6 = 9,
      TimePrecision_SecondS7 = 10,
      TimePrecision_SecondS8 = 11,
      TimePrecision_SecondS9 = 12,
    };
    
    typedef struct TimePrecision_option {union { TimePrecision ok; }; bool is_ok; } TimePrecision_option;
} // namespace capi
} // namespace

namespace icu4x {
class TimePrecision {
public:
  enum Value {
    Hour = 0,
    Minute = 1,
    MinuteOptional = 2,
    Second = 3,
    SecondS1 = 4,
    SecondS2 = 5,
    SecondS3 = 6,
    SecondS4 = 7,
    SecondS5 = 8,
    SecondS6 = 9,
    SecondS7 = 10,
    SecondS8 = 11,
    SecondS9 = 12,
  };

  TimePrecision() = default;
  // Implicit conversions between enum and ::Value
  constexpr TimePrecision(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::TimePrecision AsFFI() const;
  inline static icu4x::TimePrecision FromFFI(icu4x::capi::TimePrecision c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_TimePrecision_D_HPP

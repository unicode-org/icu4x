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
      TimePrecision_SecondF1 = 4,
      TimePrecision_SecondF2 = 5,
      TimePrecision_SecondF3 = 6,
      TimePrecision_SecondF4 = 7,
      TimePrecision_SecondF5 = 8,
      TimePrecision_SecondF6 = 9,
      TimePrecision_SecondF7 = 10,
      TimePrecision_SecondF8 = 11,
      TimePrecision_SecondF9 = 12,
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
    SecondF1 = 4,
    SecondF2 = 5,
    SecondF3 = 6,
    SecondF4 = 7,
    SecondF5 = 8,
    SecondF6 = 9,
    SecondF7 = 10,
    SecondF8 = 11,
    SecondF9 = 12,
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

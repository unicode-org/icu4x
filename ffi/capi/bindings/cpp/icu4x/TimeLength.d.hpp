#ifndef icu4x_TimeLength_D_HPP
#define icu4x_TimeLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum TimeLength {
      TimeLength_Full = 0,
      TimeLength_Long = 1,
      TimeLength_Medium = 2,
      TimeLength_Short = 3,
    };
    
    typedef struct TimeLength_option {union { TimeLength ok; }; bool is_ok; } TimeLength_option;
} // namespace capi
} // namespace

namespace icu4x {
class TimeLength {
public:
  enum Value {
    Full = 0,
    Long = 1,
    Medium = 2,
    Short = 3,
  };

  TimeLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr TimeLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::TimeLength AsFFI() const;
  inline static icu4x::TimeLength FromFFI(icu4x::capi::TimeLength c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_TimeLength_D_HPP

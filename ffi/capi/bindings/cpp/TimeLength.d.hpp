#ifndef TimeLength_D_HPP
#define TimeLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum TimeLength {
      TimeLength_Full = 0,
      TimeLength_Long = 1,
      TimeLength_Medium = 2,
      TimeLength_Short = 3,
    } TimeLength;
} // namespace capi
} // namespace

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

  inline diplomat::capi::TimeLength AsFFI() const;
  inline static TimeLength FromFFI(diplomat::capi::TimeLength c_enum);
private:
    Value value;
};


#endif // TimeLength_D_HPP

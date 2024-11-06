#ifndef icu4x_FixedDecimalRoundingMode_D_HPP
#define icu4x_FixedDecimalRoundingMode_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum FixedDecimalRoundingMode {
      FixedDecimalRoundingMode_Ceil = 0,
      FixedDecimalRoundingMode_Floor = 1,
      FixedDecimalRoundingMode_HalfCeil = 2,
      FixedDecimalRoundingMode_HalfFloor = 3,
    };
    
    typedef struct FixedDecimalRoundingMode_option {union { FixedDecimalRoundingMode ok; }; bool is_ok; } FixedDecimalRoundingMode_option;
} // namespace capi
} // namespace

namespace icu4x {
class FixedDecimalRoundingMode {
public:
  enum Value {
    Ceil = 0,
    Floor = 1,
    HalfCeil = 2,
    HalfFloor = 3,
  };

  FixedDecimalRoundingMode() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalRoundingMode(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::FixedDecimalRoundingMode AsFFI() const;
  inline static icu4x::FixedDecimalRoundingMode FromFFI(icu4x::capi::FixedDecimalRoundingMode c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_FixedDecimalRoundingMode_D_HPP

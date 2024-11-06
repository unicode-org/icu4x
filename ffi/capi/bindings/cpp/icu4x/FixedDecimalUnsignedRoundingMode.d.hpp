#ifndef icu4x_FixedDecimalUnsignedRoundingMode_D_HPP
#define icu4x_FixedDecimalUnsignedRoundingMode_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum FixedDecimalUnsignedRoundingMode {
      FixedDecimalUnsignedRoundingMode_Expand = 0,
      FixedDecimalUnsignedRoundingMode_Trunc = 1,
      FixedDecimalUnsignedRoundingMode_HalfExpand = 2,
      FixedDecimalUnsignedRoundingMode_HalfTrunc = 3,
      FixedDecimalUnsignedRoundingMode_HalfEven = 4,
    };
    
    typedef struct FixedDecimalUnsignedRoundingMode_option {union { FixedDecimalUnsignedRoundingMode ok; }; bool is_ok; } FixedDecimalUnsignedRoundingMode_option;
} // namespace capi
} // namespace

namespace icu4x {
class FixedDecimalUnsignedRoundingMode {
public:
  enum Value {
    Expand = 0,
    Trunc = 1,
    HalfExpand = 2,
    HalfTrunc = 3,
    HalfEven = 4,
  };

  FixedDecimalUnsignedRoundingMode() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalUnsignedRoundingMode(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::FixedDecimalUnsignedRoundingMode AsFFI() const;
  inline static icu4x::FixedDecimalUnsignedRoundingMode FromFFI(icu4x::capi::FixedDecimalUnsignedRoundingMode c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_FixedDecimalUnsignedRoundingMode_D_HPP

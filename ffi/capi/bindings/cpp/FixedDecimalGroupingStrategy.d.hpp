#ifndef FixedDecimalGroupingStrategy_D_HPP
#define FixedDecimalGroupingStrategy_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum FixedDecimalGroupingStrategy {
      FixedDecimalGroupingStrategy_Auto = 0,
      FixedDecimalGroupingStrategy_Never = 1,
      FixedDecimalGroupingStrategy_Always = 2,
      FixedDecimalGroupingStrategy_Min2 = 3,
    } FixedDecimalGroupingStrategy;
}

class FixedDecimalGroupingStrategy {
public:
  enum Value {
    Auto = 0,
    Never = 1,
    Always = 2,
    Min2 = 3,
  };

  FixedDecimalGroupingStrategy() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalGroupingStrategy(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::FixedDecimalGroupingStrategy AsFFI() const;
  inline static FixedDecimalGroupingStrategy FromFFI(capi::FixedDecimalGroupingStrategy c_enum);
private:
    Value value;
};


#endif // FixedDecimalGroupingStrategy_D_HPP

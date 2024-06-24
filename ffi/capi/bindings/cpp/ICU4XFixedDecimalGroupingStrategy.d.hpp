#ifndef ICU4XFixedDecimalGroupingStrategy_D_HPP
#define ICU4XFixedDecimalGroupingStrategy_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XFixedDecimalGroupingStrategy {
      ICU4XFixedDecimalGroupingStrategy_Auto = 0,
      ICU4XFixedDecimalGroupingStrategy_Never = 1,
      ICU4XFixedDecimalGroupingStrategy_Always = 2,
      ICU4XFixedDecimalGroupingStrategy_Min2 = 3,
    } ICU4XFixedDecimalGroupingStrategy;
}

class ICU4XFixedDecimalGroupingStrategy {
public:
  enum Value {
    Auto = 0,
    Never = 1,
    Always = 2,
    Min2 = 3,
  };

  ICU4XFixedDecimalGroupingStrategy() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XFixedDecimalGroupingStrategy(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XFixedDecimalGroupingStrategy AsFFI() const;
  inline static ICU4XFixedDecimalGroupingStrategy FromFFI(capi::ICU4XFixedDecimalGroupingStrategy c_enum);
private:
    Value value;
};


#endif // ICU4XFixedDecimalGroupingStrategy_D_HPP

#ifndef ICU4XFixedDecimalGroupingStrategy_HPP
#define ICU4XFixedDecimalGroupingStrategy_HPP

#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.h"


inline capi::ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategy::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalGroupingStrategy>(value);
}

inline ICU4XFixedDecimalGroupingStrategy ICU4XFixedDecimalGroupingStrategy::FromFFI(capi::ICU4XFixedDecimalGroupingStrategy c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalGroupingStrategy_Auto:
    case capi::ICU4XFixedDecimalGroupingStrategy_Never:
    case capi::ICU4XFixedDecimalGroupingStrategy_Always:
    case capi::ICU4XFixedDecimalGroupingStrategy_Min2:
      return static_cast<ICU4XFixedDecimalGroupingStrategy::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalGroupingStrategy_HPP

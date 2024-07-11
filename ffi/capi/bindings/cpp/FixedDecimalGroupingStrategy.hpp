#ifndef FixedDecimalGroupingStrategy_HPP
#define FixedDecimalGroupingStrategy_HPP

#include "FixedDecimalGroupingStrategy.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::FixedDecimalGroupingStrategy FixedDecimalGroupingStrategy::AsFFI() const {
  return static_cast<capi::FixedDecimalGroupingStrategy>(value);
}

inline FixedDecimalGroupingStrategy FixedDecimalGroupingStrategy::FromFFI(capi::FixedDecimalGroupingStrategy c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalGroupingStrategy_Auto:
    case capi::FixedDecimalGroupingStrategy_Never:
    case capi::FixedDecimalGroupingStrategy_Always:
    case capi::FixedDecimalGroupingStrategy_Min2:
      return static_cast<FixedDecimalGroupingStrategy::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalGroupingStrategy_HPP

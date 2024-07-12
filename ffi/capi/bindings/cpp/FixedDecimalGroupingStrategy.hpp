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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::FixedDecimalGroupingStrategy FixedDecimalGroupingStrategy::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalGroupingStrategy>(value);
}

inline FixedDecimalGroupingStrategy FixedDecimalGroupingStrategy::FromFFI(diplomat::capi::FixedDecimalGroupingStrategy c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalGroupingStrategy_Auto:
    case diplomat::capi::FixedDecimalGroupingStrategy_Never:
    case diplomat::capi::FixedDecimalGroupingStrategy_Always:
    case diplomat::capi::FixedDecimalGroupingStrategy_Min2:
      return static_cast<FixedDecimalGroupingStrategy::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalGroupingStrategy_HPP

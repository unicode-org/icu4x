#ifndef ICU4XLeadingAdjustment_HPP
#define ICU4XLeadingAdjustment_HPP

#include "ICU4XLeadingAdjustment.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLeadingAdjustment.h"


inline capi::ICU4XLeadingAdjustment ICU4XLeadingAdjustment::AsFFI() const {
  return static_cast<capi::ICU4XLeadingAdjustment>(value);
}

inline ICU4XLeadingAdjustment ICU4XLeadingAdjustment::FromFFI(capi::ICU4XLeadingAdjustment c_enum) {
  switch (c_enum) {
    case capi::ICU4XLeadingAdjustment_Auto:
    case capi::ICU4XLeadingAdjustment_None:
    case capi::ICU4XLeadingAdjustment_ToCased:
      return static_cast<ICU4XLeadingAdjustment::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLeadingAdjustment_HPP

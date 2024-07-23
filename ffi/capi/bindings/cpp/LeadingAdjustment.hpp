#ifndef LeadingAdjustment_HPP
#define LeadingAdjustment_HPP

#include "LeadingAdjustment.d.hpp"

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

inline diplomat::capi::LeadingAdjustment LeadingAdjustment::AsFFI() const {
  return static_cast<diplomat::capi::LeadingAdjustment>(value);
}

inline LeadingAdjustment LeadingAdjustment::FromFFI(diplomat::capi::LeadingAdjustment c_enum) {
  switch (c_enum) {
    case diplomat::capi::LeadingAdjustment_Auto:
    case diplomat::capi::LeadingAdjustment_None:
    case diplomat::capi::LeadingAdjustment_ToCased:
      return static_cast<LeadingAdjustment::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LeadingAdjustment_HPP

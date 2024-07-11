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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::LeadingAdjustment LeadingAdjustment::AsFFI() const {
  return static_cast<capi::LeadingAdjustment>(value);
}

inline LeadingAdjustment LeadingAdjustment::FromFFI(capi::LeadingAdjustment c_enum) {
  switch (c_enum) {
    case capi::LeadingAdjustment_Auto:
    case capi::LeadingAdjustment_None:
    case capi::LeadingAdjustment_ToCased:
      return static_cast<LeadingAdjustment::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LeadingAdjustment_HPP

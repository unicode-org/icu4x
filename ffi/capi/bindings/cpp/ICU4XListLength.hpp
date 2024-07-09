#ifndef ICU4XListLength_HPP
#define ICU4XListLength_HPP

#include "ICU4XListLength.d.hpp"

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


inline capi::ICU4XListLength ICU4XListLength::AsFFI() const {
  return static_cast<capi::ICU4XListLength>(value);
}

inline ICU4XListLength ICU4XListLength::FromFFI(capi::ICU4XListLength c_enum) {
  switch (c_enum) {
    case capi::ICU4XListLength_Wide:
    case capi::ICU4XListLength_Short:
    case capi::ICU4XListLength_Narrow:
      return static_cast<ICU4XListLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XListLength_HPP

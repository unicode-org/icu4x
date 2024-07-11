#ifndef CollatorCaseFirst_HPP
#define CollatorCaseFirst_HPP

#include "CollatorCaseFirst.d.hpp"

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


inline capi::CollatorCaseFirst CollatorCaseFirst::AsFFI() const {
  return static_cast<capi::CollatorCaseFirst>(value);
}

inline CollatorCaseFirst CollatorCaseFirst::FromFFI(capi::CollatorCaseFirst c_enum) {
  switch (c_enum) {
    case capi::CollatorCaseFirst_Auto:
    case capi::CollatorCaseFirst_Off:
    case capi::CollatorCaseFirst_LowerFirst:
    case capi::CollatorCaseFirst_UpperFirst:
      return static_cast<CollatorCaseFirst::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorCaseFirst_HPP

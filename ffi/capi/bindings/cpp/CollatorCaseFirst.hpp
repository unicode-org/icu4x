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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::CollatorCaseFirst CollatorCaseFirst::AsFFI() const {
  return static_cast<diplomat::capi::CollatorCaseFirst>(value);
}

inline CollatorCaseFirst CollatorCaseFirst::FromFFI(diplomat::capi::CollatorCaseFirst c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorCaseFirst_Auto:
    case diplomat::capi::CollatorCaseFirst_Off:
    case diplomat::capi::CollatorCaseFirst_LowerFirst:
    case diplomat::capi::CollatorCaseFirst_UpperFirst:
      return static_cast<CollatorCaseFirst::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorCaseFirst_HPP

#ifndef ListLength_HPP
#define ListLength_HPP

#include "ListLength.d.hpp"

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

inline diplomat::capi::ListLength ListLength::AsFFI() const {
  return static_cast<diplomat::capi::ListLength>(value);
}

inline ListLength ListLength::FromFFI(diplomat::capi::ListLength c_enum) {
  switch (c_enum) {
    case diplomat::capi::ListLength_Wide:
    case diplomat::capi::ListLength_Short:
    case diplomat::capi::ListLength_Narrow:
      return static_cast<ListLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ListLength_HPP

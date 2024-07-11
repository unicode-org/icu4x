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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ListLength ListLength::AsFFI() const {
  return static_cast<capi::ListLength>(value);
}

inline ListLength ListLength::FromFFI(capi::ListLength c_enum) {
  switch (c_enum) {
    case capi::ListLength_Wide:
    case capi::ListLength_Short:
    case capi::ListLength_Narrow:
      return static_cast<ListLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ListLength_HPP

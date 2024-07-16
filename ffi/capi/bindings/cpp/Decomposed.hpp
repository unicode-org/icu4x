#ifndef Decomposed_HPP
#define Decomposed_HPP

#include "Decomposed.d.hpp"

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


inline diplomat::capi::Decomposed Decomposed::AsFFI() const {
  return diplomat::capi::Decomposed {
    .first = first,
    .second = second,
  };
}

inline Decomposed Decomposed::FromFFI(diplomat::capi::Decomposed c_struct) {
  return Decomposed {
    .first = c_struct.first,
    .second = c_struct.second,
  };
}


#endif // Decomposed_HPP

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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::Decomposed Decomposed::AsFFI() const {
  return capi::Decomposed {
    .first = first,
    .second = second,
  };
}

inline Decomposed Decomposed::FromFFI(capi::Decomposed c_struct) {
  return Decomposed {
    .first = c_struct.first,
    .second = c_struct.second,
  };
}


#endif // Decomposed_HPP

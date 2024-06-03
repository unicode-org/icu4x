#ifndef ICU4XDecomposed_HPP
#define ICU4XDecomposed_HPP

#include "ICU4XDecomposed.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDecomposed.h"



inline capi::ICU4XDecomposed ICU4XDecomposed::AsFFI() const {
  return capi::ICU4XDecomposed {
    .first = first,
    .second = second,
  };
}

inline ICU4XDecomposed ICU4XDecomposed::FromFFI(capi::ICU4XDecomposed c_struct) {
  return ICU4XDecomposed {
    .first = c_struct.first,
    .second = c_struct.second,
  };
}


#endif // ICU4XDecomposed_HPP

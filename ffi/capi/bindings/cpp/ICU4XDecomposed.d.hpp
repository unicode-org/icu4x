#ifndef ICU4XDecomposed_D_HPP
#define ICU4XDecomposed_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XDecomposed {
      char32_t first;
      char32_t second;
    } ICU4XDecomposed;
}

struct ICU4XDecomposed {
  char32_t first;
  char32_t second;

  inline capi::ICU4XDecomposed AsFFI() const;
  inline static ICU4XDecomposed FromFFI(capi::ICU4XDecomposed c_struct);
};


#endif // ICU4XDecomposed_D_HPP

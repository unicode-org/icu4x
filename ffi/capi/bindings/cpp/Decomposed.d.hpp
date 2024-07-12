#ifndef Decomposed_D_HPP
#define Decomposed_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct Decomposed {
      char32_t first;
      char32_t second;
    };
} // namespace capi
} // namespace


struct Decomposed {
  char32_t first;
  char32_t second;

  inline diplomat::capi::Decomposed AsFFI() const;
  inline static Decomposed FromFFI(diplomat::capi::Decomposed c_struct);
};


#endif // Decomposed_D_HPP

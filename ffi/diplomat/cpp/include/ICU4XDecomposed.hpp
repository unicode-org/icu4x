#ifndef ICU4XDecomposed_HPP
#define ICU4XDecomposed_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XDecomposed.h"


/**
 * A destruction policy for using ICU4XDecomposed with std::unique_ptr.
 */
struct ICU4XDecomposedDeleter {
  void operator()(capi::ICU4XDecomposed* l) const noexcept {
    capi::ICU4XDecomposed_destroy(l);
  }
};

/**
 * The outcome of non-recursive canonical decomposition of a character.
 * `second` will be NUL when the decomposition expands to a single character
 * (which may or may not be the original one)
 * 
 * See the [Rust documentation for `Decomposed`](https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/properties/enum.Decomposed.html) for more information.
 */
struct ICU4XDecomposed {
 public:
  char32_t first;
  char32_t second;
};


#endif

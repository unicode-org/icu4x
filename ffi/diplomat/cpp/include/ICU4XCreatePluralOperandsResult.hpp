#ifndef ICU4XCreatePluralOperandsResult_HPP
#define ICU4XCreatePluralOperandsResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreatePluralOperandsResult.h"
}

#include "ICU4XPluralOperands.hpp"

/**
 * A destruction policy for using ICU4XCreatePluralOperandsResult with std::unique_ptr.
 */
struct ICU4XCreatePluralOperandsResultDeleter {
  void operator()(capi::ICU4XCreatePluralOperandsResult* l) const noexcept {
    capi::ICU4XCreatePluralOperandsResult_destroy(l);
  }
};

/**
 * This is the result returned by `ICU4XPluralOperands::create()`
 * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html) for more details.
 */
struct ICU4XCreatePluralOperandsResult {
 public:
  ICU4XPluralOperands operands;
  bool success;
};


#endif

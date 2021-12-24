#ifndef ICU4XCreateFixedDecimalResult_HPP
#define ICU4XCreateFixedDecimalResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreateFixedDecimalResult.h"
}

class ICU4XFixedDecimal;

/**
 * A destruction policy for using ICU4XCreateFixedDecimalResult with std::unique_ptr.
 */
struct ICU4XCreateFixedDecimalResultDeleter {
  void operator()(capi::ICU4XCreateFixedDecimalResult* l) const noexcept {
    capi::ICU4XCreateFixedDecimalResult_destroy(l);
  }
};
struct ICU4XCreateFixedDecimalResult {
 public:

  /**
   * Will be None if `success` is `false`
   */
  std::optional<ICU4XFixedDecimal> fd;

  /**
   * Currently just a boolean, but we might add a proper error enum as necessary
   */
  bool success;
};


#endif

#ifndef ICU4XFixedDecimalFormatResult_HPP
#define ICU4XFixedDecimalFormatResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalFormatResult.h"
}

class ICU4XFixedDecimalFormat;

/**
 * A destruction policy for using ICU4XFixedDecimalFormatResult with std::unique_ptr.
 */
struct ICU4XFixedDecimalFormatResultDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatResult* l) const noexcept {
    capi::ICU4XFixedDecimalFormatResult_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatResult {
 public:

  /**
   * The [`ICU4XFixedDecimalFormat`], exists if creation was successful.
   */
  std::optional<ICU4XFixedDecimalFormat> fdf;

  /**
   * Whether creating the [`ICU4XFixedDecimalFormat`] was successful.
   */
  bool success;
};


#endif

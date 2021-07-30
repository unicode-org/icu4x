#ifndef decimal_ffi_ICU4XFixedDecimalFormatResult_HPP
#define decimal_ffi_ICU4XFixedDecimalFormatResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "decimal_ffi_ICU4XFixedDecimalFormatResult.h"
}

class ICU4XFixedDecimalFormat;

struct ICU4XFixedDecimalFormatResultDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatResult* l) const noexcept {
    capi::ICU4XFixedDecimalFormatResult_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatResult {
 public:
  std::optional<ICU4XFixedDecimalFormat> fdf;
  bool success;
};


#endif

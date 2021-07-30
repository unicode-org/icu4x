#ifndef fixed_decimal_ffi_ICU4XCreateFixedDecimalResult_HPP
#define fixed_decimal_ffi_ICU4XCreateFixedDecimalResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "fixed_decimal_ffi_ICU4XCreateFixedDecimalResult.h"
}

class ICU4XFixedDecimal;

struct ICU4XCreateFixedDecimalResultDeleter {
  void operator()(capi::ICU4XCreateFixedDecimalResult* l) const noexcept {
    capi::ICU4XCreateFixedDecimalResult_destroy(l);
  }
};
struct ICU4XCreateFixedDecimalResult {
 public:
  std::optional<ICU4XFixedDecimal> fd;
  bool success;
};


#endif

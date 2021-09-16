#ifndef ICU4XCreatePluralOperandsResult_HPP
#define ICU4XCreatePluralOperandsResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreatePluralOperandsResult.h"
}

#include "ICU4XPluralOperands.hpp"

struct ICU4XCreatePluralOperandsResultDeleter {
  void operator()(capi::ICU4XCreatePluralOperandsResult* l) const noexcept {
    capi::ICU4XCreatePluralOperandsResult_destroy(l);
  }
};
struct ICU4XCreatePluralOperandsResult {
 public:
  ICU4XPluralOperands operands;
  bool success;
};


#endif

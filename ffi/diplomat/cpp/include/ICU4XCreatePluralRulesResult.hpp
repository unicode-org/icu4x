#ifndef ICU4XCreatePluralRulesResult_HPP
#define ICU4XCreatePluralRulesResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCreatePluralRulesResult.h"
}

class ICU4XPluralRules;

/**
 * A destruction policy for using ICU4XCreatePluralRulesResult with std::unique_ptr.
 */
struct ICU4XCreatePluralRulesResultDeleter {
  void operator()(capi::ICU4XCreatePluralRulesResult* l) const noexcept {
    capi::ICU4XCreatePluralRulesResult_destroy(l);
  }
};
struct ICU4XCreatePluralRulesResult {
 public:
  std::optional<ICU4XPluralRules> rules;
  bool success;
};


#endif

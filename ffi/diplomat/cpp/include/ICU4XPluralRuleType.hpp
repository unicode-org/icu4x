#ifndef ICU4XPluralRuleType_HPP
#define ICU4XPluralRuleType_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralRuleType.h"
}



/**
 * FFI version of `PluralRuleType`.
 * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralRuleType.html) for more details.
 */
enum struct ICU4XPluralRuleType {
  Cardinal = 0,
  Ordinal = 1,
};

#endif

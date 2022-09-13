#ifndef ICU4XCollatorNumeric_HPP
#define ICU4XCollatorNumeric_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollatorNumeric.h"



/**
 * 
 * 
 * See the [Rust documentation for `Numeric`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.Numeric.html) for more information.
 */
enum struct ICU4XCollatorNumeric {
  Auto = 0,
  Off = 1,
  On = 2,
};

#endif

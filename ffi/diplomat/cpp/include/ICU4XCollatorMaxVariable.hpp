#ifndef ICU4XCollatorMaxVariable_HPP
#define ICU4XCollatorMaxVariable_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollatorMaxVariable.h"



/**
 * 
 * 
 * See the [Rust documentation for `MaxVariable`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.MaxVariable.html) for more information.
 */
enum struct ICU4XCollatorMaxVariable {
  Auto = 0,
  Space = 1,
  Punctuation = 2,
  Symbol = 3,
  Currency = 4,
};

#endif

#ifndef ICU4XCollatorBackwardSecondLevel_HPP
#define ICU4XCollatorBackwardSecondLevel_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollatorBackwardSecondLevel.h"



/**
 * 
 * 
 * See the [Rust documentation for `BackwardSecondLevel`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.BackwardSecondLevel.html) for more information.
 */
enum struct ICU4XCollatorBackwardSecondLevel {
  Auto = 0,
  Off = 1,
  On = 2,
};

#endif

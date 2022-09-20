#ifndef ICU4XCollatorCaseFirst_HPP
#define ICU4XCollatorCaseFirst_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollatorCaseFirst.h"



/**
 * 
 * 
 * See the [Rust documentation for `CaseFirst`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.CaseFirst.html) for more information.
 */
enum struct ICU4XCollatorCaseFirst {
  Auto = 0,
  Off = 1,
  LowerFirst = 2,
  UpperFirst = 3,
};

#endif

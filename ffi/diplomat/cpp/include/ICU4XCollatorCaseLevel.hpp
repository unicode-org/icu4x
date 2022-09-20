#ifndef ICU4XCollatorCaseLevel_HPP
#define ICU4XCollatorCaseLevel_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollatorCaseLevel.h"



/**
 * 
 * 
 * See the [Rust documentation for `CaseLevel`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.CaseLevel.html) for more information.
 */
enum struct ICU4XCollatorCaseLevel {
  Auto = 0,
  Off = 1,
  On = 2,
};

#endif

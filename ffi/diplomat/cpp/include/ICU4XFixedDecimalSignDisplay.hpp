#ifndef ICU4XFixedDecimalSignDisplay_HPP
#define ICU4XFixedDecimalSignDisplay_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFixedDecimalSignDisplay.h"



/**
 * ECMA-402 compatible sign display preference.
 * 
 * See the [Rust documentation for `SignDisplay`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.SignDisplay.html) for more information.
 */
enum struct ICU4XFixedDecimalSignDisplay {
  Auto = 0,
  Never = 1,
  Always = 2,
  ExceptZero = 3,
  Negative = 4,
};

#endif

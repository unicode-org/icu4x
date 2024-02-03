#ifndef ICU4XFixedDecimalRoundingIncrement_HPP
#define ICU4XFixedDecimalRoundingIncrement_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFixedDecimalRoundingIncrement.h"



/**
 * Increment used in a rounding operation.
 * 
 * See the [Rust documentation for `RoundingIncrement`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.RoundingIncrement.html) for more information.
 */
enum struct ICU4XFixedDecimalRoundingIncrement {
  MultiplesOf1 = 0,
  MultiplesOf2 = 1,
  MultiplesOf5 = 2,
  MultiplesOf25 = 3,
};

#endif

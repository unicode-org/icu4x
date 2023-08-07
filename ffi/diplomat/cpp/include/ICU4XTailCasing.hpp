#ifndef ICU4XTailCasing_HPP
#define ICU4XTailCasing_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XTailCasing.h"



/**
 * 
 * 
 * See the [Rust documentation for `TailCasing`](https://docs.rs/icu/latest/icu/casemap/titlecase/enum.TailCasing.html) for more information.
 */
enum struct ICU4XTailCasing {
  Lowercase = 0,
  PreserveCase = 1,
};

#endif

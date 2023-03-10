#ifndef ICU4XFallback_HPP
#define ICU4XFallback_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFallback.h"



/**
 * 
 * 
 * See the [Rust documentation for `Fallback`](https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.Fallback.html) for more information.
 */
enum struct ICU4XFallback {
  Code = 0,
  None = 1,
};

#endif

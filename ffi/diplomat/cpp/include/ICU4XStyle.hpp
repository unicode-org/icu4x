#ifndef ICU4XStyle_HPP
#define ICU4XStyle_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XStyle.h"



/**
 * 
 * 
 * See the [Rust documentation for `Style`](https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.Style.html) for more information.
 */
enum struct ICU4XStyle {
  Auto = 0,
  Narrow = 1,
  Short = 2,
  Long = 3,
  Menu = 4,
};

#endif

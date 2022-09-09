#ifndef ICU4XListStyle_HPP
#define ICU4XListStyle_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XListStyle.h"



/**
 * 
 * 
 * See the [Rust documentation for `ListStyle`](https://unicode-org.github.io/icu4x-docs/doc/icu/list/enum.ListStyle.html) for more information.
 */
enum struct ICU4XListStyle {
  Wide = 0,
  Short = 1,
  Narrow = 2,
};

#endif

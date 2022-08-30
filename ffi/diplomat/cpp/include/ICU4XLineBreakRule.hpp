#ifndef ICU4XLineBreakRule_HPP
#define ICU4XLineBreakRule_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLineBreakRule.h"



/**
 * 
 * 
 * See the [Rust documentation for `LineBreakRule`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/enum.LineBreakRule.html) for more information.
 */
enum struct ICU4XLineBreakRule {
  Loose = 0,
  Normal = 1,
  Strict = 2,
  Anywhere = 3,
};

#endif

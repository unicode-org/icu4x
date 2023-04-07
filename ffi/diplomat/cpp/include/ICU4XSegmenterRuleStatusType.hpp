#ifndef ICU4XSegmenterRuleStatusType_HPP
#define ICU4XSegmenterRuleStatusType_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XSegmenterRuleStatusType.h"



/**
 * 
 * 
 * See the [Rust documentation for `RuleStatusType`](https://docs.rs/icu/latest/icu/segmenter/enum.RuleStatusType.html) for more information.
 */
enum struct ICU4XSegmenterRuleStatusType {
  None = 0,
  Number = 1,
  Letter = 2,
};

#endif

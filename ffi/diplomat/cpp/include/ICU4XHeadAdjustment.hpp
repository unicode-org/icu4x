#ifndef ICU4XHeadAdjustment_HPP
#define ICU4XHeadAdjustment_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XHeadAdjustment.h"



/**
 * 
 * 
 * See the [Rust documentation for `HeadAdjustment`](https://docs.rs/icu/latest/icu/casemap/titlecase/enum.HeadAdjustment.html) for more information.
 */
enum struct ICU4XHeadAdjustment {
  Adjust = 0,
  NoAdjust = 1,
};

#endif

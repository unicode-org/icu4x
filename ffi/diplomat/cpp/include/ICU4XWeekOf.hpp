#ifndef ICU4XWeekOf_HPP
#define ICU4XWeekOf_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XWeekOf.h"

#include "ICU4XWeekRelativeUnit.hpp"

/**
 * A destruction policy for using ICU4XWeekOf with std::unique_ptr.
 */
struct ICU4XWeekOfDeleter {
  void operator()(capi::ICU4XWeekOf* l) const noexcept {
    capi::ICU4XWeekOf_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `WeekOf`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekOf.html) for more information.
 */
struct ICU4XWeekOf {
 public:
  uint16_t week;
  ICU4XWeekRelativeUnit unit;
};


#endif

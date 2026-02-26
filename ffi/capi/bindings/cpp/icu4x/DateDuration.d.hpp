#ifndef ICU4X_DateDuration_D_HPP
#define ICU4X_DateDuration_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    struct DateDuration {
      bool is_negative;
      uint32_t years;
      uint32_t months;
      uint32_t weeks;
      uint64_t days;
    };

    typedef struct DateDuration_option {union { DateDuration ok; }; bool is_ok; } DateDuration_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * See the [Rust documentation for `DateDuration`](https://docs.rs/icu/2.1.1/icu/calendar/types/struct.DateDuration.html) for more information.
 */
struct DateDuration {
    bool is_negative;
    uint32_t years;
    uint32_t months;
    uint32_t weeks;
    uint64_t days;

    inline icu4x::capi::DateDuration AsFFI() const;
    inline static icu4x::DateDuration FromFFI(icu4x::capi::DateDuration c_struct);
};

} // namespace
#endif // ICU4X_DateDuration_D_HPP

#ifndef ICU4X_Duration_D_HPP
#define ICU4X_Duration_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DurationSign.d.hpp"
#include "diplomat_runtime.hpp"
namespace icu4x {
class DurationSign;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct Duration {
      icu4x::capi::DurationSign sign;
      uint64_t years;
      uint64_t months;
      uint64_t weeks;
      uint64_t days;
      uint64_t hours;
      uint64_t minutes;
      uint64_t seconds;
      uint64_t milliseconds;
      uint64_t microseconds;
      uint64_t nanoseconds;
    };

    typedef struct Duration_option {union { Duration ok; }; bool is_ok; } Duration_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * See the [Rust documentation for `Duration`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.Duration.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
struct Duration {
    icu4x::DurationSign sign;
    uint64_t years;
    uint64_t months;
    uint64_t weeks;
    uint64_t days;
    uint64_t hours;
    uint64_t minutes;
    uint64_t seconds;
    uint64_t milliseconds;
    uint64_t microseconds;
    uint64_t nanoseconds;

    inline icu4x::capi::Duration AsFFI() const;
    inline static icu4x::Duration FromFFI(icu4x::capi::Duration c_struct);
};

} // namespace
#endif // ICU4X_Duration_D_HPP

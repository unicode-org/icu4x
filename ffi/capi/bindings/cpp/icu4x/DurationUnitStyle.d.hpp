#ifndef ICU4X_DurationUnitStyle_D_HPP
#define ICU4X_DurationUnitStyle_D_HPP

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
    enum DurationUnitStyle {
      DurationUnitStyle_Narrow = 0,
      DurationUnitStyle_Short = 1,
      DurationUnitStyle_Long = 2,
      DurationUnitStyle_TwoDigit = 3,
      DurationUnitStyle_Numeric = 4,
      DurationUnitStyle_Fractional = 5,
    };

    typedef struct DurationUnitStyle_option {union { DurationUnitStyle ok; }; bool is_ok; } DurationUnitStyle_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `YearStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.YearStyle.html) for more information.
 *
 * See the [Rust documentation for `HourStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.HourStyle.html) for more information.
 *
 * See the [Rust documentation for `MilliSecondStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.MilliSecondStyle.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationUnitStyle {
public:
    enum Value {
        Narrow = 0,
        Short = 1,
        Long = 2,
        TwoDigit = 3,
        Numeric = 4,
        Fractional = 5,
    };

    DurationUnitStyle(): value(Value::Short) {}

    // Implicit conversions between enum and ::Value
    constexpr DurationUnitStyle(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::DurationUnitStyle AsFFI() const;
    inline static icu4x::DurationUnitStyle FromFFI(icu4x::capi::DurationUnitStyle c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_DurationUnitStyle_D_HPP

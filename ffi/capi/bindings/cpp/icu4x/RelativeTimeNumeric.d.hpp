#ifndef ICU4X_RelativeTimeNumeric_D_HPP
#define ICU4X_RelativeTimeNumeric_D_HPP

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
    enum RelativeTimeNumeric {
      RelativeTimeNumeric_Always = 0,
      RelativeTimeNumeric_Auto = 1,
    };

    typedef struct RelativeTimeNumeric_option {union { RelativeTimeNumeric ok; }; bool is_ok; } RelativeTimeNumeric_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * Configures whether to always use numeric formatting even when special formatting is available.
 *
 * See the [Rust documentation for `Numeric`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/options/enum.Numeric.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class RelativeTimeNumeric {
public:
    enum Value {
        /**
         * Always use numeric formatting.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        Always = 0,
        /**
         * Automatically select special formatting if available else fallback to numeric formatting.
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        Auto = 1,
    };

    RelativeTimeNumeric(): value(Value::Always) {}

    // Implicit conversions between enum and ::Value
    constexpr RelativeTimeNumeric(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::RelativeTimeNumeric AsFFI() const;
    inline static icu4x::RelativeTimeNumeric FromFFI(icu4x::capi::RelativeTimeNumeric c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_RelativeTimeNumeric_D_HPP

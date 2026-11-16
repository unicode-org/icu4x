#ifndef ICU4X_DurationSign_D_HPP
#define ICU4X_DurationSign_D_HPP

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
    enum DurationSign {
      DurationSign_Positive = 0,
      DurationSign_Negative = 1,
    };

    typedef struct DurationSign_option {union { DurationSign ok; }; bool is_ok; } DurationSign_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `DurationSign`](https://docs.rs/icu/2.4.0/icu/experimental/duration/enum.DurationSign.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationSign {
public:
    enum Value {
        Positive = 0,
        Negative = 1,
    };

    DurationSign(): value(Value::Positive) {}

    // Implicit conversions between enum and ::Value
    constexpr DurationSign(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::DurationSign AsFFI() const;
    inline static icu4x::DurationSign FromFFI(icu4x::capi::DurationSign c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_DurationSign_D_HPP

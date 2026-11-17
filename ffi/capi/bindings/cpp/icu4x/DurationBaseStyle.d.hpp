#ifndef ICU4X_DurationBaseStyle_D_HPP
#define ICU4X_DurationBaseStyle_D_HPP

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
    enum DurationBaseStyle {
      DurationBaseStyle_Narrow = 0,
      DurationBaseStyle_Short = 1,
      DurationBaseStyle_Long = 2,
      DurationBaseStyle_Digital = 3,
    };

    typedef struct DurationBaseStyle_option {union { DurationBaseStyle ok; }; bool is_ok; } DurationBaseStyle_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `BaseStyle`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.BaseStyle.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationBaseStyle {
public:
    enum Value {
        Narrow = 0,
        Short = 1,
        Long = 2,
        Digital = 3,
    };

    DurationBaseStyle(): value(Value::Short) {}

    // Implicit conversions between enum and ::Value
    constexpr DurationBaseStyle(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::DurationBaseStyle AsFFI() const;
    inline static icu4x::DurationBaseStyle FromFFI(icu4x::capi::DurationBaseStyle c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_DurationBaseStyle_D_HPP

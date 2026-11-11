#ifndef ICU4X_DurationFieldDisplay_D_HPP
#define ICU4X_DurationFieldDisplay_D_HPP

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
    enum DurationFieldDisplay {
      DurationFieldDisplay_Auto = 0,
      DurationFieldDisplay_Always = 1,
    };

    typedef struct DurationFieldDisplay_option {union { DurationFieldDisplay ok; }; bool is_ok; } DurationFieldDisplay_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `FieldDisplay`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/enum.FieldDisplay.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationFieldDisplay {
public:
    enum Value {
        Auto = 0,
        Always = 1,
    };

    DurationFieldDisplay(): value(Value::Auto) {}

    // Implicit conversions between enum and ::Value
    constexpr DurationFieldDisplay(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::DurationFieldDisplay AsFFI() const;
    inline static icu4x::DurationFieldDisplay FromFFI(icu4x::capi::DurationFieldDisplay c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_DurationFieldDisplay_D_HPP

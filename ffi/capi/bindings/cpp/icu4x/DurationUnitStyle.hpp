#ifndef ICU4X_DurationUnitStyle_HPP
#define ICU4X_DurationUnitStyle_HPP

#include "DurationUnitStyle.d.hpp"

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

} // namespace capi
} // namespace

inline icu4x::capi::DurationUnitStyle icu4x::DurationUnitStyle::AsFFI() const {
    return static_cast<icu4x::capi::DurationUnitStyle>(value);
}

inline icu4x::DurationUnitStyle icu4x::DurationUnitStyle::FromFFI(icu4x::capi::DurationUnitStyle c_enum) {
    switch (c_enum) {
        case icu4x::capi::DurationUnitStyle_Default:
        case icu4x::capi::DurationUnitStyle_Narrow:
        case icu4x::capi::DurationUnitStyle_Short:
        case icu4x::capi::DurationUnitStyle_Long:
        case icu4x::capi::DurationUnitStyle_TwoDigit:
        case icu4x::capi::DurationUnitStyle_Numeric:
        case icu4x::capi::DurationUnitStyle_Fractional:
            return static_cast<icu4x::DurationUnitStyle::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_DurationUnitStyle_HPP

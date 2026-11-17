#ifndef ICU4X_DurationBaseStyle_HPP
#define ICU4X_DurationBaseStyle_HPP

#include "DurationBaseStyle.d.hpp"

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

inline icu4x::capi::DurationBaseStyle icu4x::DurationBaseStyle::AsFFI() const {
    return static_cast<icu4x::capi::DurationBaseStyle>(value);
}

inline icu4x::DurationBaseStyle icu4x::DurationBaseStyle::FromFFI(icu4x::capi::DurationBaseStyle c_enum) {
    switch (c_enum) {
        case icu4x::capi::DurationBaseStyle_Narrow:
        case icu4x::capi::DurationBaseStyle_Short:
        case icu4x::capi::DurationBaseStyle_Long:
        case icu4x::capi::DurationBaseStyle_Digital:
            return static_cast<icu4x::DurationBaseStyle::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_DurationBaseStyle_HPP

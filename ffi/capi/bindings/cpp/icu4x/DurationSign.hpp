#ifndef ICU4X_DurationSign_HPP
#define ICU4X_DurationSign_HPP

#include "DurationSign.d.hpp"

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

inline icu4x::capi::DurationSign icu4x::DurationSign::AsFFI() const {
    return static_cast<icu4x::capi::DurationSign>(value);
}

inline icu4x::DurationSign icu4x::DurationSign::FromFFI(icu4x::capi::DurationSign c_enum) {
    switch (c_enum) {
        case icu4x::capi::DurationSign_Positive:
        case icu4x::capi::DurationSign_Negative:
            return static_cast<icu4x::DurationSign::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_DurationSign_HPP

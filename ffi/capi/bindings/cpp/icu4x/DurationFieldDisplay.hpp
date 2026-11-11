#ifndef ICU4X_DurationFieldDisplay_HPP
#define ICU4X_DurationFieldDisplay_HPP

#include "DurationFieldDisplay.d.hpp"

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

inline icu4x::capi::DurationFieldDisplay icu4x::DurationFieldDisplay::AsFFI() const {
    return static_cast<icu4x::capi::DurationFieldDisplay>(value);
}

inline icu4x::DurationFieldDisplay icu4x::DurationFieldDisplay::FromFFI(icu4x::capi::DurationFieldDisplay c_enum) {
    switch (c_enum) {
        case icu4x::capi::DurationFieldDisplay_Default:
        case icu4x::capi::DurationFieldDisplay_Auto:
        case icu4x::capi::DurationFieldDisplay_Always:
            return static_cast<icu4x::DurationFieldDisplay::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_DurationFieldDisplay_HPP

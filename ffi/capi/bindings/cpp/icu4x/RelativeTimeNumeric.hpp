#ifndef ICU4X_RelativeTimeNumeric_HPP
#define ICU4X_RelativeTimeNumeric_HPP

#include "RelativeTimeNumeric.d.hpp"

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

inline icu4x::capi::RelativeTimeNumeric icu4x::RelativeTimeNumeric::AsFFI() const {
    return static_cast<icu4x::capi::RelativeTimeNumeric>(value);
}

inline icu4x::RelativeTimeNumeric icu4x::RelativeTimeNumeric::FromFFI(icu4x::capi::RelativeTimeNumeric c_enum) {
    switch (c_enum) {
        case icu4x::capi::RelativeTimeNumeric_Always:
        case icu4x::capi::RelativeTimeNumeric_Auto:
            return static_cast<icu4x::RelativeTimeNumeric::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_RelativeTimeNumeric_HPP

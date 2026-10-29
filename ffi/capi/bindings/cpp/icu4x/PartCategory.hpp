#ifndef ICU4X_PartCategory_HPP
#define ICU4X_PartCategory_HPP

#include "PartCategory.d.hpp"

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

inline icu4x::capi::PartCategory icu4x::PartCategory::AsFFI() const {
    return static_cast<icu4x::capi::PartCategory>(value);
}

inline icu4x::PartCategory icu4x::PartCategory::FromFFI(icu4x::capi::PartCategory c_enum) {
    switch (c_enum) {
        case icu4x::capi::PartCategory_Unknown:
        case icu4x::capi::PartCategory_Error:
        case icu4x::capi::PartCategory_List:
        case icu4x::capi::PartCategory_Decimal:
        case icu4x::capi::PartCategory_DateTime:
        case icu4x::capi::PartCategory_RelativeTime:
        case icu4x::capi::PartCategory_Duration:
        case icu4x::capi::PartCategory_Unit:
            return static_cast<icu4x::PartCategory::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_PartCategory_HPP

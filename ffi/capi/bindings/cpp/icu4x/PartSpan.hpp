#ifndef ICU4X_PartSpan_HPP
#define ICU4X_PartSpan_HPP

#include "PartSpan.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "PartKind.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {

} // namespace capi
} // namespace


inline icu4x::capi::PartSpan icu4x::PartSpan::AsFFI() const {
    return icu4x::capi::PartSpan {
        /* .start = */ start,
        /* .end = */ end,
        /* .kind = */ kind.AsFFI(),
    };
}

inline icu4x::PartSpan icu4x::PartSpan::FromFFI(icu4x::capi::PartSpan c_struct) {
    return icu4x::PartSpan {
        /* .start = */ c_struct.start,
        /* .end = */ c_struct.end,
        /* .kind = */ icu4x::PartKind::FromFFI(c_struct.kind),
    };
}


#endif // ICU4X_PartSpan_HPP

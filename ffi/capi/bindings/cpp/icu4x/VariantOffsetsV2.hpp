#ifndef icu4x_VariantOffsetsV2_HPP
#define icu4x_VariantOffsetsV2_HPP

#include "VariantOffsetsV2.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "UtcOffset.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::VariantOffsetsV2 icu4x::VariantOffsetsV2::AsFFI() const {
  return icu4x::capi::VariantOffsetsV2 {
    /* .standard = */ standard->AsFFI(),
    /* .daylight = */ daylight ? daylight->AsFFI() : nullptr,
    /* .sundown = */ sundown ? sundown->AsFFI() : nullptr,
  };
}

inline icu4x::VariantOffsetsV2 icu4x::VariantOffsetsV2::FromFFI(icu4x::capi::VariantOffsetsV2 c_struct) {
  return icu4x::VariantOffsetsV2 {
    /* .standard = */ std::unique_ptr<icu4x::UtcOffset>(icu4x::UtcOffset::FromFFI(c_struct.standard)),
    /* .daylight = */ std::unique_ptr<icu4x::UtcOffset>(icu4x::UtcOffset::FromFFI(c_struct.daylight)),
    /* .sundown = */ std::unique_ptr<icu4x::UtcOffset>(icu4x::UtcOffset::FromFFI(c_struct.sundown)),
  };
}


#endif // icu4x_VariantOffsetsV2_HPP

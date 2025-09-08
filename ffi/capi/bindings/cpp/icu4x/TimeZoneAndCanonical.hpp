#ifndef icu4x_TimeZoneAndCanonical_HPP
#define icu4x_TimeZoneAndCanonical_HPP

#include "TimeZoneAndCanonical.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "TimeZone.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::TimeZoneAndCanonical icu4x::TimeZoneAndCanonical::AsFFI() const {
  return icu4x::capi::TimeZoneAndCanonical {
    /* .time_zone = */ time_zone->AsFFI(),
    /* .canonical = */ {canonical.data(), canonical.size()},
  };
}

inline icu4x::TimeZoneAndCanonical icu4x::TimeZoneAndCanonical::FromFFI(icu4x::capi::TimeZoneAndCanonical c_struct) {
  return icu4x::TimeZoneAndCanonical {
    /* .time_zone = */ std::unique_ptr<icu4x::TimeZone>(icu4x::TimeZone::FromFFI(c_struct.time_zone)),
    /* .canonical = */ std::string_view(c_struct.canonical.data, c_struct.canonical.len),
  };
}


#endif // icu4x_TimeZoneAndCanonical_HPP

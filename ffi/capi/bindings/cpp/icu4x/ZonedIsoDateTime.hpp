#ifndef icu4x_ZonedIsoDateTime_HPP
#define icu4x_ZonedIsoDateTime_HPP

#include "ZonedIsoDateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "IsoDate.hpp"
#include "Time.hpp"
#include "TimeZoneInfo.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::ZonedIsoDateTime icu4x::ZonedIsoDateTime::AsFFI() const {
  return icu4x::capi::ZonedIsoDateTime {
    /* .date = */ date->AsFFI(),
    /* .time = */ time->AsFFI(),
    /* .zone = */ zone->AsFFI(),
  };
}

inline icu4x::ZonedIsoDateTime icu4x::ZonedIsoDateTime::FromFFI(icu4x::capi::ZonedIsoDateTime c_struct) {
  return icu4x::ZonedIsoDateTime {
    /* .date = */ std::unique_ptr<icu4x::IsoDate>(icu4x::IsoDate::FromFFI(c_struct.date)),
    /* .time = */ std::unique_ptr<icu4x::Time>(icu4x::Time::FromFFI(c_struct.time)),
    /* .zone = */ std::unique_ptr<icu4x::TimeZoneInfo>(icu4x::TimeZoneInfo::FromFFI(c_struct.zone)),
  };
}


#endif // icu4x_ZonedIsoDateTime_HPP

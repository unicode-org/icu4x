#ifndef icu4x_ZonedDateTime_HPP
#define icu4x_ZonedDateTime_HPP

#include "ZonedDateTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "Date.hpp"
#include "Time.hpp"
#include "TimeZoneInfo.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::ZonedDateTime icu4x::ZonedDateTime::AsFFI() const {
  return icu4x::capi::ZonedDateTime {
    /* .date = */ date->AsFFI(),
    /* .time = */ time->AsFFI(),
    /* .zone = */ zone->AsFFI(),
  };
}

inline icu4x::ZonedDateTime icu4x::ZonedDateTime::FromFFI(icu4x::capi::ZonedDateTime c_struct) {
  return icu4x::ZonedDateTime {
    /* .date = */ std::unique_ptr<icu4x::Date>(icu4x::Date::FromFFI(c_struct.date)),
    /* .time = */ std::unique_ptr<icu4x::Time>(icu4x::Time::FromFFI(c_struct.time)),
    /* .zone = */ std::unique_ptr<icu4x::TimeZoneInfo>(icu4x::TimeZoneInfo::FromFFI(c_struct.zone)),
  };
}


#endif // icu4x_ZonedDateTime_HPP

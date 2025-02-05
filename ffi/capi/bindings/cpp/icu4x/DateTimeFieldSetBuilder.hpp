#ifndef icu4x_DateTimeFieldSetBuilder_HPP
#define icu4x_DateTimeFieldSetBuilder_HPP

#include "DateTimeFieldSetBuilder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DateFields.hpp"
#include "DateTimeAlignment.hpp"
#include "DateTimeLength.hpp"
#include "TimePrecision.hpp"
#include "YearStyle.hpp"
#include "ZoneStyle.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::DateTimeFieldSetBuilder icu4x::DateTimeFieldSetBuilder::AsFFI() const {
  return icu4x::capi::DateTimeFieldSetBuilder {
    /* .length = */ length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    /* .date_fields = */ date_fields.has_value() ? (icu4x::capi::DateFields_option{ { date_fields.value().AsFFI() }, true }) : (icu4x::capi::DateFields_option{ {}, false }),
    /* .time_precision = */ time_precision.has_value() ? (icu4x::capi::TimePrecision_option{ { time_precision.value().AsFFI() }, true }) : (icu4x::capi::TimePrecision_option{ {}, false }),
    /* .zone_style = */ zone_style.has_value() ? (icu4x::capi::ZoneStyle_option{ { zone_style.value().AsFFI() }, true }) : (icu4x::capi::ZoneStyle_option{ {}, false }),
    /* .alignment = */ alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    /* .year_style = */ year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }),
  };
}

inline icu4x::DateTimeFieldSetBuilder icu4x::DateTimeFieldSetBuilder::FromFFI(icu4x::capi::DateTimeFieldSetBuilder c_struct) {
  return icu4x::DateTimeFieldSetBuilder {
    /* .length = */ c_struct.length.is_ok ? std::optional(icu4x::DateTimeLength::FromFFI(c_struct.length.ok)) : std::nullopt,
    /* .date_fields = */ c_struct.date_fields.is_ok ? std::optional(icu4x::DateFields::FromFFI(c_struct.date_fields.ok)) : std::nullopt,
    /* .time_precision = */ c_struct.time_precision.is_ok ? std::optional(icu4x::TimePrecision::FromFFI(c_struct.time_precision.ok)) : std::nullopt,
    /* .zone_style = */ c_struct.zone_style.is_ok ? std::optional(icu4x::ZoneStyle::FromFFI(c_struct.zone_style.ok)) : std::nullopt,
    /* .alignment = */ c_struct.alignment.is_ok ? std::optional(icu4x::DateTimeAlignment::FromFFI(c_struct.alignment.ok)) : std::nullopt,
    /* .year_style = */ c_struct.year_style.is_ok ? std::optional(icu4x::YearStyle::FromFFI(c_struct.year_style.ok)) : std::nullopt,
  };
}


#endif // icu4x_DateTimeFieldSetBuilder_HPP

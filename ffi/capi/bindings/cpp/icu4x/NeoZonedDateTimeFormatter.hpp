#ifndef icu4x_NeoZonedDateTimeFormatter_HPP
#define icu4x_NeoZonedDateTimeFormatter_HPP

#include "NeoZonedDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "Date.hpp"
#include "DateTimeMismatchedCalendarError.hpp"
#include "DateTimeWriteError.hpp"
#include "IsoDate.hpp"
#include "Time.hpp"
#include "TimeZoneInfo.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result {union { icu4x::capi::DateTimeWriteError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result icu4x_NeoZonedDateTimeFormatter_format_iso_mv1(const icu4x::capi::NeoZonedDateTimeFormatter* self, const icu4x::capi::IsoDate* date, const icu4x::capi::Time* time, const icu4x::capi::TimeZoneInfo* zone, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1_result {union { icu4x::capi::DateTimeMismatchedCalendarError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1_result icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1(const icu4x::capi::NeoZonedDateTimeFormatter* self, const icu4x::capi::Date* _date, const icu4x::capi::Time* _time, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_NeoZonedDateTimeFormatter_destroy_mv1(NeoZonedDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::string, icu4x::DateTimeWriteError> icu4x::NeoZonedDateTimeFormatter::format_iso(const icu4x::IsoDate& date, const icu4x::Time& time, const icu4x::TimeZoneInfo& zone) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_format_iso_mv1(this->AsFFI(),
    date.AsFFI(),
    time.AsFFI(),
    zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::DateTimeWriteError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::DateTimeWriteError>(diplomat::Err<icu4x::DateTimeWriteError>(icu4x::DateTimeWriteError::FromFFI(result.err)));
}

inline diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError> icu4x::NeoZonedDateTimeFormatter::format_same_calendar(const icu4x::Date& _date, const icu4x::Time& _time) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_format_same_calendar_mv1(this->AsFFI(),
    _date.AsFFI(),
    _time.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError>(diplomat::Err<icu4x::DateTimeMismatchedCalendarError>(icu4x::DateTimeMismatchedCalendarError::FromFFI(result.err)));
}

inline const icu4x::capi::NeoZonedDateTimeFormatter* icu4x::NeoZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::NeoZonedDateTimeFormatter*>(this);
}

inline icu4x::capi::NeoZonedDateTimeFormatter* icu4x::NeoZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::NeoZonedDateTimeFormatter*>(this);
}

inline const icu4x::NeoZonedDateTimeFormatter* icu4x::NeoZonedDateTimeFormatter::FromFFI(const icu4x::capi::NeoZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const icu4x::NeoZonedDateTimeFormatter*>(ptr);
}

inline icu4x::NeoZonedDateTimeFormatter* icu4x::NeoZonedDateTimeFormatter::FromFFI(icu4x::capi::NeoZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<icu4x::NeoZonedDateTimeFormatter*>(ptr);
}

inline void icu4x::NeoZonedDateTimeFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_NeoZonedDateTimeFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::NeoZonedDateTimeFormatter*>(ptr));
}


#endif // icu4x_NeoZonedDateTimeFormatter_HPP

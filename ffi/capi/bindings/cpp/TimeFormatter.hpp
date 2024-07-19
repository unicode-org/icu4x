#ifndef TimeFormatter_HPP
#define TimeFormatter_HPP

#include "TimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateTime.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "Time.hpp"
#include "TimeLength.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_TimeFormatter_create_with_length_mv1_result {union {diplomat::capi::TimeFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_TimeFormatter_create_with_length_mv1_result;
    icu4x_TimeFormatter_create_with_length_mv1_result icu4x_TimeFormatter_create_with_length_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::TimeLength length);
    
    void icu4x_TimeFormatter_format_time_mv1(const diplomat::capi::TimeFormatter* self, const diplomat::capi::Time* value, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_TimeFormatter_format_datetime_mv1(const diplomat::capi::TimeFormatter* self, const diplomat::capi::DateTime* value, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_TimeFormatter_format_iso_datetime_mv1(const diplomat::capi::TimeFormatter* self, const diplomat::capi::IsoDateTime* value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_TimeFormatter_destroy_mv1(TimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<TimeFormatter>, Error> TimeFormatter::create_with_length(const DataProvider& provider, const Locale& locale, TimeLength length) {
  auto result = diplomat::capi::icu4x_TimeFormatter_create_with_length_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<TimeFormatter>>(std::unique_ptr<TimeFormatter>(TimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string TimeFormatter::format_time(const Time& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_TimeFormatter_format_time_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline std::string TimeFormatter::format_datetime(const DateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_TimeFormatter_format_datetime_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline std::string TimeFormatter::format_iso_datetime(const IsoDateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_TimeFormatter_format_iso_datetime_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::TimeFormatter* TimeFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TimeFormatter*>(this);
}

inline diplomat::capi::TimeFormatter* TimeFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::TimeFormatter*>(this);
}

inline const TimeFormatter* TimeFormatter::FromFFI(const diplomat::capi::TimeFormatter* ptr) {
  return reinterpret_cast<const TimeFormatter*>(ptr);
}

inline TimeFormatter* TimeFormatter::FromFFI(diplomat::capi::TimeFormatter* ptr) {
  return reinterpret_cast<TimeFormatter*>(ptr);
}

inline void TimeFormatter::operator delete(void* ptr) {
  diplomat::capi::icu4x_TimeFormatter_destroy_mv1(reinterpret_cast<diplomat::capi::TimeFormatter*>(ptr));
}


#endif // TimeFormatter_HPP

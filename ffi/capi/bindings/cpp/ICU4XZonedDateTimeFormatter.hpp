#ifndef ICU4XZonedDateTimeFormatter_HPP
#define ICU4XZonedDateTimeFormatter_HPP

#include "ICU4XZonedDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCustomTimeZone.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDateLength.hpp"
#include "ICU4XDateTime.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XIsoTimeZoneOptions.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XTimeLength.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_result {union {ICU4XZonedDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_result;
    ICU4XZonedDateTimeFormatter_create_with_lengths_result ICU4XZonedDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);
    
    typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {ICU4XZonedDateTimeFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
    ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options);
    
    typedef struct ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result {union { ICU4XError err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result;
    ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(const ICU4XZonedDateTimeFormatter* self, const ICU4XDateTime* datetime, const ICU4XCustomTimeZone* time_zone, DiplomatWrite* write);
    
    typedef struct ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result {union { ICU4XError err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result;
    ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const ICU4XZonedDateTimeFormatter* self, const ICU4XIsoDateTime* datetime, const ICU4XCustomTimeZone* time_zone, DiplomatWrite* write);
    
    
    void ICU4XZonedDateTimeFormatter_destroy(ICU4XZonedDateTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError> ICU4XZonedDateTimeFormatter::create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length) {
  auto result = capi::ICU4XZonedDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XZonedDateTimeFormatter>>(std::unique_ptr<ICU4XZonedDateTimeFormatter>(ICU4XZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError> ICU4XZonedDateTimeFormatter::create_with_lengths_and_iso_8601_time_zone_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options) {
  auto result = capi::ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI(),
    zone_options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XZonedDateTimeFormatter>>(std::unique_ptr<ICU4XZonedDateTimeFormatter>(ICU4XZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XZonedDateTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XZonedDateTimeFormatter::format_datetime_with_custom_time_zone(const ICU4XDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XZonedDateTimeFormatter* ICU4XZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XZonedDateTimeFormatter*>(this);
}

inline capi::ICU4XZonedDateTimeFormatter* ICU4XZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XZonedDateTimeFormatter*>(this);
}

inline const ICU4XZonedDateTimeFormatter* ICU4XZonedDateTimeFormatter::FromFFI(const capi::ICU4XZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const ICU4XZonedDateTimeFormatter*>(ptr);
}

inline ICU4XZonedDateTimeFormatter* ICU4XZonedDateTimeFormatter::FromFFI(capi::ICU4XZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<ICU4XZonedDateTimeFormatter*>(ptr);
}

inline void ICU4XZonedDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XZonedDateTimeFormatter_destroy(reinterpret_cast<capi::ICU4XZonedDateTimeFormatter*>(ptr));
}


#endif // ICU4XZonedDateTimeFormatter_HPP

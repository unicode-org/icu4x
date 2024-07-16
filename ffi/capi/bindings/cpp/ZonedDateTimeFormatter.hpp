#ifndef ZonedDateTimeFormatter_HPP
#define ZonedDateTimeFormatter_HPP

#include "ZonedDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CustomTimeZone.hpp"
#include "DataProvider.hpp"
#include "DateLength.hpp"
#include "DateTime.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "IsoTimeZoneOptions.hpp"
#include "Locale.hpp"
#include "TimeLength.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_result {union {diplomat::capi::ZonedDateTimeFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_result;
    ICU4XZonedDateTimeFormatter_create_with_lengths_result ICU4XZonedDateTimeFormatter_create_with_lengths(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength date_length, diplomat::capi::TimeLength time_length);
    
    typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {diplomat::capi::ZonedDateTimeFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
    ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength date_length, diplomat::capi::TimeLength time_length, diplomat::capi::IsoTimeZoneOptions zone_options);
    
    typedef struct ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result {union { diplomat::capi::Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result;
    ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(const diplomat::capi::ZonedDateTimeFormatter* self, const diplomat::capi::DateTime* datetime, const diplomat::capi::CustomTimeZone* time_zone, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result {union { diplomat::capi::Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result;
    ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const diplomat::capi::ZonedDateTimeFormatter* self, const diplomat::capi::IsoDateTime* datetime, const diplomat::capi::CustomTimeZone* time_zone, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XZonedDateTimeFormatter_destroy(ZonedDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error> ZonedDateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = diplomat::capi::ICU4XZonedDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<ZonedDateTimeFormatter>>(std::unique_ptr<ZonedDateTimeFormatter>(ZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error> ZonedDateTimeFormatter::create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options) {
  auto result = diplomat::capi::ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI(),
    zone_options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<ZonedDateTimeFormatter>>(std::unique_ptr<ZonedDateTimeFormatter>(ZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> ZonedDateTimeFormatter::format_datetime_with_custom_time_zone(const DateTime& datetime, const CustomTimeZone& time_zone) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> ZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const IsoDateTime& datetime, const CustomTimeZone& time_zone) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline const diplomat::capi::ZonedDateTimeFormatter* ZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ZonedDateTimeFormatter*>(this);
}

inline diplomat::capi::ZonedDateTimeFormatter* ZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::ZonedDateTimeFormatter*>(this);
}

inline const ZonedDateTimeFormatter* ZonedDateTimeFormatter::FromFFI(const diplomat::capi::ZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const ZonedDateTimeFormatter*>(ptr);
}

inline ZonedDateTimeFormatter* ZonedDateTimeFormatter::FromFFI(diplomat::capi::ZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<ZonedDateTimeFormatter*>(ptr);
}

inline void ZonedDateTimeFormatter::operator delete(void* ptr) {
  diplomat::capi::ICU4XZonedDateTimeFormatter_destroy(reinterpret_cast<diplomat::capi::ZonedDateTimeFormatter*>(ptr));
}


#endif // ZonedDateTimeFormatter_HPP

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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_result {union {ZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_result;
    ICU4XZonedDateTimeFormatter_create_with_lengths_result ICU4XZonedDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);
    
    typedef struct ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {ZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
    ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options);
    
    typedef struct ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result {union { Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result;
    ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(const ZonedDateTimeFormatter* self, const DateTime* datetime, const CustomTimeZone* time_zone, DiplomatWrite* write);
    
    typedef struct ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result {union { Error err;}; bool is_ok;} ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result;
    ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_result ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const ZonedDateTimeFormatter* self, const IsoDateTime* datetime, const CustomTimeZone* time_zone, DiplomatWrite* write);
    
    
    void ICU4XZonedDateTimeFormatter_destroy(ZonedDateTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error> ZonedDateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = capi::ICU4XZonedDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<ZonedDateTimeFormatter>>(std::unique_ptr<ZonedDateTimeFormatter>(ZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error> ZonedDateTimeFormatter::create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options) {
  auto result = capi::ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI(),
    zone_options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<ZonedDateTimeFormatter>>(std::unique_ptr<ZonedDateTimeFormatter>(ZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> ZonedDateTimeFormatter::format_datetime_with_custom_time_zone(const DateTime& datetime, const CustomTimeZone& time_zone) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> ZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const IsoDateTime& datetime, const CustomTimeZone& time_zone) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline const capi::ZonedDateTimeFormatter* ZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ZonedDateTimeFormatter*>(this);
}

inline capi::ZonedDateTimeFormatter* ZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::ZonedDateTimeFormatter*>(this);
}

inline const ZonedDateTimeFormatter* ZonedDateTimeFormatter::FromFFI(const capi::ZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const ZonedDateTimeFormatter*>(ptr);
}

inline ZonedDateTimeFormatter* ZonedDateTimeFormatter::FromFFI(capi::ZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<ZonedDateTimeFormatter*>(ptr);
}

inline void ZonedDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XZonedDateTimeFormatter_destroy(reinterpret_cast<capi::ZonedDateTimeFormatter*>(ptr));
}


#endif // ZonedDateTimeFormatter_HPP

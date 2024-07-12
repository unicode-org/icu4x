#ifndef GregorianZonedDateTimeFormatter_HPP
#define GregorianZonedDateTimeFormatter_HPP

#include "GregorianZonedDateTimeFormatter.d.hpp"

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
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "IsoTimeZoneOptions.hpp"
#include "Locale.hpp"
#include "TimeLength.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result {union {diplomat::capi::GregorianZonedDateTimeFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result;
    ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength date_length, diplomat::capi::TimeLength time_length);
    
    typedef struct ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {diplomat::capi::GregorianZonedDateTimeFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
    ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength date_length, diplomat::capi::TimeLength time_length, diplomat::capi::IsoTimeZoneOptions zone_options);
    
    void ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const diplomat::capi::GregorianZonedDateTimeFormatter* self, const diplomat::capi::IsoDateTime* datetime, const diplomat::capi::CustomTimeZone* time_zone, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XGregorianZonedDateTimeFormatter_destroy(GregorianZonedDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error> GregorianZonedDateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = diplomat::capi::ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianZonedDateTimeFormatter>>(std::unique_ptr<GregorianZonedDateTimeFormatter>(GregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error> GregorianZonedDateTimeFormatter::create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options) {
  auto result = diplomat::capi::ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI(),
    zone_options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianZonedDateTimeFormatter>>(std::unique_ptr<GregorianZonedDateTimeFormatter>(GregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string GregorianZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const IsoDateTime& datetime, const CustomTimeZone& time_zone) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GregorianZonedDateTimeFormatter*>(this);
}

inline diplomat::capi::GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::GregorianZonedDateTimeFormatter*>(this);
}

inline const GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::FromFFI(const diplomat::capi::GregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const GregorianZonedDateTimeFormatter*>(ptr);
}

inline GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::FromFFI(diplomat::capi::GregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<GregorianZonedDateTimeFormatter*>(ptr);
}

inline void GregorianZonedDateTimeFormatter::operator delete(void* ptr) {
  diplomat::capi::ICU4XGregorianZonedDateTimeFormatter_destroy(reinterpret_cast<diplomat::capi::GregorianZonedDateTimeFormatter*>(ptr));
}


#endif // GregorianZonedDateTimeFormatter_HPP

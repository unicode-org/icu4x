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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result {union {GregorianZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result;
    ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_result ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);
    
    typedef struct ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result {union {GregorianZonedDateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result;
    ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_result ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options);
    
    void ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(const GregorianZonedDateTimeFormatter* self, const IsoDateTime* datetime, const CustomTimeZone* time_zone, DiplomatWrite* write);
    
    
    void ICU4XGregorianZonedDateTimeFormatter_destroy(GregorianZonedDateTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error> GregorianZonedDateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = capi::ICU4XGregorianZonedDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianZonedDateTimeFormatter>>(std::unique_ptr<GregorianZonedDateTimeFormatter>(GregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error> GregorianZonedDateTimeFormatter::create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options) {
  auto result = capi::ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI(),
    zone_options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianZonedDateTimeFormatter>>(std::unique_ptr<GregorianZonedDateTimeFormatter>(GregorianZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string GregorianZonedDateTimeFormatter::format_iso_datetime_with_custom_time_zone(const IsoDateTime& datetime, const CustomTimeZone& time_zone) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(this->AsFFI(),
    datetime.AsFFI(),
    time_zone.AsFFI(),
    &write);
  return output;
}

inline const capi::GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::GregorianZonedDateTimeFormatter*>(this);
}

inline capi::GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::GregorianZonedDateTimeFormatter*>(this);
}

inline const GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::FromFFI(const capi::GregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<const GregorianZonedDateTimeFormatter*>(ptr);
}

inline GregorianZonedDateTimeFormatter* GregorianZonedDateTimeFormatter::FromFFI(capi::GregorianZonedDateTimeFormatter* ptr) {
  return reinterpret_cast<GregorianZonedDateTimeFormatter*>(ptr);
}

inline void GregorianZonedDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XGregorianZonedDateTimeFormatter_destroy(reinterpret_cast<capi::GregorianZonedDateTimeFormatter*>(ptr));
}


#endif // GregorianZonedDateTimeFormatter_HPP

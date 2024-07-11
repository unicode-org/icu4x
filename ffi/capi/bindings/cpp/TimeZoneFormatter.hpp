#ifndef TimeZoneFormatter_HPP
#define TimeZoneFormatter_HPP

#include "TimeZoneFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CustomTimeZone.hpp"
#include "DataProvider.hpp"
#include "Error.hpp"
#include "IsoTimeZoneOptions.hpp"
#include "Locale.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result {union {TimeZoneFormatter* ok; Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result;
    ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback(const DataProvider* provider, const Locale* locale);
    
    typedef struct ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result {union {TimeZoneFormatter* ok; Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result;
    ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result ICU4XTimeZoneFormatter_create_with_iso_8601_fallback(const DataProvider* provider, const Locale* locale, IsoTimeZoneOptions options);
    
    typedef struct ICU4XTimeZoneFormatter_load_generic_non_location_long_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_generic_non_location_long_result;
    ICU4XTimeZoneFormatter_load_generic_non_location_long_result ICU4XTimeZoneFormatter_load_generic_non_location_long(TimeZoneFormatter* self, const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneFormatter_load_generic_non_location_short_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_generic_non_location_short_result;
    ICU4XTimeZoneFormatter_load_generic_non_location_short_result ICU4XTimeZoneFormatter_load_generic_non_location_short(TimeZoneFormatter* self, const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneFormatter_load_specific_non_location_long_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_specific_non_location_long_result;
    ICU4XTimeZoneFormatter_load_specific_non_location_long_result ICU4XTimeZoneFormatter_load_specific_non_location_long(TimeZoneFormatter* self, const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneFormatter_load_specific_non_location_short_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_specific_non_location_short_result;
    ICU4XTimeZoneFormatter_load_specific_non_location_short_result ICU4XTimeZoneFormatter_load_specific_non_location_short(TimeZoneFormatter* self, const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneFormatter_load_generic_location_format_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_generic_location_format_result;
    ICU4XTimeZoneFormatter_load_generic_location_format_result ICU4XTimeZoneFormatter_load_generic_location_format(TimeZoneFormatter* self, const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneFormatter_include_localized_gmt_format_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_include_localized_gmt_format_result;
    ICU4XTimeZoneFormatter_include_localized_gmt_format_result ICU4XTimeZoneFormatter_include_localized_gmt_format(TimeZoneFormatter* self);
    
    typedef struct ICU4XTimeZoneFormatter_load_iso_8601_format_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_load_iso_8601_format_result;
    ICU4XTimeZoneFormatter_load_iso_8601_format_result ICU4XTimeZoneFormatter_load_iso_8601_format(TimeZoneFormatter* self, IsoTimeZoneOptions options);
    
    void ICU4XTimeZoneFormatter_format_custom_time_zone(const TimeZoneFormatter* self, const CustomTimeZone* value, DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result {union { Error err;}; bool is_ok;} ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result;
    ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback(const TimeZoneFormatter* self, const CustomTimeZone* value, DiplomatWrite* write);
    
    
    void ICU4XTimeZoneFormatter_destroy(TimeZoneFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error> TimeZoneFormatter::create_with_localized_gmt_fallback(const DataProvider& provider, const Locale& locale) {
  auto result = capi::ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error>(diplomat::Ok<std::unique_ptr<TimeZoneFormatter>>(std::unique_ptr<TimeZoneFormatter>(TimeZoneFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error> TimeZoneFormatter::create_with_iso_8601_fallback(const DataProvider& provider, const Locale& locale, IsoTimeZoneOptions options) {
  auto result = capi::ICU4XTimeZoneFormatter_create_with_iso_8601_fallback(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error>(diplomat::Ok<std::unique_ptr<TimeZoneFormatter>>(std::unique_ptr<TimeZoneFormatter>(TimeZoneFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::load_generic_non_location_long(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_generic_non_location_long(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::load_generic_non_location_short(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_generic_non_location_short(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::load_specific_non_location_long(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_specific_non_location_long(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::load_specific_non_location_short(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_specific_non_location_short(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::load_generic_location_format(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_generic_location_format(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::include_localized_gmt_format() {
  auto result = capi::ICU4XTimeZoneFormatter_include_localized_gmt_format(this->AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, Error> TimeZoneFormatter::load_iso_8601_format(IsoTimeZoneOptions options) {
  auto result = capi::ICU4XTimeZoneFormatter_load_iso_8601_format(this->AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string TimeZoneFormatter::format_custom_time_zone(const CustomTimeZone& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XTimeZoneFormatter_format_custom_time_zone(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::string, Error> TimeZoneFormatter::format_custom_time_zone_no_fallback(const CustomTimeZone& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline const capi::TimeZoneFormatter* TimeZoneFormatter::AsFFI() const {
  return reinterpret_cast<const capi::TimeZoneFormatter*>(this);
}

inline capi::TimeZoneFormatter* TimeZoneFormatter::AsFFI() {
  return reinterpret_cast<capi::TimeZoneFormatter*>(this);
}

inline const TimeZoneFormatter* TimeZoneFormatter::FromFFI(const capi::TimeZoneFormatter* ptr) {
  return reinterpret_cast<const TimeZoneFormatter*>(ptr);
}

inline TimeZoneFormatter* TimeZoneFormatter::FromFFI(capi::TimeZoneFormatter* ptr) {
  return reinterpret_cast<TimeZoneFormatter*>(ptr);
}

inline void TimeZoneFormatter::operator delete(void* ptr) {
  capi::ICU4XTimeZoneFormatter_destroy(reinterpret_cast<capi::TimeZoneFormatter*>(ptr));
}


#endif // TimeZoneFormatter_HPP

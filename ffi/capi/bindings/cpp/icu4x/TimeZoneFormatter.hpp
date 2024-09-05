#ifndef icu4x_TimeZoneFormatter_HPP
#define icu4x_TimeZoneFormatter_HPP

#include "TimeZoneFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "CustomTimeZone.hpp"
#include "DataProvider.hpp"
#include "Error.hpp"
#include "IsoTimeZoneOptions.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1_result {union {icu4x::capi::TimeZoneFormatter* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1_result;
    icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1_result icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    typedef struct icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1_result {union {icu4x::capi::TimeZoneFormatter* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1_result;
    icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1_result icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::IsoTimeZoneOptions options);
    
    typedef struct icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1_result;
    icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1_result icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1(icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1_result;
    icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1_result icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1(icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1_result;
    icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1_result icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1(icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1_result;
    icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1_result icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1(icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_TimeZoneFormatter_load_generic_location_format_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_generic_location_format_mv1_result;
    icu4x_TimeZoneFormatter_load_generic_location_format_mv1_result icu4x_TimeZoneFormatter_load_generic_location_format_mv1(icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_TimeZoneFormatter_include_localized_offset_format_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_include_localized_offset_format_mv1_result;
    icu4x_TimeZoneFormatter_include_localized_offset_format_mv1_result icu4x_TimeZoneFormatter_include_localized_offset_format_mv1(icu4x::capi::TimeZoneFormatter* self);
    
    typedef struct icu4x_TimeZoneFormatter_load_iso_8601_format_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_load_iso_8601_format_mv1_result;
    icu4x_TimeZoneFormatter_load_iso_8601_format_mv1_result icu4x_TimeZoneFormatter_load_iso_8601_format_mv1(icu4x::capi::TimeZoneFormatter* self, icu4x::capi::IsoTimeZoneOptions options);
    
    void icu4x_TimeZoneFormatter_format_custom_time_zone_mv1(const icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::CustomTimeZone* value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1_result;
    icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1_result icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1(const icu4x::capi::TimeZoneFormatter* self, const icu4x::capi::CustomTimeZone* value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_TimeZoneFormatter_destroy_mv1(TimeZoneFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error> icu4x::TimeZoneFormatter::create_with_localized_offset_fallback(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_create_with_localized_offset_fallback_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::TimeZoneFormatter>>(std::unique_ptr<icu4x::TimeZoneFormatter>(icu4x::TimeZoneFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error> icu4x::TimeZoneFormatter::create_with_iso_8601_fallback(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::IsoTimeZoneOptions options) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_create_with_iso_8601_fallback_mv1(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::TimeZoneFormatter>>(std::unique_ptr<icu4x::TimeZoneFormatter>(icu4x::TimeZoneFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::load_generic_non_location_long(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_load_generic_non_location_long_mv1(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::load_generic_non_location_short(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_load_generic_non_location_short_mv1(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::load_specific_non_location_long(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_load_specific_non_location_long_mv1(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::load_specific_non_location_short(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_load_specific_non_location_short_mv1(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::load_generic_location_format(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_load_generic_location_format_mv1(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::include_localized_offset_format() {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_include_localized_offset_format_mv1(this->AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::Error> icu4x::TimeZoneFormatter::load_iso_8601_format(icu4x::IsoTimeZoneOptions options) {
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_load_iso_8601_format_mv1(this->AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::Error>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline std::string icu4x::TimeZoneFormatter::format_custom_time_zone(const icu4x::CustomTimeZone& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_TimeZoneFormatter_format_custom_time_zone_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::string, icu4x::Error> icu4x::TimeZoneFormatter::format_custom_time_zone_no_fallback(const icu4x::CustomTimeZone& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_TimeZoneFormatter_format_custom_time_zone_no_fallback_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline const icu4x::capi::TimeZoneFormatter* icu4x::TimeZoneFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::TimeZoneFormatter*>(this);
}

inline icu4x::capi::TimeZoneFormatter* icu4x::TimeZoneFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::TimeZoneFormatter*>(this);
}

inline const icu4x::TimeZoneFormatter* icu4x::TimeZoneFormatter::FromFFI(const icu4x::capi::TimeZoneFormatter* ptr) {
  return reinterpret_cast<const icu4x::TimeZoneFormatter*>(ptr);
}

inline icu4x::TimeZoneFormatter* icu4x::TimeZoneFormatter::FromFFI(icu4x::capi::TimeZoneFormatter* ptr) {
  return reinterpret_cast<icu4x::TimeZoneFormatter*>(ptr);
}

inline void icu4x::TimeZoneFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_TimeZoneFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::TimeZoneFormatter*>(ptr));
}


#endif // icu4x_TimeZoneFormatter_HPP

#ifndef ICU4XTimeZoneFormatter_HPP
#define ICU4XTimeZoneFormatter_HPP

#include "ICU4XTimeZoneFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCustomTimeZone.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoTimeZoneOptions.hpp"
#include "ICU4XLocale.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result {union {ICU4XTimeZoneFormatter* ok; ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback_result ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale);
    
    struct ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result {union {ICU4XTimeZoneFormatter* ok; ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_create_with_iso_8601_fallback_result ICU4XTimeZoneFormatter_create_with_iso_8601_fallback(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XIsoTimeZoneOptions options);
    
    struct ICU4XTimeZoneFormatter_load_generic_non_location_long_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_load_generic_non_location_long_result ICU4XTimeZoneFormatter_load_generic_non_location_long(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);
    
    struct ICU4XTimeZoneFormatter_load_generic_non_location_short_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_load_generic_non_location_short_result ICU4XTimeZoneFormatter_load_generic_non_location_short(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);
    
    struct ICU4XTimeZoneFormatter_load_specific_non_location_long_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_load_specific_non_location_long_result ICU4XTimeZoneFormatter_load_specific_non_location_long(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);
    
    struct ICU4XTimeZoneFormatter_load_specific_non_location_short_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_load_specific_non_location_short_result ICU4XTimeZoneFormatter_load_specific_non_location_short(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);
    
    struct ICU4XTimeZoneFormatter_load_generic_location_format_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_load_generic_location_format_result ICU4XTimeZoneFormatter_load_generic_location_format(ICU4XTimeZoneFormatter* self, const ICU4XDataProvider* provider);
    
    struct ICU4XTimeZoneFormatter_include_localized_gmt_format_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_include_localized_gmt_format_result ICU4XTimeZoneFormatter_include_localized_gmt_format(ICU4XTimeZoneFormatter* self);
    
    struct ICU4XTimeZoneFormatter_load_iso_8601_format_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_load_iso_8601_format_result ICU4XTimeZoneFormatter_load_iso_8601_format(ICU4XTimeZoneFormatter* self, ICU4XIsoTimeZoneOptions options);
    
    void ICU4XTimeZoneFormatter_format_custom_time_zone(const ICU4XTimeZoneFormatter* self, const ICU4XCustomTimeZone* value, DiplomatWrite* write);
    
    struct ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback_result ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback(const ICU4XTimeZoneFormatter* self, const ICU4XCustomTimeZone* value, DiplomatWrite* write);
    
    
    void ICU4XTimeZoneFormatter_destroy(ICU4XTimeZoneFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError> ICU4XTimeZoneFormatter::create_with_localized_gmt_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XTimeZoneFormatter>>(std::unique_ptr<ICU4XTimeZoneFormatter>(ICU4XTimeZoneFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError> ICU4XTimeZoneFormatter::create_with_iso_8601_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XIsoTimeZoneOptions options) {
  auto result = capi::ICU4XTimeZoneFormatter_create_with_iso_8601_fallback(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XTimeZoneFormatter>>(std::unique_ptr<ICU4XTimeZoneFormatter>(ICU4XTimeZoneFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::load_generic_non_location_long(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_generic_non_location_long(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::load_generic_non_location_short(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_generic_non_location_short(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::load_specific_non_location_long(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_specific_non_location_long(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::load_specific_non_location_short(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_specific_non_location_short(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::load_generic_location_format(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneFormatter_load_generic_location_format(this->AsFFI(),
    provider.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::include_localized_gmt_format() {
  auto result = capi::ICU4XTimeZoneFormatter_include_localized_gmt_format(this->AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::load_iso_8601_format(ICU4XIsoTimeZoneOptions options) {
  auto result = capi::ICU4XTimeZoneFormatter_load_iso_8601_format(this->AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::string ICU4XTimeZoneFormatter::format_custom_time_zone(const ICU4XCustomTimeZone& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XTimeZoneFormatter_format_custom_time_zone(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::string, ICU4XError> ICU4XTimeZoneFormatter::format_custom_time_zone_no_fallback(const ICU4XCustomTimeZone& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneFormatter_format_custom_time_zone_no_fallback(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XTimeZoneFormatter* ICU4XTimeZoneFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XTimeZoneFormatter*>(this);
}

inline capi::ICU4XTimeZoneFormatter* ICU4XTimeZoneFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XTimeZoneFormatter*>(this);
}

inline const ICU4XTimeZoneFormatter* ICU4XTimeZoneFormatter::FromFFI(const capi::ICU4XTimeZoneFormatter* ptr) {
  return reinterpret_cast<const ICU4XTimeZoneFormatter*>(ptr);
}

inline ICU4XTimeZoneFormatter* ICU4XTimeZoneFormatter::FromFFI(capi::ICU4XTimeZoneFormatter* ptr) {
  return reinterpret_cast<ICU4XTimeZoneFormatter*>(ptr);
}

inline void ICU4XTimeZoneFormatter::operator delete(void* ptr) {
  capi::ICU4XTimeZoneFormatter_destroy(reinterpret_cast<capi::ICU4XTimeZoneFormatter*>(ptr));
}


#endif // ICU4XTimeZoneFormatter_HPP

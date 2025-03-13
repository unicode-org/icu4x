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
#include "DataProvider.hpp"
#include "DateTimeFormatter.hpp"
#include "DateTimeFormatterLoadError.hpp"
#include "DateTimeWriteError.hpp"
#include "IsoDate.hpp"
#include "Locale.hpp"
#include "Time.hpp"
#include "TimeZoneInfo.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_generic_short_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_generic_short_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_generic_short_mv1_result icu4x_NeoZonedDateTimeFormatter_create_generic_short_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_generic_short_with_provider_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_generic_short_with_provider_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_generic_short_with_provider_mv1_result icu4x_NeoZonedDateTimeFormatter_create_generic_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_generic_long_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_generic_long_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_generic_long_mv1_result icu4x_NeoZonedDateTimeFormatter_create_generic_long_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_generic_long_with_provider_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_generic_long_with_provider_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_generic_long_with_provider_mv1_result icu4x_NeoZonedDateTimeFormatter_create_generic_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_specific_short_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_specific_short_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_specific_short_mv1_result icu4x_NeoZonedDateTimeFormatter_create_specific_short_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_specific_short_with_provider_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_specific_short_with_provider_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_specific_short_with_provider_mv1_result icu4x_NeoZonedDateTimeFormatter_create_specific_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_specific_long_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_specific_long_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_specific_long_mv1_result icu4x_NeoZonedDateTimeFormatter_create_specific_long_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_specific_long_with_provider_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_specific_long_with_provider_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_specific_long_with_provider_mv1_result icu4x_NeoZonedDateTimeFormatter_create_specific_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_mv1_result icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1_result icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_mv1_result icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1_result {union {icu4x::capi::NeoZonedDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1_result icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateTimeFormatter* formatter);
    
    typedef struct icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result {union { icu4x::capi::DateTimeWriteError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result;
    icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result icu4x_NeoZonedDateTimeFormatter_format_iso_mv1(const icu4x::capi::NeoZonedDateTimeFormatter* self, const icu4x::capi::IsoDate* date, const icu4x::capi::Time* time, const icu4x::capi::TimeZoneInfo* zone, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_NeoZonedDateTimeFormatter_destroy_mv1(NeoZonedDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_generic_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_generic_short_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_generic_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_generic_short_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_generic_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_generic_long_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_generic_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_generic_long_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_specific_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_specific_short_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_specific_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_specific_short_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_specific_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_specific_long_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_specific_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_specific_long_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_localized_offset_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_localized_offset_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_localized_offset_short_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_localized_offset_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoZonedDateTimeFormatter::create_localized_offset_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatter& formatter) {
  auto result = icu4x::capi::icu4x_NeoZonedDateTimeFormatter_create_localized_offset_long_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>>(std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>(icu4x::NeoZonedDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

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

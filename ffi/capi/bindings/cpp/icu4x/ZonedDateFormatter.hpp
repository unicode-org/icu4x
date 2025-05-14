#ifndef icu4x_ZonedDateFormatter_HPP
#define icu4x_ZonedDateFormatter_HPP

#include "ZonedDateFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateFormatter.hpp"
#include "DateTimeFormatterLoadError.hpp"
#include "DateTimeWriteError.hpp"
#include "IsoDate.hpp"
#include "Locale.hpp"
#include "TimeZoneInfo.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_ZonedDateFormatter_create_specific_long_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_specific_long_mv1_result;
    icu4x_ZonedDateFormatter_create_specific_long_mv1_result icu4x_ZonedDateFormatter_create_specific_long_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_specific_long_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_specific_long_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_specific_long_with_provider_mv1_result icu4x_ZonedDateFormatter_create_specific_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_specific_short_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_specific_short_mv1_result;
    icu4x_ZonedDateFormatter_create_specific_short_mv1_result icu4x_ZonedDateFormatter_create_specific_short_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_specific_short_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_specific_short_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_specific_short_with_provider_mv1_result icu4x_ZonedDateFormatter_create_specific_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_localized_offset_long_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_localized_offset_long_mv1_result;
    icu4x_ZonedDateFormatter_create_localized_offset_long_mv1_result icu4x_ZonedDateFormatter_create_localized_offset_long_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_localized_offset_long_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_localized_offset_long_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_localized_offset_long_with_provider_mv1_result icu4x_ZonedDateFormatter_create_localized_offset_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_localized_offset_short_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_localized_offset_short_mv1_result;
    icu4x_ZonedDateFormatter_create_localized_offset_short_mv1_result icu4x_ZonedDateFormatter_create_localized_offset_short_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_localized_offset_short_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_localized_offset_short_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_localized_offset_short_with_provider_mv1_result icu4x_ZonedDateFormatter_create_localized_offset_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_generic_long_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_generic_long_mv1_result;
    icu4x_ZonedDateFormatter_create_generic_long_mv1_result icu4x_ZonedDateFormatter_create_generic_long_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_generic_long_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_generic_long_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_generic_long_with_provider_mv1_result icu4x_ZonedDateFormatter_create_generic_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_generic_short_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_generic_short_mv1_result;
    icu4x_ZonedDateFormatter_create_generic_short_mv1_result icu4x_ZonedDateFormatter_create_generic_short_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_generic_short_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_generic_short_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_generic_short_with_provider_mv1_result icu4x_ZonedDateFormatter_create_generic_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_location_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_location_mv1_result;
    icu4x_ZonedDateFormatter_create_location_mv1_result icu4x_ZonedDateFormatter_create_location_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_location_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_location_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_location_with_provider_mv1_result icu4x_ZonedDateFormatter_create_location_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_exemplar_city_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_exemplar_city_mv1_result;
    icu4x_ZonedDateFormatter_create_exemplar_city_mv1_result icu4x_ZonedDateFormatter_create_exemplar_city_mv1(const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_create_exemplar_city_with_provider_mv1_result {union {icu4x::capi::ZonedDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_ZonedDateFormatter_create_exemplar_city_with_provider_mv1_result;
    icu4x_ZonedDateFormatter_create_exemplar_city_with_provider_mv1_result icu4x_ZonedDateFormatter_create_exemplar_city_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, const icu4x::capi::DateFormatter* formatter);

    typedef struct icu4x_ZonedDateFormatter_format_iso_mv1_result {union { icu4x::capi::DateTimeWriteError err;}; bool is_ok;} icu4x_ZonedDateFormatter_format_iso_mv1_result;
    icu4x_ZonedDateFormatter_format_iso_mv1_result icu4x_ZonedDateFormatter_format_iso_mv1(const icu4x::capi::ZonedDateFormatter* self, const icu4x::capi::IsoDate* iso_date, const icu4x::capi::TimeZoneInfo* zone, diplomat::capi::DiplomatWrite* write);

    void icu4x_ZonedDateFormatter_destroy_mv1(ZonedDateFormatter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_specific_long(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_specific_long_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_specific_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_specific_long_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_specific_short(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_specific_short_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_specific_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_specific_short_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_localized_offset_long(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_localized_offset_long_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_localized_offset_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_localized_offset_long_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_localized_offset_short(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_localized_offset_short_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_localized_offset_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_localized_offset_short_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_generic_long(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_generic_long_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_generic_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_generic_long_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_generic_short(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_generic_short_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_generic_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_generic_short_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_location(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_location_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_location_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_location_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_exemplar_city(const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_exemplar_city_mv1(locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::ZonedDateFormatter::create_exemplar_city_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateFormatter& formatter) {
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_create_exemplar_city_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    formatter.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateFormatter>>(std::unique_ptr<icu4x::ZonedDateFormatter>(icu4x::ZonedDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::string, icu4x::DateTimeWriteError> icu4x::ZonedDateFormatter::format_iso(const icu4x::IsoDate& iso_date, const icu4x::TimeZoneInfo& zone) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_ZonedDateFormatter_format_iso_mv1(this->AsFFI(),
    iso_date.AsFFI(),
    zone.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::DateTimeWriteError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::DateTimeWriteError>(diplomat::Err<icu4x::DateTimeWriteError>(icu4x::DateTimeWriteError::FromFFI(result.err)));
}

inline const icu4x::capi::ZonedDateFormatter* icu4x::ZonedDateFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ZonedDateFormatter*>(this);
}

inline icu4x::capi::ZonedDateFormatter* icu4x::ZonedDateFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::ZonedDateFormatter*>(this);
}

inline const icu4x::ZonedDateFormatter* icu4x::ZonedDateFormatter::FromFFI(const icu4x::capi::ZonedDateFormatter* ptr) {
  return reinterpret_cast<const icu4x::ZonedDateFormatter*>(ptr);
}

inline icu4x::ZonedDateFormatter* icu4x::ZonedDateFormatter::FromFFI(icu4x::capi::ZonedDateFormatter* ptr) {
  return reinterpret_cast<icu4x::ZonedDateFormatter*>(ptr);
}

inline void icu4x::ZonedDateFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_ZonedDateFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::ZonedDateFormatter*>(ptr));
}


#endif // icu4x_ZonedDateFormatter_HPP

#ifndef icu4x_DateTimeFormatter_HPP
#define icu4x_DateTimeFormatter_HPP

#include "DateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "Calendar.hpp"
#include "DataProvider.hpp"
#include "Date.hpp"
#include "DateTimeFormatError.hpp"
#include "DateTimeFormatterLoadError.hpp"
#include "DateTimeLength.hpp"
#include "IsoDate.hpp"
#include "Locale.hpp"
#include "Time.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_DateTimeFormatter_create_with_length_mv1_result {union {icu4x::capi::DateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_with_length_mv1_result;
    icu4x_DateTimeFormatter_create_with_length_mv1_result icu4x_DateTimeFormatter_create_with_length_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength length);
    
    typedef struct icu4x_DateTimeFormatter_create_with_length_and_provider_mv1_result {union {icu4x::capi::DateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_DateTimeFormatter_create_with_length_and_provider_mv1_result;
    icu4x_DateTimeFormatter_create_with_length_and_provider_mv1_result icu4x_DateTimeFormatter_create_with_length_and_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength length);
    
    typedef struct icu4x_DateTimeFormatter_format_datetime_mv1_result {union { icu4x::capi::DateTimeFormatError err;}; bool is_ok;} icu4x_DateTimeFormatter_format_datetime_mv1_result;
    icu4x_DateTimeFormatter_format_datetime_mv1_result icu4x_DateTimeFormatter_format_datetime_mv1(const icu4x::capi::DateTimeFormatter* self, const icu4x::capi::Date* date, const icu4x::capi::Time* time, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_DateTimeFormatter_format_iso_datetime_mv1_result {union { icu4x::capi::DateTimeFormatError err;}; bool is_ok;} icu4x_DateTimeFormatter_format_iso_datetime_mv1_result;
    icu4x_DateTimeFormatter_format_iso_datetime_mv1_result icu4x_DateTimeFormatter_format_iso_datetime_mv1(const icu4x::capi::DateTimeFormatter* self, const icu4x::capi::IsoDate* date, const icu4x::capi::Time* time, diplomat::capi::DiplomatWrite* write);
    
    icu4x::capi::Calendar* icu4x_DateTimeFormatter_calendar_mv1(const icu4x::capi::DateTimeFormatter* self);
    
    
    void icu4x_DateTimeFormatter_destroy_mv1(DateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::DateTimeFormatter::create_with_length(const icu4x::Locale& locale, icu4x::DateTimeLength length) {
  auto result = icu4x::capi::icu4x_DateTimeFormatter_create_with_length_mv1(locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::DateTimeFormatter>>(std::unique_ptr<icu4x::DateTimeFormatter>(icu4x::DateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::DateTimeFormatter::create_with_length_and_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DateTimeLength length) {
  auto result = icu4x::capi::icu4x_DateTimeFormatter_create_with_length_and_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::DateTimeFormatter>>(std::unique_ptr<icu4x::DateTimeFormatter>(icu4x::DateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::string, icu4x::DateTimeFormatError> icu4x::DateTimeFormatter::format_datetime(const icu4x::Date& date, const icu4x::Time& time) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_DateTimeFormatter_format_datetime_mv1(this->AsFFI(),
    date.AsFFI(),
    time.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::DateTimeFormatError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::DateTimeFormatError>(diplomat::Err<icu4x::DateTimeFormatError>(icu4x::DateTimeFormatError::FromFFI(result.err)));
}

inline diplomat::result<std::string, icu4x::DateTimeFormatError> icu4x::DateTimeFormatter::format_iso_datetime(const icu4x::IsoDate& date, const icu4x::Time& time) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_DateTimeFormatter_format_iso_datetime_mv1(this->AsFFI(),
    date.AsFFI(),
    time.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::DateTimeFormatError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::DateTimeFormatError>(diplomat::Err<icu4x::DateTimeFormatError>(icu4x::DateTimeFormatError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::Calendar> icu4x::DateTimeFormatter::calendar() const {
  auto result = icu4x::capi::icu4x_DateTimeFormatter_calendar_mv1(this->AsFFI());
  return std::unique_ptr<icu4x::Calendar>(icu4x::Calendar::FromFFI(result));
}

inline const icu4x::capi::DateTimeFormatter* icu4x::DateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::DateTimeFormatter*>(this);
}

inline icu4x::capi::DateTimeFormatter* icu4x::DateTimeFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::DateTimeFormatter*>(this);
}

inline const icu4x::DateTimeFormatter* icu4x::DateTimeFormatter::FromFFI(const icu4x::capi::DateTimeFormatter* ptr) {
  return reinterpret_cast<const icu4x::DateTimeFormatter*>(ptr);
}

inline icu4x::DateTimeFormatter* icu4x::DateTimeFormatter::FromFFI(icu4x::capi::DateTimeFormatter* ptr) {
  return reinterpret_cast<icu4x::DateTimeFormatter*>(ptr);
}

inline void icu4x::DateTimeFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_DateTimeFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::DateTimeFormatter*>(ptr));
}


#endif // icu4x_DateTimeFormatter_HPP

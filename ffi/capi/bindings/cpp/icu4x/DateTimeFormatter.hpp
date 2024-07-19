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
#include "DataProvider.hpp"
#include "DateLength.hpp"
#include "DateTime.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "TimeLength.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_DateTimeFormatter_create_with_lengths_mv1_result {union {icu4x::capi::DateTimeFormatter* ok; icu4x::capi::Error err;}; bool is_ok;} icu4x_DateTimeFormatter_create_with_lengths_mv1_result;
    icu4x_DateTimeFormatter_create_with_lengths_mv1_result icu4x_DateTimeFormatter_create_with_lengths_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateLength date_length, icu4x::capi::TimeLength time_length);
    
    typedef struct icu4x_DateTimeFormatter_format_datetime_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_DateTimeFormatter_format_datetime_mv1_result;
    icu4x_DateTimeFormatter_format_datetime_mv1_result icu4x_DateTimeFormatter_format_datetime_mv1(const icu4x::capi::DateTimeFormatter* self, const icu4x::capi::DateTime* value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_DateTimeFormatter_format_iso_datetime_mv1_result {union { icu4x::capi::Error err;}; bool is_ok;} icu4x_DateTimeFormatter_format_iso_datetime_mv1_result;
    icu4x_DateTimeFormatter_format_iso_datetime_mv1_result icu4x_DateTimeFormatter_format_iso_datetime_mv1(const icu4x::capi::DateTimeFormatter* self, const icu4x::capi::IsoDateTime* value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_DateTimeFormatter_destroy_mv1(DateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::Error> icu4x::DateTimeFormatter::create_with_lengths(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DateLength date_length, icu4x::TimeLength time_length) {
  auto result = icu4x::capi::icu4x_DateTimeFormatter_create_with_lengths_mv1(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::Error>(diplomat::Ok<std::unique_ptr<icu4x::DateTimeFormatter>>(std::unique_ptr<icu4x::DateTimeFormatter>(icu4x::DateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DateTimeFormatter>, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, icu4x::Error> icu4x::DateTimeFormatter::format_datetime(const icu4x::DateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_DateTimeFormatter_format_datetime_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, icu4x::Error> icu4x::DateTimeFormatter::format_iso_datetime(const icu4x::IsoDateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_DateTimeFormatter_format_iso_datetime_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::Error>(diplomat::Err<icu4x::Error>(icu4x::Error::FromFFI(result.err)));
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

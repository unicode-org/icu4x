#ifndef DateFormatter_HPP
#define DateFormatter_HPP

#include "DateFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "Date.hpp"
#include "DateLength.hpp"
#include "DateTime.hpp"
#include "Error.hpp"
#include "IsoDate.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XDateFormatter_create_with_length_result {union {diplomat::capi::DateFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XDateFormatter_create_with_length_result;
    ICU4XDateFormatter_create_with_length_result ICU4XDateFormatter_create_with_length(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength date_length);
    
    typedef struct ICU4XDateFormatter_format_date_result {union { diplomat::capi::Error err;}; bool is_ok;} ICU4XDateFormatter_format_date_result;
    ICU4XDateFormatter_format_date_result ICU4XDateFormatter_format_date(const diplomat::capi::DateFormatter* self, const diplomat::capi::Date* value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XDateFormatter_format_iso_date_result {union { diplomat::capi::Error err;}; bool is_ok;} ICU4XDateFormatter_format_iso_date_result;
    ICU4XDateFormatter_format_iso_date_result ICU4XDateFormatter_format_iso_date(const diplomat::capi::DateFormatter* self, const diplomat::capi::IsoDate* value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XDateFormatter_format_datetime_result {union { diplomat::capi::Error err;}; bool is_ok;} ICU4XDateFormatter_format_datetime_result;
    ICU4XDateFormatter_format_datetime_result ICU4XDateFormatter_format_datetime(const diplomat::capi::DateFormatter* self, const diplomat::capi::DateTime* value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XDateFormatter_format_iso_datetime_result {union { diplomat::capi::Error err;}; bool is_ok;} ICU4XDateFormatter_format_iso_datetime_result;
    ICU4XDateFormatter_format_iso_datetime_result ICU4XDateFormatter_format_iso_datetime(const diplomat::capi::DateFormatter* self, const diplomat::capi::IsoDateTime* value, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XDateFormatter_destroy(DateFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<DateFormatter>, Error> DateFormatter::create_with_length(const DataProvider& provider, const Locale& locale, DateLength date_length) {
  auto result = diplomat::capi::ICU4XDateFormatter_create_with_length(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DateFormatter>, Error>(diplomat::Ok<std::unique_ptr<DateFormatter>>(std::unique_ptr<DateFormatter>(DateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DateFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> DateFormatter::format_date(const Date& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XDateFormatter_format_date(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> DateFormatter::format_iso_date(const IsoDate& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XDateFormatter_format_iso_date(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> DateFormatter::format_datetime(const DateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XDateFormatter_format_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> DateFormatter::format_iso_datetime(const IsoDateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XDateFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline const diplomat::capi::DateFormatter* DateFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::DateFormatter*>(this);
}

inline diplomat::capi::DateFormatter* DateFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::DateFormatter*>(this);
}

inline const DateFormatter* DateFormatter::FromFFI(const diplomat::capi::DateFormatter* ptr) {
  return reinterpret_cast<const DateFormatter*>(ptr);
}

inline DateFormatter* DateFormatter::FromFFI(diplomat::capi::DateFormatter* ptr) {
  return reinterpret_cast<DateFormatter*>(ptr);
}

inline void DateFormatter::operator delete(void* ptr) {
  diplomat::capi::ICU4XDateFormatter_destroy(reinterpret_cast<diplomat::capi::DateFormatter*>(ptr));
}


#endif // DateFormatter_HPP

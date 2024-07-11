#ifndef DateTimeFormatter_HPP
#define DateTimeFormatter_HPP

#include "DateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateLength.hpp"
#include "DateTime.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "TimeLength.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XDateTimeFormatter_create_with_lengths_result {union {DateTimeFormatter* ok; Error err;}; bool is_ok;} ICU4XDateTimeFormatter_create_with_lengths_result;
    ICU4XDateTimeFormatter_create_with_lengths_result ICU4XDateTimeFormatter_create_with_lengths(const DataProvider* provider, const Locale* locale, DateLength date_length, TimeLength time_length);
    
    typedef struct ICU4XDateTimeFormatter_format_datetime_result {union { Error err;}; bool is_ok;} ICU4XDateTimeFormatter_format_datetime_result;
    ICU4XDateTimeFormatter_format_datetime_result ICU4XDateTimeFormatter_format_datetime(const DateTimeFormatter* self, const DateTime* value, DiplomatWrite* write);
    
    typedef struct ICU4XDateTimeFormatter_format_iso_datetime_result {union { Error err;}; bool is_ok;} ICU4XDateTimeFormatter_format_iso_datetime_result;
    ICU4XDateTimeFormatter_format_iso_datetime_result ICU4XDateTimeFormatter_format_iso_datetime(const DateTimeFormatter* self, const IsoDateTime* value, DiplomatWrite* write);
    
    
    void ICU4XDateTimeFormatter_destroy(DateTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<DateTimeFormatter>, Error> DateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = capi::ICU4XDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<DateTimeFormatter>>(std::unique_ptr<DateTimeFormatter>(DateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> DateTimeFormatter::format_datetime(const DateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateTimeFormatter_format_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline diplomat::result<std::string, Error> DateTimeFormatter::format_iso_datetime(const IsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateTimeFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, Error>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline const capi::DateTimeFormatter* DateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::DateTimeFormatter*>(this);
}

inline capi::DateTimeFormatter* DateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::DateTimeFormatter*>(this);
}

inline const DateTimeFormatter* DateTimeFormatter::FromFFI(const capi::DateTimeFormatter* ptr) {
  return reinterpret_cast<const DateTimeFormatter*>(ptr);
}

inline DateTimeFormatter* DateTimeFormatter::FromFFI(capi::DateTimeFormatter* ptr) {
  return reinterpret_cast<DateTimeFormatter*>(ptr);
}

inline void DateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XDateTimeFormatter_destroy(reinterpret_cast<capi::DateTimeFormatter*>(ptr));
}


#endif // DateTimeFormatter_HPP

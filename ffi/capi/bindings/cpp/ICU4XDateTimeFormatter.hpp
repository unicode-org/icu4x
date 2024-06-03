#ifndef ICU4XDateTimeFormatter_HPP
#define ICU4XDateTimeFormatter_HPP

#include "ICU4XDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDateLength.hpp"
#include "ICU4XDateTime.hpp"
#include "ICU4XDateTimeFormatter.h"
#include "ICU4XError.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XTimeLength.hpp"


inline diplomat::result<std::unique_ptr<ICU4XDateTimeFormatter>, ICU4XError> ICU4XDateTimeFormatter::create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length) {
  auto result = capi::ICU4XDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDateTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XDateTimeFormatter>>(std::unique_ptr<ICU4XDateTimeFormatter>(ICU4XDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDateTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XDateTimeFormatter::format_datetime(const ICU4XDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateTimeFormatter_format_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XDateTimeFormatter::format_iso_datetime(const ICU4XIsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateTimeFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XDateTimeFormatter* ICU4XDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDateTimeFormatter*>(this);
}

inline capi::ICU4XDateTimeFormatter* ICU4XDateTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XDateTimeFormatter*>(this);
}

inline const ICU4XDateTimeFormatter* ICU4XDateTimeFormatter::FromFFI(const capi::ICU4XDateTimeFormatter* ptr) {
  return reinterpret_cast<const ICU4XDateTimeFormatter*>(ptr);
}

inline ICU4XDateTimeFormatter* ICU4XDateTimeFormatter::FromFFI(capi::ICU4XDateTimeFormatter* ptr) {
  return reinterpret_cast<ICU4XDateTimeFormatter*>(ptr);
}

inline void ICU4XDateTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XDateTimeFormatter_destroy(reinterpret_cast<capi::ICU4XDateTimeFormatter*>(ptr));
}


#endif // ICU4XDateTimeFormatter_HPP

#ifndef ICU4XDateFormatter_HPP
#define ICU4XDateFormatter_HPP

#include "ICU4XDateFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDate.hpp"
#include "ICU4XDateLength.hpp"
#include "ICU4XDateTime.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoDate.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XLocale.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XDateFormatter_create_with_length_result {union {ICU4XDateFormatter* ok; ICU4XError err;}; bool is_ok;};
    struct ICU4XDateFormatter_create_with_length_result ICU4XDateFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length);
    
    struct ICU4XDateFormatter_format_date_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XDateFormatter_format_date_result ICU4XDateFormatter_format_date(const ICU4XDateFormatter* self, const ICU4XDate* value, DiplomatWrite* write);
    
    struct ICU4XDateFormatter_format_iso_date_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XDateFormatter_format_iso_date_result ICU4XDateFormatter_format_iso_date(const ICU4XDateFormatter* self, const ICU4XIsoDate* value, DiplomatWrite* write);
    
    struct ICU4XDateFormatter_format_datetime_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XDateFormatter_format_datetime_result ICU4XDateFormatter_format_datetime(const ICU4XDateFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);
    
    struct ICU4XDateFormatter_format_iso_datetime_result {union { ICU4XError err;}; bool is_ok;};
    struct ICU4XDateFormatter_format_iso_datetime_result ICU4XDateFormatter_format_iso_datetime(const ICU4XDateFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);
    
    
    void ICU4XDateFormatter_destroy(ICU4XDateFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XDateFormatter>, ICU4XError> ICU4XDateFormatter::create_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length) {
  auto result = capi::ICU4XDateFormatter_create_with_length(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDateFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XDateFormatter>>(std::unique_ptr<ICU4XDateFormatter>(ICU4XDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDateFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XDateFormatter::format_date(const ICU4XDate& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateFormatter_format_date(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XDateFormatter::format_iso_date(const ICU4XIsoDate& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateFormatter_format_iso_date(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XDateFormatter::format_datetime(const ICU4XDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateFormatter_format_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XDateFormatter::format_iso_datetime(const ICU4XIsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XDateFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XDateFormatter* ICU4XDateFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDateFormatter*>(this);
}

inline capi::ICU4XDateFormatter* ICU4XDateFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XDateFormatter*>(this);
}

inline const ICU4XDateFormatter* ICU4XDateFormatter::FromFFI(const capi::ICU4XDateFormatter* ptr) {
  return reinterpret_cast<const ICU4XDateFormatter*>(ptr);
}

inline ICU4XDateFormatter* ICU4XDateFormatter::FromFFI(capi::ICU4XDateFormatter* ptr) {
  return reinterpret_cast<ICU4XDateFormatter*>(ptr);
}

inline void ICU4XDateFormatter::operator delete(void* ptr) {
  capi::ICU4XDateFormatter_destroy(reinterpret_cast<capi::ICU4XDateFormatter*>(ptr));
}


#endif // ICU4XDateFormatter_HPP

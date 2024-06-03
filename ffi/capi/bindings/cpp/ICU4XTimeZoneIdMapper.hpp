#ifndef ICU4XTimeZoneIdMapper_HPP
#define ICU4XTimeZoneIdMapper_HPP

#include "ICU4XTimeZoneIdMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XTimeZoneIdMapper.h"


inline diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapper>, ICU4XError> ICU4XTimeZoneIdMapper::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneIdMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapper>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XTimeZoneIdMapper>>(std::unique_ptr<ICU4XTimeZoneIdMapper>(ICU4XTimeZoneIdMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapper>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XTimeZoneIdMapper::iana_to_bcp47(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_iana_to_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::string, ICU4XError>, diplomat::Utf8Error> ICU4XTimeZoneIdMapper::normalize_iana(std::string_view value) const {
  if (!capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_normalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, ICU4XError>>(std::move(result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)))));
}

inline diplomat::result<diplomat::result<std::string, ICU4XError>, diplomat::Utf8Error> ICU4XTimeZoneIdMapper::canonicalize_iana(std::string_view value) const {
  if (!capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_canonicalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, ICU4XError>>(std::move(result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)))));
}

inline diplomat::result<std::string, ICU4XError> ICU4XTimeZoneIdMapper::find_canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XTimeZoneIdMapper* ICU4XTimeZoneIdMapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XTimeZoneIdMapper*>(this);
}

inline capi::ICU4XTimeZoneIdMapper* ICU4XTimeZoneIdMapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XTimeZoneIdMapper*>(this);
}

inline const ICU4XTimeZoneIdMapper* ICU4XTimeZoneIdMapper::FromFFI(const capi::ICU4XTimeZoneIdMapper* ptr) {
  return reinterpret_cast<const ICU4XTimeZoneIdMapper*>(ptr);
}

inline ICU4XTimeZoneIdMapper* ICU4XTimeZoneIdMapper::FromFFI(capi::ICU4XTimeZoneIdMapper* ptr) {
  return reinterpret_cast<ICU4XTimeZoneIdMapper*>(ptr);
}

inline void ICU4XTimeZoneIdMapper::operator delete(void* ptr) {
  capi::ICU4XTimeZoneIdMapper_destroy(reinterpret_cast<capi::ICU4XTimeZoneIdMapper*>(ptr));
}


#endif // ICU4XTimeZoneIdMapper_HPP

#ifndef TimeZoneIdMapper_HPP
#define TimeZoneIdMapper_HPP

#include "TimeZoneIdMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "TimeZoneInvalidIdError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XTimeZoneIdMapper_create_result {union {diplomat::capi::TimeZoneIdMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_create_result;
    ICU4XTimeZoneIdMapper_create_result ICU4XTimeZoneIdMapper_create(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XTimeZoneIdMapper_iana_to_bcp47_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_iana_to_bcp47_result;
    ICU4XTimeZoneIdMapper_iana_to_bcp47_result ICU4XTimeZoneIdMapper_iana_to_bcp47(const diplomat::capi::TimeZoneIdMapper* self, const char* value_data, size_t value_len, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapper_normalize_iana_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_normalize_iana_result;
    ICU4XTimeZoneIdMapper_normalize_iana_result ICU4XTimeZoneIdMapper_normalize_iana(const diplomat::capi::TimeZoneIdMapper* self, const char* value_data, size_t value_len, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapper_canonicalize_iana_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_canonicalize_iana_result;
    ICU4XTimeZoneIdMapper_canonicalize_iana_result ICU4XTimeZoneIdMapper_canonicalize_iana(const diplomat::capi::TimeZoneIdMapper* self, const char* value_data, size_t value_len, diplomat::capi::DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result;
    ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(const diplomat::capi::TimeZoneIdMapper* self, const char* value_data, size_t value_len, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XTimeZoneIdMapper_destroy(TimeZoneIdMapper* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError> TimeZoneIdMapper::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XTimeZoneIdMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError>(diplomat::Ok<std::unique_ptr<TimeZoneIdMapper>>(std::unique_ptr<TimeZoneIdMapper>(TimeZoneIdMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, TimeZoneInvalidIdError> TimeZoneIdMapper::iana_to_bcp47(std::string_view value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XTimeZoneIdMapper_iana_to_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> TimeZoneIdMapper::normalize_iana(std::string_view value) const {
  if (!diplomat::capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XTimeZoneIdMapper_normalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, TimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)))));
}

inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> TimeZoneIdMapper::canonicalize_iana(std::string_view value) const {
  if (!diplomat::capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XTimeZoneIdMapper_canonicalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, TimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)))));
}

inline diplomat::result<std::string, TimeZoneInvalidIdError> TimeZoneIdMapper::find_canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline const diplomat::capi::TimeZoneIdMapper* TimeZoneIdMapper::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TimeZoneIdMapper*>(this);
}

inline diplomat::capi::TimeZoneIdMapper* TimeZoneIdMapper::AsFFI() {
  return reinterpret_cast<diplomat::capi::TimeZoneIdMapper*>(this);
}

inline const TimeZoneIdMapper* TimeZoneIdMapper::FromFFI(const diplomat::capi::TimeZoneIdMapper* ptr) {
  return reinterpret_cast<const TimeZoneIdMapper*>(ptr);
}

inline TimeZoneIdMapper* TimeZoneIdMapper::FromFFI(diplomat::capi::TimeZoneIdMapper* ptr) {
  return reinterpret_cast<TimeZoneIdMapper*>(ptr);
}

inline void TimeZoneIdMapper::operator delete(void* ptr) {
  diplomat::capi::ICU4XTimeZoneIdMapper_destroy(reinterpret_cast<diplomat::capi::TimeZoneIdMapper*>(ptr));
}


#endif // TimeZoneIdMapper_HPP

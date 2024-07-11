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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XTimeZoneIdMapper_create_result {union {TimeZoneIdMapper* ok; DataError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_create_result;
    ICU4XTimeZoneIdMapper_create_result ICU4XTimeZoneIdMapper_create(const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneIdMapper_iana_to_bcp47_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_iana_to_bcp47_result;
    ICU4XTimeZoneIdMapper_iana_to_bcp47_result ICU4XTimeZoneIdMapper_iana_to_bcp47(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapper_normalize_iana_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_normalize_iana_result;
    ICU4XTimeZoneIdMapper_normalize_iana_result ICU4XTimeZoneIdMapper_normalize_iana(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapper_canonicalize_iana_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_canonicalize_iana_result;
    ICU4XTimeZoneIdMapper_canonicalize_iana_result ICU4XTimeZoneIdMapper_canonicalize_iana(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result;
    ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(const TimeZoneIdMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);
    
    
    void ICU4XTimeZoneIdMapper_destroy(TimeZoneIdMapper* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError> TimeZoneIdMapper::create(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneIdMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError>(diplomat::Ok<std::unique_ptr<TimeZoneIdMapper>>(std::unique_ptr<TimeZoneIdMapper>(TimeZoneIdMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, TimeZoneInvalidIdError> TimeZoneIdMapper::iana_to_bcp47(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_iana_to_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> TimeZoneIdMapper::normalize_iana(std::string_view value) const {
  if (!capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_normalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, TimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)))));
}

inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> TimeZoneIdMapper::canonicalize_iana(std::string_view value) const {
  if (!capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_canonicalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, TimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)))));
}

inline diplomat::result<std::string, TimeZoneInvalidIdError> TimeZoneIdMapper::find_canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapper_find_canonical_iana_from_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline const capi::TimeZoneIdMapper* TimeZoneIdMapper::AsFFI() const {
  return reinterpret_cast<const capi::TimeZoneIdMapper*>(this);
}

inline capi::TimeZoneIdMapper* TimeZoneIdMapper::AsFFI() {
  return reinterpret_cast<capi::TimeZoneIdMapper*>(this);
}

inline const TimeZoneIdMapper* TimeZoneIdMapper::FromFFI(const capi::TimeZoneIdMapper* ptr) {
  return reinterpret_cast<const TimeZoneIdMapper*>(ptr);
}

inline TimeZoneIdMapper* TimeZoneIdMapper::FromFFI(capi::TimeZoneIdMapper* ptr) {
  return reinterpret_cast<TimeZoneIdMapper*>(ptr);
}

inline void TimeZoneIdMapper::operator delete(void* ptr) {
  capi::ICU4XTimeZoneIdMapper_destroy(reinterpret_cast<capi::TimeZoneIdMapper*>(ptr));
}


#endif // TimeZoneIdMapper_HPP

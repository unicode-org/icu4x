#ifndef TimeZoneIdMapperWithFastCanonicalization_HPP
#define TimeZoneIdMapperWithFastCanonicalization_HPP

#include "TimeZoneIdMapperWithFastCanonicalization.d.hpp"

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
    
    typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result {union {TimeZoneIdMapperWithFastCanonicalization* ok; DataError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result;
    ICU4XTimeZoneIdMapperWithFastCanonicalization_create_result ICU4XTimeZoneIdMapperWithFastCanonicalization_create(const DataProvider* provider);
    
    typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result;
    ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_result ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(const TimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);
    
    typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result;
    ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_result ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(const TimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, DiplomatWrite* write);
    
    
    void ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(TimeZoneIdMapperWithFastCanonicalization* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>, DataError> TimeZoneIdMapperWithFastCanonicalization::create(const DataProvider& provider) {
  auto result = capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>, DataError>(diplomat::Ok<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>>(std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>(TimeZoneIdMapperWithFastCanonicalization::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> TimeZoneIdMapperWithFastCanonicalization::canonicalize_iana(std::string_view value) const {
  if (!capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, TimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)))));
}

inline diplomat::result<std::string, TimeZoneInvalidIdError> TimeZoneIdMapperWithFastCanonicalization::canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline const capi::TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::AsFFI() const {
  return reinterpret_cast<const capi::TimeZoneIdMapperWithFastCanonicalization*>(this);
}

inline capi::TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::AsFFI() {
  return reinterpret_cast<capi::TimeZoneIdMapperWithFastCanonicalization*>(this);
}

inline const TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::FromFFI(const capi::TimeZoneIdMapperWithFastCanonicalization* ptr) {
  return reinterpret_cast<const TimeZoneIdMapperWithFastCanonicalization*>(ptr);
}

inline TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::FromFFI(capi::TimeZoneIdMapperWithFastCanonicalization* ptr) {
  return reinterpret_cast<TimeZoneIdMapperWithFastCanonicalization*>(ptr);
}

inline void TimeZoneIdMapperWithFastCanonicalization::operator delete(void* ptr) {
  capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(reinterpret_cast<capi::TimeZoneIdMapperWithFastCanonicalization*>(ptr));
}


#endif // TimeZoneIdMapperWithFastCanonicalization_HPP

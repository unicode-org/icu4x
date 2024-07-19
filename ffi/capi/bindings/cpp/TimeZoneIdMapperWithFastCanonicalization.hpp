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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_TimeZoneIdMapperWithFastCanonicalization_create_mv1_result {union {diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_TimeZoneIdMapperWithFastCanonicalization_create_mv1_result;
    icu4x_TimeZoneIdMapperWithFastCanonicalization_create_mv1_result icu4x_TimeZoneIdMapperWithFastCanonicalization_create_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_TimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_mv1_result { bool is_ok;} icu4x_TimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_mv1_result;
    icu4x_TimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_mv1_result icu4x_TimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_mv1(const diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_TimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_mv1_result { bool is_ok;} icu4x_TimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_mv1_result;
    icu4x_TimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_mv1_result icu4x_TimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_mv1(const diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* self, const char* value_data, size_t value_len, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_TimeZoneIdMapperWithFastCanonicalization_destroy_mv1(TimeZoneIdMapperWithFastCanonicalization* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>, DataError> TimeZoneIdMapperWithFastCanonicalization::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_TimeZoneIdMapperWithFastCanonicalization_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>, DataError>(diplomat::Ok<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>>(std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>(TimeZoneIdMapperWithFastCanonicalization::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TimeZoneIdMapperWithFastCanonicalization>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> TimeZoneIdMapperWithFastCanonicalization::canonicalize_iana(std::string_view value) const {
  if (!diplomat::capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_TimeZoneIdMapperWithFastCanonicalization_canonicalize_iana_mv1(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, TimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError {}))));
}

inline diplomat::result<std::string, TimeZoneInvalidIdError> TimeZoneIdMapperWithFastCanonicalization::canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_TimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47_mv1(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError {}));
}

inline const diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TimeZoneIdMapperWithFastCanonicalization*>(this);
}

inline diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::AsFFI() {
  return reinterpret_cast<diplomat::capi::TimeZoneIdMapperWithFastCanonicalization*>(this);
}

inline const TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::FromFFI(const diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* ptr) {
  return reinterpret_cast<const TimeZoneIdMapperWithFastCanonicalization*>(ptr);
}

inline TimeZoneIdMapperWithFastCanonicalization* TimeZoneIdMapperWithFastCanonicalization::FromFFI(diplomat::capi::TimeZoneIdMapperWithFastCanonicalization* ptr) {
  return reinterpret_cast<TimeZoneIdMapperWithFastCanonicalization*>(ptr);
}

inline void TimeZoneIdMapperWithFastCanonicalization::operator delete(void* ptr) {
  diplomat::capi::icu4x_TimeZoneIdMapperWithFastCanonicalization_destroy_mv1(reinterpret_cast<diplomat::capi::TimeZoneIdMapperWithFastCanonicalization*>(ptr));
}


#endif // TimeZoneIdMapperWithFastCanonicalization_HPP

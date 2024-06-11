#ifndef ICU4XTimeZoneIdMapperWithFastCanonicalization_HPP
#define ICU4XTimeZoneIdMapperWithFastCanonicalization_HPP

#include "ICU4XTimeZoneIdMapperWithFastCanonicalization.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XTimeZoneIdMapperWithFastCanonicalization.h"
#include "ICU4XTimeZoneInvalidIdError.hpp"


inline diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization>, ICU4XDataError> ICU4XTimeZoneIdMapperWithFastCanonicalization::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization>>(std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization>(ICU4XTimeZoneIdMapperWithFastCanonicalization::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>, diplomat::Utf8Error> ICU4XTimeZoneIdMapperWithFastCanonicalization::canonicalize_iana(std::string_view value) const {
  if (!capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_canonicalize_iana(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return diplomat::Ok<diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>>(std::move(result.is_ok ? diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>(diplomat::Err<ICU4XTimeZoneInvalidIdError>(ICU4XTimeZoneInvalidIdError::FromFFI(result.err)))));
}

inline diplomat::result<std::string, ICU4XTimeZoneInvalidIdError> ICU4XTimeZoneIdMapperWithFastCanonicalization::canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_canonical_iana_from_bcp47(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>(diplomat::Err<ICU4XTimeZoneInvalidIdError>(ICU4XTimeZoneInvalidIdError::FromFFI(result.err)));
}

inline const capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* ICU4XTimeZoneIdMapperWithFastCanonicalization::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XTimeZoneIdMapperWithFastCanonicalization*>(this);
}

inline capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* ICU4XTimeZoneIdMapperWithFastCanonicalization::AsFFI() {
  return reinterpret_cast<capi::ICU4XTimeZoneIdMapperWithFastCanonicalization*>(this);
}

inline const ICU4XTimeZoneIdMapperWithFastCanonicalization* ICU4XTimeZoneIdMapperWithFastCanonicalization::FromFFI(const capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* ptr) {
  return reinterpret_cast<const ICU4XTimeZoneIdMapperWithFastCanonicalization*>(ptr);
}

inline ICU4XTimeZoneIdMapperWithFastCanonicalization* ICU4XTimeZoneIdMapperWithFastCanonicalization::FromFFI(capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* ptr) {
  return reinterpret_cast<ICU4XTimeZoneIdMapperWithFastCanonicalization*>(ptr);
}

inline void ICU4XTimeZoneIdMapperWithFastCanonicalization::operator delete(void* ptr) {
  capi::ICU4XTimeZoneIdMapperWithFastCanonicalization_destroy(reinterpret_cast<capi::ICU4XTimeZoneIdMapperWithFastCanonicalization*>(ptr));
}


#endif // ICU4XTimeZoneIdMapperWithFastCanonicalization_HPP

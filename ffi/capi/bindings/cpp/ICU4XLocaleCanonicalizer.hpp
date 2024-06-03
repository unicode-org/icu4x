#ifndef ICU4XLocaleCanonicalizer_HPP
#define ICU4XLocaleCanonicalizer_HPP

#include "ICU4XLocaleCanonicalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XLocaleCanonicalizer.h"
#include "ICU4XTransformResult.hpp"


inline diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError> ICU4XLocaleCanonicalizer::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleCanonicalizer_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleCanonicalizer>>(std::unique_ptr<ICU4XLocaleCanonicalizer>(ICU4XLocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError> ICU4XLocaleCanonicalizer::create_extended(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleCanonicalizer_create_extended(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleCanonicalizer>>(std::unique_ptr<ICU4XLocaleCanonicalizer>(ICU4XLocaleCanonicalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline ICU4XTransformResult ICU4XLocaleCanonicalizer::canonicalize(ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleCanonicalizer_canonicalize(this->AsFFI(),
    locale.AsFFI());
  return ICU4XTransformResult::FromFFI(result);
}

inline const capi::ICU4XLocaleCanonicalizer* ICU4XLocaleCanonicalizer::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleCanonicalizer*>(this);
}

inline capi::ICU4XLocaleCanonicalizer* ICU4XLocaleCanonicalizer::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleCanonicalizer*>(this);
}

inline const ICU4XLocaleCanonicalizer* ICU4XLocaleCanonicalizer::FromFFI(const capi::ICU4XLocaleCanonicalizer* ptr) {
  return reinterpret_cast<const ICU4XLocaleCanonicalizer*>(ptr);
}

inline ICU4XLocaleCanonicalizer* ICU4XLocaleCanonicalizer::FromFFI(capi::ICU4XLocaleCanonicalizer* ptr) {
  return reinterpret_cast<ICU4XLocaleCanonicalizer*>(ptr);
}

inline void ICU4XLocaleCanonicalizer::operator delete(void* ptr) {
  capi::ICU4XLocaleCanonicalizer_destroy(reinterpret_cast<capi::ICU4XLocaleCanonicalizer*>(ptr));
}


#endif // ICU4XLocaleCanonicalizer_HPP

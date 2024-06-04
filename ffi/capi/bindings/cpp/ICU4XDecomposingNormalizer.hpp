#ifndef ICU4XDecomposingNormalizer_HPP
#define ICU4XDecomposingNormalizer_HPP

#include "ICU4XDecomposingNormalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDecomposingNormalizer.h"
#include "ICU4XError.hpp"


inline diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XError> ICU4XDecomposingNormalizer::create_nfd(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XDecomposingNormalizer_create_nfd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XDecomposingNormalizer>>(std::unique_ptr<ICU4XDecomposingNormalizer>(ICU4XDecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XError> ICU4XDecomposingNormalizer::create_nfkd(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XDecomposingNormalizer_create_nfkd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XDecomposingNormalizer>>(std::unique_ptr<ICU4XDecomposingNormalizer>(ICU4XDecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::string ICU4XDecomposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDecomposingNormalizer_normalize(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool ICU4XDecomposingNormalizer::is_normalized(std::string_view s) const {
  auto result = capi::ICU4XDecomposingNormalizer_is_normalized(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const capi::ICU4XDecomposingNormalizer* ICU4XDecomposingNormalizer::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDecomposingNormalizer*>(this);
}

inline capi::ICU4XDecomposingNormalizer* ICU4XDecomposingNormalizer::AsFFI() {
  return reinterpret_cast<capi::ICU4XDecomposingNormalizer*>(this);
}

inline const ICU4XDecomposingNormalizer* ICU4XDecomposingNormalizer::FromFFI(const capi::ICU4XDecomposingNormalizer* ptr) {
  return reinterpret_cast<const ICU4XDecomposingNormalizer*>(ptr);
}

inline ICU4XDecomposingNormalizer* ICU4XDecomposingNormalizer::FromFFI(capi::ICU4XDecomposingNormalizer* ptr) {
  return reinterpret_cast<ICU4XDecomposingNormalizer*>(ptr);
}

inline void ICU4XDecomposingNormalizer::operator delete(void* ptr) {
  capi::ICU4XDecomposingNormalizer_destroy(reinterpret_cast<capi::ICU4XDecomposingNormalizer*>(ptr));
}


#endif // ICU4XDecomposingNormalizer_HPP

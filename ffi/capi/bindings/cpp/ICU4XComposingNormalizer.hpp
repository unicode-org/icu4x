#ifndef ICU4XComposingNormalizer_HPP
#define ICU4XComposingNormalizer_HPP

#include "ICU4XComposingNormalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XComposingNormalizer.h"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


inline diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError> ICU4XComposingNormalizer::create_nfc(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XComposingNormalizer_create_nfc(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XComposingNormalizer>>(std::unique_ptr<ICU4XComposingNormalizer>(ICU4XComposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError> ICU4XComposingNormalizer::create_nfkc(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XComposingNormalizer_create_nfkc(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XComposingNormalizer>>(std::unique_ptr<ICU4XComposingNormalizer>(ICU4XComposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::string ICU4XComposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XComposingNormalizer_normalize(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool ICU4XComposingNormalizer::is_normalized(std::string_view s) const {
  auto result = capi::ICU4XComposingNormalizer_is_normalized(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const capi::ICU4XComposingNormalizer* ICU4XComposingNormalizer::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XComposingNormalizer*>(this);
}

inline capi::ICU4XComposingNormalizer* ICU4XComposingNormalizer::AsFFI() {
  return reinterpret_cast<capi::ICU4XComposingNormalizer*>(this);
}

inline const ICU4XComposingNormalizer* ICU4XComposingNormalizer::FromFFI(const capi::ICU4XComposingNormalizer* ptr) {
  return reinterpret_cast<const ICU4XComposingNormalizer*>(ptr);
}

inline ICU4XComposingNormalizer* ICU4XComposingNormalizer::FromFFI(capi::ICU4XComposingNormalizer* ptr) {
  return reinterpret_cast<ICU4XComposingNormalizer*>(ptr);
}

inline void ICU4XComposingNormalizer::operator delete(void* ptr) {
  capi::ICU4XComposingNormalizer_destroy(reinterpret_cast<capi::ICU4XComposingNormalizer*>(ptr));
}


#endif // ICU4XComposingNormalizer_HPP

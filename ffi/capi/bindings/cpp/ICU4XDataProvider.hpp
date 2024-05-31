#ifndef ICU4XDataProvider_HPP
#define ICU4XDataProvider_HPP

#include "ICU4XDataProvider.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.h"
#include "ICU4XError.hpp"
#include "ICU4XLocaleFallbacker.hpp"


inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::create_compiled() {
  auto result = capi::ICU4XDataProvider_create_compiled();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XError> ICU4XDataProvider::create_fs(std::string_view path) {
  auto result = capi::ICU4XDataProvider_create_fs(path.data(),
    path.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XDataProvider>>(std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::create_test() {
  auto result = capi::ICU4XDataProvider_create_test();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XError> ICU4XDataProvider::create_from_byte_slice(diplomat::span<const uint8_t> blob) {
  auto result = capi::ICU4XDataProvider_create_from_byte_slice(blob.data(),
    blob.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XDataProvider>>(std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::create_empty() {
  auto result = capi::ICU4XDataProvider_create_empty();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XDataProvider::fork_by_key(ICU4XDataProvider& other) {
  auto result = capi::ICU4XDataProvider_fork_by_key(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XDataProvider::fork_by_locale(ICU4XDataProvider& other) {
  auto result = capi::ICU4XDataProvider_fork_by_locale(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XDataProvider::enable_locale_fallback() {
  auto result = capi::ICU4XDataProvider_enable_locale_fallback(this->AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XError> ICU4XDataProvider::enable_locale_fallback_with(const ICU4XLocaleFallbacker& fallbacker) {
  auto result = capi::ICU4XDataProvider_enable_locale_fallback_with(this->AsFFI(),
    fallbacker.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XDataProvider* ICU4XDataProvider::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDataProvider*>(this);
}

inline capi::ICU4XDataProvider* ICU4XDataProvider::AsFFI() {
  return reinterpret_cast<capi::ICU4XDataProvider*>(this);
}

inline const ICU4XDataProvider* ICU4XDataProvider::FromFFI(const capi::ICU4XDataProvider* ptr) {
  return reinterpret_cast<const ICU4XDataProvider*>(ptr);
}

inline ICU4XDataProvider* ICU4XDataProvider::FromFFI(capi::ICU4XDataProvider* ptr) {
  return reinterpret_cast<ICU4XDataProvider*>(ptr);
}

inline void ICU4XDataProvider::operator delete(void* ptr) {
  capi::ICU4XDataProvider_destroy(reinterpret_cast<capi::ICU4XDataProvider*>(ptr));
}


#endif // ICU4XDataProvider_HPP

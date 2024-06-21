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
#include "ICU4XDataError.hpp"
#include "ICU4XLocaleFallbacker.hpp"


namespace capi {
    extern "C" {
    
    ICU4XDataProvider* ICU4XDataProvider_create_compiled();
    
    typedef struct ICU4XDataProvider_create_fs_result {union {ICU4XDataProvider* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_create_fs_result;
    ICU4XDataProvider_create_fs_result ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);
    
    typedef struct ICU4XDataProvider_create_from_byte_slice_result {union {ICU4XDataProvider* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_create_from_byte_slice_result;
    ICU4XDataProvider_create_from_byte_slice_result ICU4XDataProvider_create_from_byte_slice(const uint8_t* blob_data, size_t blob_len);
    
    ICU4XDataProvider* ICU4XDataProvider_create_empty();
    
    typedef struct ICU4XDataProvider_fork_by_key_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_fork_by_key_result;
    ICU4XDataProvider_fork_by_key_result ICU4XDataProvider_fork_by_key(ICU4XDataProvider* self, ICU4XDataProvider* other);
    
    typedef struct ICU4XDataProvider_fork_by_locale_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_fork_by_locale_result;
    ICU4XDataProvider_fork_by_locale_result ICU4XDataProvider_fork_by_locale(ICU4XDataProvider* self, ICU4XDataProvider* other);
    
    typedef struct ICU4XDataProvider_enable_locale_fallback_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_enable_locale_fallback_result;
    ICU4XDataProvider_enable_locale_fallback_result ICU4XDataProvider_enable_locale_fallback(ICU4XDataProvider* self);
    
    typedef struct ICU4XDataProvider_enable_locale_fallback_with_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_enable_locale_fallback_with_result;
    ICU4XDataProvider_enable_locale_fallback_with_result ICU4XDataProvider_enable_locale_fallback_with(ICU4XDataProvider* self, const ICU4XLocaleFallbacker* fallbacker);
    
    
    void ICU4XDataProvider_destroy(ICU4XDataProvider* self);
    
    } // extern "C"
}

inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::create_compiled() {
  auto result = capi::ICU4XDataProvider_create_compiled();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XDataError> ICU4XDataProvider::create_fs(std::string_view path) {
  auto result = capi::ICU4XDataProvider_create_fs(path.data(),
    path.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XDataProvider>>(std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XDataError> ICU4XDataProvider::create_from_byte_slice(diplomat::span<const uint8_t> blob) {
  auto result = capi::ICU4XDataProvider_create_from_byte_slice(blob.data(),
    blob.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XDataProvider>>(std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDataProvider>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XDataProvider> ICU4XDataProvider::create_empty() {
  auto result = capi::ICU4XDataProvider_create_empty();
  return std::unique_ptr<ICU4XDataProvider>(ICU4XDataProvider::FromFFI(result));
}

inline diplomat::result<std::monostate, ICU4XDataError> ICU4XDataProvider::fork_by_key(ICU4XDataProvider& other) {
  auto result = capi::ICU4XDataProvider_fork_by_key(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XDataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XDataError> ICU4XDataProvider::fork_by_locale(ICU4XDataProvider& other) {
  auto result = capi::ICU4XDataProvider_fork_by_locale(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XDataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XDataError> ICU4XDataProvider::enable_locale_fallback() {
  auto result = capi::ICU4XDataProvider_enable_locale_fallback(this->AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XDataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XDataError> ICU4XDataProvider::enable_locale_fallback_with(const ICU4XLocaleFallbacker& fallbacker) {
  auto result = capi::ICU4XDataProvider_enable_locale_fallback_with(this->AsFFI(),
    fallbacker.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XDataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
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

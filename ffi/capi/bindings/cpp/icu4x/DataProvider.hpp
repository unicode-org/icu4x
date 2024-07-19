#ifndef icu4x_DataProvider_HPP
#define icu4x_DataProvider_HPP

#include "DataProvider.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "LocaleFallbacker.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::DataProvider* icu4x_DataProvider_compiled_mv1(void);
    
    typedef struct icu4x_DataProvider_from_fs_mv1_result {union {icu4x::capi::DataProvider* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_from_fs_mv1_result;
    icu4x_DataProvider_from_fs_mv1_result icu4x_DataProvider_from_fs_mv1(const char* path_data, size_t path_len);
    
    typedef struct icu4x_DataProvider_from_byte_slice_mv1_result {union {icu4x::capi::DataProvider* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_from_byte_slice_mv1_result;
    icu4x_DataProvider_from_byte_slice_mv1_result icu4x_DataProvider_from_byte_slice_mv1(const uint8_t* blob_data, size_t blob_len);
    
    icu4x::capi::DataProvider* icu4x_DataProvider_empty_mv1(void);
    
    typedef struct icu4x_DataProvider_fork_by_key_mv1_result {union { icu4x::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_fork_by_key_mv1_result;
    icu4x_DataProvider_fork_by_key_mv1_result icu4x_DataProvider_fork_by_key_mv1(icu4x::capi::DataProvider* self, icu4x::capi::DataProvider* other);
    
    typedef struct icu4x_DataProvider_fork_by_locale_mv1_result {union { icu4x::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_fork_by_locale_mv1_result;
    icu4x_DataProvider_fork_by_locale_mv1_result icu4x_DataProvider_fork_by_locale_mv1(icu4x::capi::DataProvider* self, icu4x::capi::DataProvider* other);
    
    typedef struct icu4x_DataProvider_enable_locale_fallback_with_mv1_result {union { icu4x::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_enable_locale_fallback_with_mv1_result;
    icu4x_DataProvider_enable_locale_fallback_with_mv1_result icu4x_DataProvider_enable_locale_fallback_with_mv1(icu4x::capi::DataProvider* self, const icu4x::capi::LocaleFallbacker* fallbacker);
    
    
    void icu4x_DataProvider_destroy_mv1(DataProvider* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::DataProvider> icu4x::DataProvider::compiled() {
  auto result = icu4x::capi::icu4x_DataProvider_compiled_mv1();
  return std::unique_ptr<icu4x::DataProvider>(icu4x::DataProvider::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::DataProvider>, icu4x::DataError> icu4x::DataProvider::from_fs(std::string_view path) {
  auto result = icu4x::capi::icu4x_DataProvider_from_fs_mv1(path.data(),
    path.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DataProvider>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::DataProvider>>(std::unique_ptr<icu4x::DataProvider>(icu4x::DataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DataProvider>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::DataProvider>, icu4x::DataError> icu4x::DataProvider::from_byte_slice(diplomat::span<const uint8_t> blob) {
  auto result = icu4x::capi::icu4x_DataProvider_from_byte_slice_mv1(blob.data(),
    blob.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DataProvider>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::DataProvider>>(std::unique_ptr<icu4x::DataProvider>(icu4x::DataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DataProvider>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::DataProvider> icu4x::DataProvider::empty() {
  auto result = icu4x::capi::icu4x_DataProvider_empty_mv1();
  return std::unique_ptr<icu4x::DataProvider>(icu4x::DataProvider::FromFFI(result));
}

inline diplomat::result<std::monostate, icu4x::DataError> icu4x::DataProvider::fork_by_key(icu4x::DataProvider& other) {
  auto result = icu4x::capi::icu4x_DataProvider_fork_by_key_mv1(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::DataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::DataError> icu4x::DataProvider::fork_by_locale(icu4x::DataProvider& other) {
  auto result = icu4x::capi::icu4x_DataProvider_fork_by_locale_mv1(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::DataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, icu4x::DataError> icu4x::DataProvider::enable_locale_fallback_with(const icu4x::LocaleFallbacker& fallbacker) {
  auto result = icu4x::capi::icu4x_DataProvider_enable_locale_fallback_with_mv1(this->AsFFI(),
    fallbacker.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, icu4x::DataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::DataProvider* icu4x::DataProvider::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::DataProvider*>(this);
}

inline icu4x::capi::DataProvider* icu4x::DataProvider::AsFFI() {
  return reinterpret_cast<icu4x::capi::DataProvider*>(this);
}

inline const icu4x::DataProvider* icu4x::DataProvider::FromFFI(const icu4x::capi::DataProvider* ptr) {
  return reinterpret_cast<const icu4x::DataProvider*>(ptr);
}

inline icu4x::DataProvider* icu4x::DataProvider::FromFFI(icu4x::capi::DataProvider* ptr) {
  return reinterpret_cast<icu4x::DataProvider*>(ptr);
}

inline void icu4x::DataProvider::operator delete(void* ptr) {
  icu4x::capi::icu4x_DataProvider_destroy_mv1(reinterpret_cast<icu4x::capi::DataProvider*>(ptr));
}


#endif // icu4x_DataProvider_HPP

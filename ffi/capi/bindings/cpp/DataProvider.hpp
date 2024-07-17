#ifndef DataProvider_HPP
#define DataProvider_HPP

#include "DataProvider.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "LocaleFallbacker.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::DataProvider* icu4x_DataProvider_create_compiled_mv1();
    
    typedef struct icu4x_DataProvider_create_fs_mv1_result {union {diplomat::capi::DataProvider* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_create_fs_mv1_result;
    icu4x_DataProvider_create_fs_mv1_result icu4x_DataProvider_create_fs_mv1(const char* path_data, size_t path_len);
    
    typedef struct icu4x_DataProvider_create_from_byte_slice_mv1_result {union {diplomat::capi::DataProvider* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_create_from_byte_slice_mv1_result;
    icu4x_DataProvider_create_from_byte_slice_mv1_result icu4x_DataProvider_create_from_byte_slice_mv1(const uint8_t* blob_data, size_t blob_len);
    
    diplomat::capi::DataProvider* icu4x_DataProvider_create_empty_mv1();
    
    typedef struct icu4x_DataProvider_fork_by_key_mv1_result {union { diplomat::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_fork_by_key_mv1_result;
    icu4x_DataProvider_fork_by_key_mv1_result icu4x_DataProvider_fork_by_key_mv1(diplomat::capi::DataProvider* self, diplomat::capi::DataProvider* other);
    
    typedef struct icu4x_DataProvider_fork_by_locale_mv1_result {union { diplomat::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_fork_by_locale_mv1_result;
    icu4x_DataProvider_fork_by_locale_mv1_result icu4x_DataProvider_fork_by_locale_mv1(diplomat::capi::DataProvider* self, diplomat::capi::DataProvider* other);
    
    typedef struct icu4x_DataProvider_enable_locale_fallback_with_mv1_result {union { diplomat::capi::DataError err;}; bool is_ok;} icu4x_DataProvider_enable_locale_fallback_with_mv1_result;
    icu4x_DataProvider_enable_locale_fallback_with_mv1_result icu4x_DataProvider_enable_locale_fallback_with_mv1(diplomat::capi::DataProvider* self, const diplomat::capi::LocaleFallbacker* fallbacker);
    
    
    void icu4x_DataProvider_destroy_mv1(DataProvider* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<DataProvider> DataProvider::create_compiled() {
  auto result = diplomat::capi::icu4x_DataProvider_create_compiled_mv1();
  return std::unique_ptr<DataProvider>(DataProvider::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<DataProvider>, DataError> DataProvider::create_fs(std::string_view path) {
  auto result = diplomat::capi::icu4x_DataProvider_create_fs_mv1(path.data(),
    path.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<DataProvider>, DataError>(diplomat::Ok<std::unique_ptr<DataProvider>>(std::unique_ptr<DataProvider>(DataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DataProvider>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<DataProvider>, DataError> DataProvider::create_from_byte_slice(diplomat::span<const uint8_t> blob) {
  auto result = diplomat::capi::icu4x_DataProvider_create_from_byte_slice_mv1(blob.data(),
    blob.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<DataProvider>, DataError>(diplomat::Ok<std::unique_ptr<DataProvider>>(std::unique_ptr<DataProvider>(DataProvider::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DataProvider>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<DataProvider> DataProvider::create_empty() {
  auto result = diplomat::capi::icu4x_DataProvider_create_empty_mv1();
  return std::unique_ptr<DataProvider>(DataProvider::FromFFI(result));
}

inline diplomat::result<std::monostate, DataError> DataProvider::fork_by_key(DataProvider& other) {
  auto result = diplomat::capi::icu4x_DataProvider_fork_by_key_mv1(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, DataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, DataError> DataProvider::fork_by_locale(DataProvider& other) {
  auto result = diplomat::capi::icu4x_DataProvider_fork_by_locale_mv1(this->AsFFI(),
    other.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, DataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, DataError> DataProvider::enable_locale_fallback_with(const LocaleFallbacker& fallbacker) {
  auto result = diplomat::capi::icu4x_DataProvider_enable_locale_fallback_with_mv1(this->AsFFI(),
    fallbacker.AsFFI());
  return result.is_ok ? diplomat::result<std::monostate, DataError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::DataProvider* DataProvider::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::DataProvider*>(this);
}

inline diplomat::capi::DataProvider* DataProvider::AsFFI() {
  return reinterpret_cast<diplomat::capi::DataProvider*>(this);
}

inline const DataProvider* DataProvider::FromFFI(const diplomat::capi::DataProvider* ptr) {
  return reinterpret_cast<const DataProvider*>(ptr);
}

inline DataProvider* DataProvider::FromFFI(diplomat::capi::DataProvider* ptr) {
  return reinterpret_cast<DataProvider*>(ptr);
}

inline void DataProvider::operator delete(void* ptr) {
  diplomat::capi::icu4x_DataProvider_destroy_mv1(reinterpret_cast<diplomat::capi::DataProvider*>(ptr));
}


#endif // DataProvider_HPP

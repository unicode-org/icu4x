#ifndef icu4x_DecomposingNormalizer_HPP
#define icu4x_DecomposingNormalizer_HPP

#include "DecomposingNormalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_DecomposingNormalizer_create_nfd_mv1_result {union {icu4x::capi::DecomposingNormalizer* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_DecomposingNormalizer_create_nfd_mv1_result;
    icu4x_DecomposingNormalizer_create_nfd_mv1_result icu4x_DecomposingNormalizer_create_nfd_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_DecomposingNormalizer_create_nfkd_mv1_result {union {icu4x::capi::DecomposingNormalizer* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_DecomposingNormalizer_create_nfkd_mv1_result;
    icu4x_DecomposingNormalizer_create_nfkd_mv1_result icu4x_DecomposingNormalizer_create_nfkd_mv1(const icu4x::capi::DataProvider* provider);
    
    void icu4x_DecomposingNormalizer_normalize_mv1(const icu4x::capi::DecomposingNormalizer* self, const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    bool icu4x_DecomposingNormalizer_is_normalized_mv1(const icu4x::capi::DecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    bool icu4x_DecomposingNormalizer_is_normalized_utf16_mv1(const icu4x::capi::DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    size_t icu4x_DecomposingNormalizer_is_normalized_up_to_mv1(const icu4x::capi::DecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    size_t icu4x_DecomposingNormalizer_is_normalized_utf16_up_to_mv1(const icu4x::capi::DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    
    void icu4x_DecomposingNormalizer_destroy_mv1(DecomposingNormalizer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::DecomposingNormalizer>, icu4x::DataError> icu4x::DecomposingNormalizer::create_nfd(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_DecomposingNormalizer_create_nfd_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DecomposingNormalizer>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::DecomposingNormalizer>>(std::unique_ptr<icu4x::DecomposingNormalizer>(icu4x::DecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DecomposingNormalizer>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::DecomposingNormalizer>, icu4x::DataError> icu4x::DecomposingNormalizer::create_nfkd(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_DecomposingNormalizer_create_nfkd_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::DecomposingNormalizer>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::DecomposingNormalizer>>(std::unique_ptr<icu4x::DecomposingNormalizer>(icu4x::DecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::DecomposingNormalizer>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::string icu4x::DecomposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_DecomposingNormalizer_normalize_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool icu4x::DecomposingNormalizer::is_normalized(std::string_view s) const {
  auto result = icu4x::capi::icu4x_DecomposingNormalizer_is_normalized_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool icu4x::DecomposingNormalizer::is_normalized_utf16(std::u16string_view s) const {
  auto result = icu4x::capi::icu4x_DecomposingNormalizer_is_normalized_utf16_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t icu4x::DecomposingNormalizer::is_normalized_up_to(std::string_view s) const {
  auto result = icu4x::capi::icu4x_DecomposingNormalizer_is_normalized_up_to_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t icu4x::DecomposingNormalizer::is_normalized_utf16_up_to(std::u16string_view s) const {
  auto result = icu4x::capi::icu4x_DecomposingNormalizer_is_normalized_utf16_up_to_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const icu4x::capi::DecomposingNormalizer* icu4x::DecomposingNormalizer::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::DecomposingNormalizer*>(this);
}

inline icu4x::capi::DecomposingNormalizer* icu4x::DecomposingNormalizer::AsFFI() {
  return reinterpret_cast<icu4x::capi::DecomposingNormalizer*>(this);
}

inline const icu4x::DecomposingNormalizer* icu4x::DecomposingNormalizer::FromFFI(const icu4x::capi::DecomposingNormalizer* ptr) {
  return reinterpret_cast<const icu4x::DecomposingNormalizer*>(ptr);
}

inline icu4x::DecomposingNormalizer* icu4x::DecomposingNormalizer::FromFFI(icu4x::capi::DecomposingNormalizer* ptr) {
  return reinterpret_cast<icu4x::DecomposingNormalizer*>(ptr);
}

inline void icu4x::DecomposingNormalizer::operator delete(void* ptr) {
  icu4x::capi::icu4x_DecomposingNormalizer_destroy_mv1(reinterpret_cast<icu4x::capi::DecomposingNormalizer*>(ptr));
}


#endif // icu4x_DecomposingNormalizer_HPP

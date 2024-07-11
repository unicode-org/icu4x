#ifndef DecomposingNormalizer_HPP
#define DecomposingNormalizer_HPP

#include "DecomposingNormalizer.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XDecomposingNormalizer_create_nfd_result {union {DecomposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfd_result;
    ICU4XDecomposingNormalizer_create_nfd_result ICU4XDecomposingNormalizer_create_nfd(const DataProvider* provider);
    
    typedef struct ICU4XDecomposingNormalizer_create_nfkd_result {union {DecomposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfkd_result;
    ICU4XDecomposingNormalizer_create_nfkd_result ICU4XDecomposingNormalizer_create_nfkd(const DataProvider* provider);
    
    void ICU4XDecomposingNormalizer_normalize(const DecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    bool ICU4XDecomposingNormalizer_is_normalized(const DecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    bool ICU4XDecomposingNormalizer_is_normalized_utf16(const DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    size_t ICU4XDecomposingNormalizer_is_normalized_up_to(const DecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    size_t ICU4XDecomposingNormalizer_is_normalized_utf16_up_to(const DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    
    void ICU4XDecomposingNormalizer_destroy(DecomposingNormalizer* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError> DecomposingNormalizer::create_nfd(const DataProvider& provider) {
  auto result = capi::ICU4XDecomposingNormalizer_create_nfd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<DecomposingNormalizer>>(std::unique_ptr<DecomposingNormalizer>(DecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError> DecomposingNormalizer::create_nfkd(const DataProvider& provider) {
  auto result = capi::ICU4XDecomposingNormalizer_create_nfkd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<DecomposingNormalizer>>(std::unique_ptr<DecomposingNormalizer>(DecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string DecomposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XDecomposingNormalizer_normalize(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool DecomposingNormalizer::is_normalized(std::string_view s) const {
  auto result = capi::ICU4XDecomposingNormalizer_is_normalized(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool DecomposingNormalizer::is_normalized_utf16(std::u16string_view s) const {
  auto result = capi::ICU4XDecomposingNormalizer_is_normalized_utf16(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t DecomposingNormalizer::is_normalized_up_to(std::string_view s) const {
  auto result = capi::ICU4XDecomposingNormalizer_is_normalized_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t DecomposingNormalizer::is_normalized_utf16_up_to(std::u16string_view s) const {
  auto result = capi::ICU4XDecomposingNormalizer_is_normalized_utf16_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const capi::DecomposingNormalizer* DecomposingNormalizer::AsFFI() const {
  return reinterpret_cast<const capi::DecomposingNormalizer*>(this);
}

inline capi::DecomposingNormalizer* DecomposingNormalizer::AsFFI() {
  return reinterpret_cast<capi::DecomposingNormalizer*>(this);
}

inline const DecomposingNormalizer* DecomposingNormalizer::FromFFI(const capi::DecomposingNormalizer* ptr) {
  return reinterpret_cast<const DecomposingNormalizer*>(ptr);
}

inline DecomposingNormalizer* DecomposingNormalizer::FromFFI(capi::DecomposingNormalizer* ptr) {
  return reinterpret_cast<DecomposingNormalizer*>(ptr);
}

inline void DecomposingNormalizer::operator delete(void* ptr) {
  capi::ICU4XDecomposingNormalizer_destroy(reinterpret_cast<capi::DecomposingNormalizer*>(ptr));
}


#endif // DecomposingNormalizer_HPP

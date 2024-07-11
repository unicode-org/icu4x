#ifndef ComposingNormalizer_HPP
#define ComposingNormalizer_HPP

#include "ComposingNormalizer.d.hpp"

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
    
    typedef struct ICU4XComposingNormalizer_create_nfc_result {union {ComposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfc_result;
    ICU4XComposingNormalizer_create_nfc_result ICU4XComposingNormalizer_create_nfc(const DataProvider* provider);
    
    typedef struct ICU4XComposingNormalizer_create_nfkc_result {union {ComposingNormalizer* ok; DataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfkc_result;
    ICU4XComposingNormalizer_create_nfkc_result ICU4XComposingNormalizer_create_nfkc(const DataProvider* provider);
    
    void ICU4XComposingNormalizer_normalize(const ComposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    bool ICU4XComposingNormalizer_is_normalized(const ComposingNormalizer* self, const char* s_data, size_t s_len);
    
    bool ICU4XComposingNormalizer_is_normalized_utf16(const ComposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    size_t ICU4XComposingNormalizer_is_normalized_up_to(const ComposingNormalizer* self, const char* s_data, size_t s_len);
    
    size_t ICU4XComposingNormalizer_is_normalized_utf16_up_to(const ComposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    
    void ICU4XComposingNormalizer_destroy(ComposingNormalizer* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> ComposingNormalizer::create_nfc(const DataProvider& provider) {
  auto result = capi::ICU4XComposingNormalizer_create_nfc(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<ComposingNormalizer>>(std::unique_ptr<ComposingNormalizer>(ComposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> ComposingNormalizer::create_nfkc(const DataProvider& provider) {
  auto result = capi::ICU4XComposingNormalizer_create_nfkc(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<ComposingNormalizer>>(std::unique_ptr<ComposingNormalizer>(ComposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string ComposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XComposingNormalizer_normalize(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool ComposingNormalizer::is_normalized(std::string_view s) const {
  auto result = capi::ICU4XComposingNormalizer_is_normalized(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool ComposingNormalizer::is_normalized_utf16(std::u16string_view s) const {
  auto result = capi::ICU4XComposingNormalizer_is_normalized_utf16(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t ComposingNormalizer::is_normalized_up_to(std::string_view s) const {
  auto result = capi::ICU4XComposingNormalizer_is_normalized_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t ComposingNormalizer::is_normalized_utf16_up_to(std::u16string_view s) const {
  auto result = capi::ICU4XComposingNormalizer_is_normalized_utf16_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const capi::ComposingNormalizer* ComposingNormalizer::AsFFI() const {
  return reinterpret_cast<const capi::ComposingNormalizer*>(this);
}

inline capi::ComposingNormalizer* ComposingNormalizer::AsFFI() {
  return reinterpret_cast<capi::ComposingNormalizer*>(this);
}

inline const ComposingNormalizer* ComposingNormalizer::FromFFI(const capi::ComposingNormalizer* ptr) {
  return reinterpret_cast<const ComposingNormalizer*>(ptr);
}

inline ComposingNormalizer* ComposingNormalizer::FromFFI(capi::ComposingNormalizer* ptr) {
  return reinterpret_cast<ComposingNormalizer*>(ptr);
}

inline void ComposingNormalizer::operator delete(void* ptr) {
  capi::ICU4XComposingNormalizer_destroy(reinterpret_cast<capi::ComposingNormalizer*>(ptr));
}


#endif // ComposingNormalizer_HPP

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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XComposingNormalizer_create_nfc_result {union {diplomat::capi::ComposingNormalizer* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfc_result;
    ICU4XComposingNormalizer_create_nfc_result ICU4XComposingNormalizer_create_nfc(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XComposingNormalizer_create_nfkc_result {union {diplomat::capi::ComposingNormalizer* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XComposingNormalizer_create_nfkc_result;
    ICU4XComposingNormalizer_create_nfkc_result ICU4XComposingNormalizer_create_nfkc(const diplomat::capi::DataProvider* provider);
    
    void ICU4XComposingNormalizer_normalize(const diplomat::capi::ComposingNormalizer* self, const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    bool ICU4XComposingNormalizer_is_normalized(const diplomat::capi::ComposingNormalizer* self, const char* s_data, size_t s_len);
    
    bool ICU4XComposingNormalizer_is_normalized_utf16(const diplomat::capi::ComposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    size_t ICU4XComposingNormalizer_is_normalized_up_to(const diplomat::capi::ComposingNormalizer* self, const char* s_data, size_t s_len);
    
    size_t ICU4XComposingNormalizer_is_normalized_utf16_up_to(const diplomat::capi::ComposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    
    void ICU4XComposingNormalizer_destroy(ComposingNormalizer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> ComposingNormalizer::create_nfc(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XComposingNormalizer_create_nfc(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<ComposingNormalizer>>(std::unique_ptr<ComposingNormalizer>(ComposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> ComposingNormalizer::create_nfkc(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XComposingNormalizer_create_nfkc(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<ComposingNormalizer>>(std::unique_ptr<ComposingNormalizer>(ComposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string ComposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XComposingNormalizer_normalize(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool ComposingNormalizer::is_normalized(std::string_view s) const {
  auto result = diplomat::capi::ICU4XComposingNormalizer_is_normalized(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool ComposingNormalizer::is_normalized_utf16(std::u16string_view s) const {
  auto result = diplomat::capi::ICU4XComposingNormalizer_is_normalized_utf16(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t ComposingNormalizer::is_normalized_up_to(std::string_view s) const {
  auto result = diplomat::capi::ICU4XComposingNormalizer_is_normalized_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t ComposingNormalizer::is_normalized_utf16_up_to(std::u16string_view s) const {
  auto result = diplomat::capi::ICU4XComposingNormalizer_is_normalized_utf16_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const diplomat::capi::ComposingNormalizer* ComposingNormalizer::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ComposingNormalizer*>(this);
}

inline diplomat::capi::ComposingNormalizer* ComposingNormalizer::AsFFI() {
  return reinterpret_cast<diplomat::capi::ComposingNormalizer*>(this);
}

inline const ComposingNormalizer* ComposingNormalizer::FromFFI(const diplomat::capi::ComposingNormalizer* ptr) {
  return reinterpret_cast<const ComposingNormalizer*>(ptr);
}

inline ComposingNormalizer* ComposingNormalizer::FromFFI(diplomat::capi::ComposingNormalizer* ptr) {
  return reinterpret_cast<ComposingNormalizer*>(ptr);
}

inline void ComposingNormalizer::operator delete(void* ptr) {
  diplomat::capi::ICU4XComposingNormalizer_destroy(reinterpret_cast<diplomat::capi::ComposingNormalizer*>(ptr));
}


#endif // ComposingNormalizer_HPP

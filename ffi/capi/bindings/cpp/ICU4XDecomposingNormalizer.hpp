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
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XDecomposingNormalizer_create_nfd_result {union {ICU4XDecomposingNormalizer* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfd_result;
    ICU4XDecomposingNormalizer_create_nfd_result ICU4XDecomposingNormalizer_create_nfd(const ICU4XDataProvider* provider);
    
    typedef struct ICU4XDecomposingNormalizer_create_nfkd_result {union {ICU4XDecomposingNormalizer* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfkd_result;
    ICU4XDecomposingNormalizer_create_nfkd_result ICU4XDecomposingNormalizer_create_nfkd(const ICU4XDataProvider* provider);
    
    void ICU4XDecomposingNormalizer_normalize(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    bool ICU4XDecomposingNormalizer_is_normalized(const ICU4XDecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    
    void ICU4XDecomposingNormalizer_destroy(ICU4XDecomposingNormalizer* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError> ICU4XDecomposingNormalizer::create_nfd(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XDecomposingNormalizer_create_nfd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XDecomposingNormalizer>>(std::unique_ptr<ICU4XDecomposingNormalizer>(ICU4XDecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError> ICU4XDecomposingNormalizer::create_nfkd(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XDecomposingNormalizer_create_nfkd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XDecomposingNormalizer>>(std::unique_ptr<ICU4XDecomposingNormalizer>(ICU4XDecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
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

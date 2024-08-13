#ifndef icu4x_Collator_HPP
#define icu4x_Collator_HPP

#include "Collator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "CollatorOptionsV1.hpp"
#include "CollatorResolvedOptionsV1.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_Collator_create_v1_mv1_result {union {icu4x::capi::Collator* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_Collator_create_v1_mv1_result;
    icu4x_Collator_create_v1_mv1_result icu4x_Collator_create_v1_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::CollatorOptionsV1 options);
    
    int8_t icu4x_Collator_compare_utf8_mv1(const icu4x::capi::Collator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);
    
    int8_t icu4x_Collator_compare_utf16_mv1(const icu4x::capi::Collator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);
    
    icu4x::capi::CollatorResolvedOptionsV1 icu4x_Collator_resolved_options_v1_mv1(const icu4x::capi::Collator* self);
    
    
    void icu4x_Collator_destroy_mv1(Collator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::Collator>, icu4x::DataError> icu4x::Collator::create_v1(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::CollatorOptionsV1 options) {
  auto result = icu4x::capi::icu4x_Collator_create_v1_mv1(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::Collator>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::Collator>>(std::unique_ptr<icu4x::Collator>(icu4x::Collator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::Collator>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline int8_t icu4x::Collator::compare(std::string_view left, std::string_view right) const {
  auto result = icu4x::capi::icu4x_Collator_compare_utf8_mv1(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline int8_t icu4x::Collator::compare16(std::u16string_view left, std::u16string_view right) const {
  auto result = icu4x::capi::icu4x_Collator_compare_utf16_mv1(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline icu4x::CollatorResolvedOptionsV1 icu4x::Collator::resolved_options_v1() const {
  auto result = icu4x::capi::icu4x_Collator_resolved_options_v1_mv1(this->AsFFI());
  return icu4x::CollatorResolvedOptionsV1::FromFFI(result);
}

inline const icu4x::capi::Collator* icu4x::Collator::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::Collator*>(this);
}

inline icu4x::capi::Collator* icu4x::Collator::AsFFI() {
  return reinterpret_cast<icu4x::capi::Collator*>(this);
}

inline const icu4x::Collator* icu4x::Collator::FromFFI(const icu4x::capi::Collator* ptr) {
  return reinterpret_cast<const icu4x::Collator*>(ptr);
}

inline icu4x::Collator* icu4x::Collator::FromFFI(icu4x::capi::Collator* ptr) {
  return reinterpret_cast<icu4x::Collator*>(ptr);
}

inline void icu4x::Collator::operator delete(void* ptr) {
  icu4x::capi::icu4x_Collator_destroy_mv1(reinterpret_cast<icu4x::capi::Collator*>(ptr));
}


#endif // icu4x_Collator_HPP

#ifndef Collator_HPP
#define Collator_HPP

#include "Collator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CollatorOptionsV1.hpp"
#include "CollatorResolvedOptionsV1.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XCollator_create_v1_result {union {diplomat::capi::Collator* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCollator_create_v1_result;
    ICU4XCollator_create_v1_result ICU4XCollator_create_v1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::CollatorOptionsV1 options);
    
    int8_t ICU4XCollator_compare_utf16_(const diplomat::capi::Collator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);
    
    int8_t ICU4XCollator_compare_(const diplomat::capi::Collator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);
    
    diplomat::capi::CollatorResolvedOptionsV1 ICU4XCollator_resolved_options(const diplomat::capi::Collator* self);
    
    
    void ICU4XCollator_destroy(Collator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Collator>, DataError> Collator::create_v1(const DataProvider& provider, const Locale& locale, CollatorOptionsV1 options) {
  auto result = diplomat::capi::ICU4XCollator_create_v1(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Collator>, DataError>(diplomat::Ok<std::unique_ptr<Collator>>(std::unique_ptr<Collator>(Collator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Collator>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline int8_t Collator::compare16(std::u16string_view left, std::u16string_view right) const {
  auto result = diplomat::capi::ICU4XCollator_compare_utf16_(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline int8_t Collator::compare(std::string_view left, std::string_view right) const {
  auto result = diplomat::capi::ICU4XCollator_compare_(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline CollatorResolvedOptionsV1 Collator::resolved_options() const {
  auto result = diplomat::capi::ICU4XCollator_resolved_options(this->AsFFI());
  return CollatorResolvedOptionsV1::FromFFI(result);
}

inline const diplomat::capi::Collator* Collator::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Collator*>(this);
}

inline diplomat::capi::Collator* Collator::AsFFI() {
  return reinterpret_cast<diplomat::capi::Collator*>(this);
}

inline const Collator* Collator::FromFFI(const diplomat::capi::Collator* ptr) {
  return reinterpret_cast<const Collator*>(ptr);
}

inline Collator* Collator::FromFFI(diplomat::capi::Collator* ptr) {
  return reinterpret_cast<Collator*>(ptr);
}

inline void Collator::operator delete(void* ptr) {
  diplomat::capi::ICU4XCollator_destroy(reinterpret_cast<diplomat::capi::Collator*>(ptr));
}


#endif // Collator_HPP

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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCollator_create_v1_result {union {Collator* ok; DataError err;}; bool is_ok;} ICU4XCollator_create_v1_result;
    ICU4XCollator_create_v1_result ICU4XCollator_create_v1(const DataProvider* provider, const Locale* locale, CollatorOptionsV1 options);
    
    int8_t ICU4XCollator_compare_utf16_(const Collator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);
    
    int8_t ICU4XCollator_compare_(const Collator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);
    
    CollatorResolvedOptionsV1 ICU4XCollator_resolved_options(const Collator* self);
    
    
    void ICU4XCollator_destroy(Collator* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<Collator>, DataError> Collator::create_v1(const DataProvider& provider, const Locale& locale, CollatorOptionsV1 options) {
  auto result = capi::ICU4XCollator_create_v1(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Collator>, DataError>(diplomat::Ok<std::unique_ptr<Collator>>(std::unique_ptr<Collator>(Collator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Collator>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline int8_t Collator::compare16(std::u16string_view left, std::u16string_view right) const {
  auto result = capi::ICU4XCollator_compare_utf16_(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline int8_t Collator::compare(std::string_view left, std::string_view right) const {
  auto result = capi::ICU4XCollator_compare_(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline CollatorResolvedOptionsV1 Collator::resolved_options() const {
  auto result = capi::ICU4XCollator_resolved_options(this->AsFFI());
  return CollatorResolvedOptionsV1::FromFFI(result);
}

inline const capi::Collator* Collator::AsFFI() const {
  return reinterpret_cast<const capi::Collator*>(this);
}

inline capi::Collator* Collator::AsFFI() {
  return reinterpret_cast<capi::Collator*>(this);
}

inline const Collator* Collator::FromFFI(const capi::Collator* ptr) {
  return reinterpret_cast<const Collator*>(ptr);
}

inline Collator* Collator::FromFFI(capi::Collator* ptr) {
  return reinterpret_cast<Collator*>(ptr);
}

inline void Collator::operator delete(void* ptr) {
  capi::ICU4XCollator_destroy(reinterpret_cast<capi::Collator*>(ptr));
}


#endif // Collator_HPP

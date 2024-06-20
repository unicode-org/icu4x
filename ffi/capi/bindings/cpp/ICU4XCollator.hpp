#ifndef ICU4XCollator_HPP
#define ICU4XCollator_HPP

#include "ICU4XCollator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorOptionsV1.hpp"
#include "ICU4XCollatorResolvedOptionsV1.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XCollator_create_v1_result {union {ICU4XCollator* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XCollator_create_v1_result ICU4XCollator_create_v1(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XCollatorOptionsV1 options);
    
    int8_t ICU4XCollator_compare_utf16_(const ICU4XCollator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);
    
    int8_t ICU4XCollator_compare_(const ICU4XCollator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);
    
    ICU4XCollatorResolvedOptionsV1 ICU4XCollator_resolved_options(const ICU4XCollator* self);
    
    
    void ICU4XCollator_destroy(ICU4XCollator* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XCollator>, ICU4XDataError> ICU4XCollator::create_v1(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XCollatorOptionsV1 options) {
  auto result = capi::ICU4XCollator_create_v1(provider.AsFFI(),
    locale.AsFFI(),
    options.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCollator>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCollator>>(std::unique_ptr<ICU4XCollator>(ICU4XCollator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCollator>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline int8_t ICU4XCollator::compare16(std::u16string_view left, std::u16string_view right) const {
  auto result = capi::ICU4XCollator_compare_utf16_(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline int8_t ICU4XCollator::compare(std::string_view left, std::string_view right) const {
  auto result = capi::ICU4XCollator_compare_(this->AsFFI(),
    left.data(),
    left.size(),
    right.data(),
    right.size());
  return result;
}

inline ICU4XCollatorResolvedOptionsV1 ICU4XCollator::resolved_options() const {
  auto result = capi::ICU4XCollator_resolved_options(this->AsFFI());
  return ICU4XCollatorResolvedOptionsV1::FromFFI(result);
}

inline const capi::ICU4XCollator* ICU4XCollator::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCollator*>(this);
}

inline capi::ICU4XCollator* ICU4XCollator::AsFFI() {
  return reinterpret_cast<capi::ICU4XCollator*>(this);
}

inline const ICU4XCollator* ICU4XCollator::FromFFI(const capi::ICU4XCollator* ptr) {
  return reinterpret_cast<const ICU4XCollator*>(ptr);
}

inline ICU4XCollator* ICU4XCollator::FromFFI(capi::ICU4XCollator* ptr) {
  return reinterpret_cast<ICU4XCollator*>(ptr);
}

inline void ICU4XCollator::operator delete(void* ptr) {
  capi::ICU4XCollator_destroy(reinterpret_cast<capi::ICU4XCollator*>(ptr));
}


#endif // ICU4XCollator_HPP

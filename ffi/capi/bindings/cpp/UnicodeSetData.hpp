#ifndef UnicodeSetData_HPP
#define UnicodeSetData_HPP

#include "UnicodeSetData.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    bool ICU4XUnicodeSetData_contains(const diplomat::capi::UnicodeSetData* self, const char* s_data, size_t s_len);
    
    bool ICU4XUnicodeSetData_contains_char(const diplomat::capi::UnicodeSetData* self, char32_t cp);
    
    bool ICU4XUnicodeSetData_contains32(const diplomat::capi::UnicodeSetData* self, uint32_t cp);
    
    typedef struct ICU4XUnicodeSetData_load_basic_emoji_result {union {diplomat::capi::UnicodeSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_basic_emoji_result;
    ICU4XUnicodeSetData_load_basic_emoji_result ICU4XUnicodeSetData_load_basic_emoji(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XUnicodeSetData_load_exemplars_main_result {union {diplomat::capi::UnicodeSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_main_result;
    ICU4XUnicodeSetData_load_exemplars_main_result ICU4XUnicodeSetData_load_exemplars_main(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct ICU4XUnicodeSetData_load_exemplars_auxiliary_result {union {diplomat::capi::UnicodeSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_auxiliary_result;
    ICU4XUnicodeSetData_load_exemplars_auxiliary_result ICU4XUnicodeSetData_load_exemplars_auxiliary(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct ICU4XUnicodeSetData_load_exemplars_punctuation_result {union {diplomat::capi::UnicodeSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_punctuation_result;
    ICU4XUnicodeSetData_load_exemplars_punctuation_result ICU4XUnicodeSetData_load_exemplars_punctuation(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct ICU4XUnicodeSetData_load_exemplars_numbers_result {union {diplomat::capi::UnicodeSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_numbers_result;
    ICU4XUnicodeSetData_load_exemplars_numbers_result ICU4XUnicodeSetData_load_exemplars_numbers(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct ICU4XUnicodeSetData_load_exemplars_index_result {union {diplomat::capi::UnicodeSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnicodeSetData_load_exemplars_index_result;
    ICU4XUnicodeSetData_load_exemplars_index_result ICU4XUnicodeSetData_load_exemplars_index(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    
    void ICU4XUnicodeSetData_destroy(UnicodeSetData* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline bool UnicodeSetData::contains(std::string_view s) const {
  auto result = diplomat::capi::ICU4XUnicodeSetData_contains(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool UnicodeSetData::contains_char(char32_t cp) const {
  auto result = diplomat::capi::ICU4XUnicodeSetData_contains_char(this->AsFFI(),
    cp);
  return result;
}

inline bool UnicodeSetData::contains32(uint32_t cp) const {
  auto result = diplomat::capi::ICU4XUnicodeSetData_contains32(this->AsFFI(),
    cp);
  return result;
}

inline diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> UnicodeSetData::load_basic_emoji(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XUnicodeSetData_load_basic_emoji(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Ok<std::unique_ptr<UnicodeSetData>>(std::unique_ptr<UnicodeSetData>(UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> UnicodeSetData::load_exemplars_main(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::ICU4XUnicodeSetData_load_exemplars_main(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Ok<std::unique_ptr<UnicodeSetData>>(std::unique_ptr<UnicodeSetData>(UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> UnicodeSetData::load_exemplars_auxiliary(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::ICU4XUnicodeSetData_load_exemplars_auxiliary(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Ok<std::unique_ptr<UnicodeSetData>>(std::unique_ptr<UnicodeSetData>(UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> UnicodeSetData::load_exemplars_punctuation(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::ICU4XUnicodeSetData_load_exemplars_punctuation(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Ok<std::unique_ptr<UnicodeSetData>>(std::unique_ptr<UnicodeSetData>(UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> UnicodeSetData::load_exemplars_numbers(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::ICU4XUnicodeSetData_load_exemplars_numbers(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Ok<std::unique_ptr<UnicodeSetData>>(std::unique_ptr<UnicodeSetData>(UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> UnicodeSetData::load_exemplars_index(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::ICU4XUnicodeSetData_load_exemplars_index(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Ok<std::unique_ptr<UnicodeSetData>>(std::unique_ptr<UnicodeSetData>(UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnicodeSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::UnicodeSetData* UnicodeSetData::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::UnicodeSetData*>(this);
}

inline diplomat::capi::UnicodeSetData* UnicodeSetData::AsFFI() {
  return reinterpret_cast<diplomat::capi::UnicodeSetData*>(this);
}

inline const UnicodeSetData* UnicodeSetData::FromFFI(const diplomat::capi::UnicodeSetData* ptr) {
  return reinterpret_cast<const UnicodeSetData*>(ptr);
}

inline UnicodeSetData* UnicodeSetData::FromFFI(diplomat::capi::UnicodeSetData* ptr) {
  return reinterpret_cast<UnicodeSetData*>(ptr);
}

inline void UnicodeSetData::operator delete(void* ptr) {
  diplomat::capi::ICU4XUnicodeSetData_destroy(reinterpret_cast<diplomat::capi::UnicodeSetData*>(ptr));
}


#endif // UnicodeSetData_HPP

#ifndef icu4x_UnicodeSetData_HPP
#define icu4x_UnicodeSetData_HPP

#include "UnicodeSetData.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    bool icu4x_UnicodeSetData_contains_mv1(const icu4x::capi::UnicodeSetData* self, diplomat::capi::DiplomatStringView s);
    
    bool icu4x_UnicodeSetData_contains_char_mv1(const icu4x::capi::UnicodeSetData* self, char32_t cp);
    
    typedef struct icu4x_UnicodeSetData_load_basic_emoji_mv1_result {union {icu4x::capi::UnicodeSetData* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_basic_emoji_mv1_result;
    icu4x_UnicodeSetData_load_basic_emoji_mv1_result icu4x_UnicodeSetData_load_basic_emoji_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_UnicodeSetData_load_exemplars_main_mv1_result {union {icu4x::capi::UnicodeSetData* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_main_mv1_result;
    icu4x_UnicodeSetData_load_exemplars_main_mv1_result icu4x_UnicodeSetData_load_exemplars_main_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    typedef struct icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1_result {union {icu4x::capi::UnicodeSetData* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1_result;
    icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1_result icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    typedef struct icu4x_UnicodeSetData_load_exemplars_punctuation_mv1_result {union {icu4x::capi::UnicodeSetData* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_punctuation_mv1_result;
    icu4x_UnicodeSetData_load_exemplars_punctuation_mv1_result icu4x_UnicodeSetData_load_exemplars_punctuation_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    typedef struct icu4x_UnicodeSetData_load_exemplars_numbers_mv1_result {union {icu4x::capi::UnicodeSetData* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_numbers_mv1_result;
    icu4x_UnicodeSetData_load_exemplars_numbers_mv1_result icu4x_UnicodeSetData_load_exemplars_numbers_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    typedef struct icu4x_UnicodeSetData_load_exemplars_index_mv1_result {union {icu4x::capi::UnicodeSetData* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_exemplars_index_mv1_result;
    icu4x_UnicodeSetData_load_exemplars_index_mv1_result icu4x_UnicodeSetData_load_exemplars_index_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale);
    
    
    void icu4x_UnicodeSetData_destroy_mv1(UnicodeSetData* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline bool icu4x::UnicodeSetData::contains(std::string_view s) const {
  auto result = icu4x::capi::icu4x_UnicodeSetData_contains_mv1(this->AsFFI(),
    {s.data(), s.size()});
  return result;
}

inline bool icu4x::UnicodeSetData::contains_char(char32_t cp) const {
  auto result = icu4x::capi::icu4x_UnicodeSetData_contains_char_mv1(this->AsFFI(),
    cp);
  return result;
}

inline diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> icu4x::UnicodeSetData::load_basic_emoji(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_UnicodeSetData_load_basic_emoji_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::UnicodeSetData>>(std::unique_ptr<icu4x::UnicodeSetData>(icu4x::UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> icu4x::UnicodeSetData::load_exemplars_main(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_UnicodeSetData_load_exemplars_main_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::UnicodeSetData>>(std::unique_ptr<icu4x::UnicodeSetData>(icu4x::UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> icu4x::UnicodeSetData::load_exemplars_auxiliary(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_UnicodeSetData_load_exemplars_auxiliary_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::UnicodeSetData>>(std::unique_ptr<icu4x::UnicodeSetData>(icu4x::UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> icu4x::UnicodeSetData::load_exemplars_punctuation(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_UnicodeSetData_load_exemplars_punctuation_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::UnicodeSetData>>(std::unique_ptr<icu4x::UnicodeSetData>(icu4x::UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> icu4x::UnicodeSetData::load_exemplars_numbers(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_UnicodeSetData_load_exemplars_numbers_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::UnicodeSetData>>(std::unique_ptr<icu4x::UnicodeSetData>(icu4x::UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> icu4x::UnicodeSetData::load_exemplars_index(const icu4x::DataProvider& provider, const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_UnicodeSetData_load_exemplars_index_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::UnicodeSetData>>(std::unique_ptr<icu4x::UnicodeSetData>(icu4x::UnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::UnicodeSetData* icu4x::UnicodeSetData::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::UnicodeSetData*>(this);
}

inline icu4x::capi::UnicodeSetData* icu4x::UnicodeSetData::AsFFI() {
  return reinterpret_cast<icu4x::capi::UnicodeSetData*>(this);
}

inline const icu4x::UnicodeSetData* icu4x::UnicodeSetData::FromFFI(const icu4x::capi::UnicodeSetData* ptr) {
  return reinterpret_cast<const icu4x::UnicodeSetData*>(ptr);
}

inline icu4x::UnicodeSetData* icu4x::UnicodeSetData::FromFFI(icu4x::capi::UnicodeSetData* ptr) {
  return reinterpret_cast<icu4x::UnicodeSetData*>(ptr);
}

inline void icu4x::UnicodeSetData::operator delete(void* ptr) {
  icu4x::capi::icu4x_UnicodeSetData_destroy_mv1(reinterpret_cast<icu4x::capi::UnicodeSetData*>(ptr));
}


#endif // icu4x_UnicodeSetData_HPP

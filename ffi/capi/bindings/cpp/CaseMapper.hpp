#ifndef CaseMapper_HPP
#define CaseMapper_HPP

#include "CaseMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CodePointSetBuilder.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "TitlecaseOptionsV1.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CaseMapper_create_mv1_result {union {diplomat::capi::CaseMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CaseMapper_create_mv1_result;
    icu4x_CaseMapper_create_mv1_result icu4x_CaseMapper_create_mv1(const diplomat::capi::DataProvider* provider);
    
    void icu4x_CaseMapper_lowercase_mv1(const diplomat::capi::CaseMapper* self, const char* s_data, size_t s_len, const diplomat::capi::Locale* locale, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CaseMapper_uppercase_mv1(const diplomat::capi::CaseMapper* self, const char* s_data, size_t s_len, const diplomat::capi::Locale* locale, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CaseMapper_titlecase_segment_with_only_case_data_v1_mv1(const diplomat::capi::CaseMapper* self, const char* s_data, size_t s_len, const diplomat::capi::Locale* locale, diplomat::capi::TitlecaseOptionsV1 options, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CaseMapper_fold_mv1(const diplomat::capi::CaseMapper* self, const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CaseMapper_fold_turkic_mv1(const diplomat::capi::CaseMapper* self, const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CaseMapper_add_case_closure_to_mv1(const diplomat::capi::CaseMapper* self, char32_t c, diplomat::capi::CodePointSetBuilder* builder);
    
    char32_t icu4x_CaseMapper_simple_lowercase_mv1(const diplomat::capi::CaseMapper* self, char32_t ch);
    
    char32_t icu4x_CaseMapper_simple_uppercase_mv1(const diplomat::capi::CaseMapper* self, char32_t ch);
    
    char32_t icu4x_CaseMapper_simple_titlecase_mv1(const diplomat::capi::CaseMapper* self, char32_t ch);
    
    char32_t icu4x_CaseMapper_simple_fold_mv1(const diplomat::capi::CaseMapper* self, char32_t ch);
    
    char32_t icu4x_CaseMapper_simple_fold_turkic_mv1(const diplomat::capi::CaseMapper* self, char32_t ch);
    
    
    void icu4x_CaseMapper_destroy_mv1(CaseMapper* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CaseMapper>, DataError> CaseMapper::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CaseMapper_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CaseMapper>, DataError>(diplomat::Ok<std::unique_ptr<CaseMapper>>(std::unique_ptr<CaseMapper>(CaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CaseMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::lowercase(std::string_view s, const Locale& locale) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_CaseMapper_lowercase_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::uppercase(std::string_view s, const Locale& locale) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_CaseMapper_uppercase_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::titlecase_segment_with_only_case_data_v1(std::string_view s, const Locale& locale, TitlecaseOptionsV1 options) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_CaseMapper_titlecase_segment_with_only_case_data_v1_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    options.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::fold(std::string_view s) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_CaseMapper_fold_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::fold_turkic(std::string_view s) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_CaseMapper_fold_turkic_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline void CaseMapper::add_case_closure_to(char32_t c, CodePointSetBuilder& builder) const {
  diplomat::capi::icu4x_CaseMapper_add_case_closure_to_mv1(this->AsFFI(),
    c,
    builder.AsFFI());
}

inline char32_t CaseMapper::simple_lowercase(char32_t ch) const {
  auto result = diplomat::capi::icu4x_CaseMapper_simple_lowercase_mv1(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_uppercase(char32_t ch) const {
  auto result = diplomat::capi::icu4x_CaseMapper_simple_uppercase_mv1(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_titlecase(char32_t ch) const {
  auto result = diplomat::capi::icu4x_CaseMapper_simple_titlecase_mv1(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_fold(char32_t ch) const {
  auto result = diplomat::capi::icu4x_CaseMapper_simple_fold_mv1(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_fold_turkic(char32_t ch) const {
  auto result = diplomat::capi::icu4x_CaseMapper_simple_fold_turkic_mv1(this->AsFFI(),
    ch);
  return result;
}

inline const diplomat::capi::CaseMapper* CaseMapper::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CaseMapper*>(this);
}

inline diplomat::capi::CaseMapper* CaseMapper::AsFFI() {
  return reinterpret_cast<diplomat::capi::CaseMapper*>(this);
}

inline const CaseMapper* CaseMapper::FromFFI(const diplomat::capi::CaseMapper* ptr) {
  return reinterpret_cast<const CaseMapper*>(ptr);
}

inline CaseMapper* CaseMapper::FromFFI(diplomat::capi::CaseMapper* ptr) {
  return reinterpret_cast<CaseMapper*>(ptr);
}

inline void CaseMapper::operator delete(void* ptr) {
  diplomat::capi::icu4x_CaseMapper_destroy_mv1(reinterpret_cast<diplomat::capi::CaseMapper*>(ptr));
}


#endif // CaseMapper_HPP

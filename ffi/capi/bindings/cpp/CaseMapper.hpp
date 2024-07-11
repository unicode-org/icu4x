#ifndef CaseMapper_HPP
#define CaseMapper_HPP

#include "CaseMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointSetBuilder.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "TitlecaseOptionsV1.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCaseMapper_create_result {union {CaseMapper* ok; DataError err;}; bool is_ok;} ICU4XCaseMapper_create_result;
    ICU4XCaseMapper_create_result ICU4XCaseMapper_create(const DataProvider* provider);
    
    void ICU4XCaseMapper_lowercase(const CaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, DiplomatWrite* write);
    
    void ICU4XCaseMapper_uppercase(const CaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, DiplomatWrite* write);
    
    void ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1(const CaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, TitlecaseOptionsV1 options, DiplomatWrite* write);
    
    void ICU4XCaseMapper_fold(const CaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XCaseMapper_fold_turkic(const CaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XCaseMapper_add_case_closure_to(const CaseMapper* self, char32_t c, CodePointSetBuilder* builder);
    
    char32_t ICU4XCaseMapper_simple_lowercase(const CaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_uppercase(const CaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_titlecase(const CaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_fold(const CaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_fold_turkic(const CaseMapper* self, char32_t ch);
    
    
    void ICU4XCaseMapper_destroy(CaseMapper* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<CaseMapper>, DataError> CaseMapper::create(const DataProvider& provider) {
  auto result = capi::ICU4XCaseMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CaseMapper>, DataError>(diplomat::Ok<std::unique_ptr<CaseMapper>>(std::unique_ptr<CaseMapper>(CaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CaseMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::lowercase(std::string_view s, const Locale& locale) const {
  if (!capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XCaseMapper_lowercase(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::uppercase(std::string_view s, const Locale& locale) const {
  if (!capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XCaseMapper_uppercase(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::titlecase_segment_with_only_case_data_v1(std::string_view s, const Locale& locale, TitlecaseOptionsV1 options) const {
  if (!capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    options.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::fold(std::string_view s) const {
  if (!capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XCaseMapper_fold(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline diplomat::result<std::string, diplomat::Utf8Error> CaseMapper::fold_turkic(std::string_view s) const {
  if (!capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XCaseMapper_fold_turkic(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline void CaseMapper::add_case_closure_to(char32_t c, CodePointSetBuilder& builder) const {
  capi::ICU4XCaseMapper_add_case_closure_to(this->AsFFI(),
    c,
    builder.AsFFI());
}

inline char32_t CaseMapper::simple_lowercase(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_lowercase(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_uppercase(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_uppercase(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_titlecase(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_titlecase(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_fold(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_fold(this->AsFFI(),
    ch);
  return result;
}

inline char32_t CaseMapper::simple_fold_turkic(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_fold_turkic(this->AsFFI(),
    ch);
  return result;
}

inline const capi::CaseMapper* CaseMapper::AsFFI() const {
  return reinterpret_cast<const capi::CaseMapper*>(this);
}

inline capi::CaseMapper* CaseMapper::AsFFI() {
  return reinterpret_cast<capi::CaseMapper*>(this);
}

inline const CaseMapper* CaseMapper::FromFFI(const capi::CaseMapper* ptr) {
  return reinterpret_cast<const CaseMapper*>(ptr);
}

inline CaseMapper* CaseMapper::FromFFI(capi::CaseMapper* ptr) {
  return reinterpret_cast<CaseMapper*>(ptr);
}

inline void CaseMapper::operator delete(void* ptr) {
  capi::ICU4XCaseMapper_destroy(reinterpret_cast<capi::CaseMapper*>(ptr));
}


#endif // CaseMapper_HPP

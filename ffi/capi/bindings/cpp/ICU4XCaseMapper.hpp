#ifndef ICU4XCaseMapper_HPP
#define ICU4XCaseMapper_HPP

#include "ICU4XCaseMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointSetBuilder.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XTitlecaseOptionsV1.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCaseMapper_create_result {union {ICU4XCaseMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCaseMapper_create_result;
    ICU4XCaseMapper_create_result ICU4XCaseMapper_create(const ICU4XDataProvider* provider);
    
    void ICU4XCaseMapper_lowercase(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, DiplomatWrite* write);
    
    void ICU4XCaseMapper_uppercase(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, DiplomatWrite* write);
    
    void ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, ICU4XTitlecaseOptionsV1 options, DiplomatWrite* write);
    
    void ICU4XCaseMapper_fold(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XCaseMapper_fold_turkic(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XCaseMapper_add_case_closure_to(const ICU4XCaseMapper* self, char32_t c, ICU4XCodePointSetBuilder* builder);
    
    char32_t ICU4XCaseMapper_simple_lowercase(const ICU4XCaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_uppercase(const ICU4XCaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_titlecase(const ICU4XCaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_fold(const ICU4XCaseMapper* self, char32_t ch);
    
    char32_t ICU4XCaseMapper_simple_fold_turkic(const ICU4XCaseMapper* self, char32_t ch);
    
    
    void ICU4XCaseMapper_destroy(ICU4XCaseMapper* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XCaseMapper>, ICU4XDataError> ICU4XCaseMapper::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCaseMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCaseMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCaseMapper>>(std::unique_ptr<ICU4XCaseMapper>(ICU4XCaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCaseMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> ICU4XCaseMapper::lowercase(std::string_view s, const ICU4XLocale& locale) const {
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

inline diplomat::result<std::string, diplomat::Utf8Error> ICU4XCaseMapper::uppercase(std::string_view s, const ICU4XLocale& locale) const {
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

inline diplomat::result<std::string, diplomat::Utf8Error> ICU4XCaseMapper::titlecase_segment_with_only_case_data_v1(std::string_view s, const ICU4XLocale& locale, ICU4XTitlecaseOptionsV1 options) const {
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

inline diplomat::result<std::string, diplomat::Utf8Error> ICU4XCaseMapper::fold(std::string_view s) const {
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

inline diplomat::result<std::string, diplomat::Utf8Error> ICU4XCaseMapper::fold_turkic(std::string_view s) const {
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

inline void ICU4XCaseMapper::add_case_closure_to(char32_t c, ICU4XCodePointSetBuilder& builder) const {
  capi::ICU4XCaseMapper_add_case_closure_to(this->AsFFI(),
    c,
    builder.AsFFI());
}

inline char32_t ICU4XCaseMapper::simple_lowercase(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_lowercase(this->AsFFI(),
    ch);
  return result;
}

inline char32_t ICU4XCaseMapper::simple_uppercase(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_uppercase(this->AsFFI(),
    ch);
  return result;
}

inline char32_t ICU4XCaseMapper::simple_titlecase(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_titlecase(this->AsFFI(),
    ch);
  return result;
}

inline char32_t ICU4XCaseMapper::simple_fold(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_fold(this->AsFFI(),
    ch);
  return result;
}

inline char32_t ICU4XCaseMapper::simple_fold_turkic(char32_t ch) const {
  auto result = capi::ICU4XCaseMapper_simple_fold_turkic(this->AsFFI(),
    ch);
  return result;
}

inline const capi::ICU4XCaseMapper* ICU4XCaseMapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCaseMapper*>(this);
}

inline capi::ICU4XCaseMapper* ICU4XCaseMapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XCaseMapper*>(this);
}

inline const ICU4XCaseMapper* ICU4XCaseMapper::FromFFI(const capi::ICU4XCaseMapper* ptr) {
  return reinterpret_cast<const ICU4XCaseMapper*>(ptr);
}

inline ICU4XCaseMapper* ICU4XCaseMapper::FromFFI(capi::ICU4XCaseMapper* ptr) {
  return reinterpret_cast<ICU4XCaseMapper*>(ptr);
}

inline void ICU4XCaseMapper::operator delete(void* ptr) {
  capi::ICU4XCaseMapper_destroy(reinterpret_cast<capi::ICU4XCaseMapper*>(ptr));
}


#endif // ICU4XCaseMapper_HPP

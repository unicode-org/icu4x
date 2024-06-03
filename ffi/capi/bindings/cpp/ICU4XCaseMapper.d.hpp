#ifndef ICU4XCaseMapper_D_HPP
#define ICU4XCaseMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCaseMapper.d.h"
#include "ICU4XError.d.hpp"
#include "ICU4XTitlecaseOptionsV1.d.hpp"

class ICU4XCodePointSetBuilder;
class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XTitlecaseOptionsV1;
class ICU4XError;


class ICU4XCaseMapper {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCaseMapper>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline diplomat::result<std::string, diplomat::Utf8Error> lowercase(std::string_view s, const ICU4XLocale& locale) const;

  inline diplomat::result<std::string, diplomat::Utf8Error> uppercase(std::string_view s, const ICU4XLocale& locale) const;

  inline diplomat::result<std::string, diplomat::Utf8Error> titlecase_segment_with_only_case_data_v1(std::string_view s, const ICU4XLocale& locale, ICU4XTitlecaseOptionsV1 options) const;

  inline diplomat::result<std::string, diplomat::Utf8Error> fold(std::string_view s) const;

  inline diplomat::result<std::string, diplomat::Utf8Error> fold_turkic(std::string_view s) const;

  inline void add_case_closure_to(char32_t c, ICU4XCodePointSetBuilder& builder) const;

  inline char32_t simple_lowercase(char32_t ch) const;

  inline char32_t simple_uppercase(char32_t ch) const;

  inline char32_t simple_titlecase(char32_t ch) const;

  inline char32_t simple_fold(char32_t ch) const;

  inline char32_t simple_fold_turkic(char32_t ch) const;

  inline const capi::ICU4XCaseMapper* AsFFI() const;
  inline capi::ICU4XCaseMapper* AsFFI();
  inline static const ICU4XCaseMapper* FromFFI(const capi::ICU4XCaseMapper* ptr);
  inline static ICU4XCaseMapper* FromFFI(capi::ICU4XCaseMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCaseMapper() = delete;
  ICU4XCaseMapper(const ICU4XCaseMapper&) = delete;
  ICU4XCaseMapper(ICU4XCaseMapper&&) noexcept = delete;
  ICU4XCaseMapper operator=(const ICU4XCaseMapper&) = delete;
  ICU4XCaseMapper operator=(ICU4XCaseMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCaseMapper_D_HPP

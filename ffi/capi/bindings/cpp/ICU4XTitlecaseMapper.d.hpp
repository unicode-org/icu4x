#ifndef ICU4XTitlecaseMapper_D_HPP
#define ICU4XTitlecaseMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XTitlecaseMapper.d.h"
#include "ICU4XTitlecaseOptionsV1.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XTitlecaseOptionsV1;
class ICU4XDataError;


class ICU4XTitlecaseMapper {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XTitlecaseMapper>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline diplomat::result<std::string, diplomat::Utf8Error> titlecase_segment_v1(std::string_view s, const ICU4XLocale& locale, ICU4XTitlecaseOptionsV1 options) const;

  inline const capi::ICU4XTitlecaseMapper* AsFFI() const;
  inline capi::ICU4XTitlecaseMapper* AsFFI();
  inline static const ICU4XTitlecaseMapper* FromFFI(const capi::ICU4XTitlecaseMapper* ptr);
  inline static ICU4XTitlecaseMapper* FromFFI(capi::ICU4XTitlecaseMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XTitlecaseMapper() = delete;
  ICU4XTitlecaseMapper(const ICU4XTitlecaseMapper&) = delete;
  ICU4XTitlecaseMapper(ICU4XTitlecaseMapper&&) noexcept = delete;
  ICU4XTitlecaseMapper operator=(const ICU4XTitlecaseMapper&) = delete;
  ICU4XTitlecaseMapper operator=(ICU4XTitlecaseMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XTitlecaseMapper_D_HPP

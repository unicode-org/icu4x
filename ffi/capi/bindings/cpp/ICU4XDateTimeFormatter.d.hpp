#ifndef ICU4XDateTimeFormatter_D_HPP
#define ICU4XDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateLength.d.hpp"
#include "ICU4XDateTimeFormatter.d.h"
#include "ICU4XError.d.hpp"
#include "ICU4XTimeLength.d.hpp"

class ICU4XDataProvider;
class ICU4XDateTime;
class ICU4XIsoDateTime;
class ICU4XLocale;
class ICU4XDateLength;
class ICU4XError;
class ICU4XTimeLength;


class ICU4XDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XDateTimeFormatter>, ICU4XError> create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

  inline diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XDateTime& value) const;

  inline diplomat::result<std::string, ICU4XError> format_iso_datetime(const ICU4XIsoDateTime& value) const;

  inline const capi::ICU4XDateTimeFormatter* AsFFI() const;
  inline capi::ICU4XDateTimeFormatter* AsFFI();
  inline static const ICU4XDateTimeFormatter* FromFFI(const capi::ICU4XDateTimeFormatter* ptr);
  inline static ICU4XDateTimeFormatter* FromFFI(capi::ICU4XDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDateTimeFormatter() = delete;
  ICU4XDateTimeFormatter(const ICU4XDateTimeFormatter&) = delete;
  ICU4XDateTimeFormatter(ICU4XDateTimeFormatter&&) noexcept = delete;
  ICU4XDateTimeFormatter operator=(const ICU4XDateTimeFormatter&) = delete;
  ICU4XDateTimeFormatter operator=(ICU4XDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDateTimeFormatter_D_HPP

#ifndef ICU4XDateFormatter_D_HPP
#define ICU4XDateFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateLength.d.hpp"
#include "ICU4XError.d.hpp"

class ICU4XDataProvider;
class ICU4XDate;
class ICU4XDateTime;
class ICU4XIsoDate;
class ICU4XIsoDateTime;
class ICU4XLocale;
class ICU4XDateLength;
class ICU4XError;


namespace capi {
    typedef struct ICU4XDateFormatter ICU4XDateFormatter;
}

class ICU4XDateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XDateFormatter>, ICU4XError> create_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length);

  inline diplomat::result<std::string, ICU4XError> format_date(const ICU4XDate& value) const;

  inline diplomat::result<std::string, ICU4XError> format_iso_date(const ICU4XIsoDate& value) const;

  inline diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XDateTime& value) const;

  inline diplomat::result<std::string, ICU4XError> format_iso_datetime(const ICU4XIsoDateTime& value) const;

  inline const capi::ICU4XDateFormatter* AsFFI() const;
  inline capi::ICU4XDateFormatter* AsFFI();
  inline static const ICU4XDateFormatter* FromFFI(const capi::ICU4XDateFormatter* ptr);
  inline static ICU4XDateFormatter* FromFFI(capi::ICU4XDateFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDateFormatter() = delete;
  ICU4XDateFormatter(const ICU4XDateFormatter&) = delete;
  ICU4XDateFormatter(ICU4XDateFormatter&&) noexcept = delete;
  ICU4XDateFormatter operator=(const ICU4XDateFormatter&) = delete;
  ICU4XDateFormatter operator=(ICU4XDateFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDateFormatter_D_HPP

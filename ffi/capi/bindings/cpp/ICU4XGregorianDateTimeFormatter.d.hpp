#ifndef ICU4XGregorianDateTimeFormatter_D_HPP
#define ICU4XGregorianDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateLength.d.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XGregorianDateTimeFormatter.d.h"
#include "ICU4XTimeLength.d.hpp"

class ICU4XDataProvider;
class ICU4XIsoDateTime;
class ICU4XLocale;
class ICU4XDateLength;
class ICU4XError;
class ICU4XTimeLength;


class ICU4XGregorianDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XGregorianDateTimeFormatter>, ICU4XError> create_with_lengths(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

  inline std::string format_iso_datetime(const ICU4XIsoDateTime& value) const;

  inline const capi::ICU4XGregorianDateTimeFormatter* AsFFI() const;
  inline capi::ICU4XGregorianDateTimeFormatter* AsFFI();
  inline static const ICU4XGregorianDateTimeFormatter* FromFFI(const capi::ICU4XGregorianDateTimeFormatter* ptr);
  inline static ICU4XGregorianDateTimeFormatter* FromFFI(capi::ICU4XGregorianDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGregorianDateTimeFormatter() = delete;
  ICU4XGregorianDateTimeFormatter(const ICU4XGregorianDateTimeFormatter&) = delete;
  ICU4XGregorianDateTimeFormatter(ICU4XGregorianDateTimeFormatter&&) noexcept = delete;
  ICU4XGregorianDateTimeFormatter operator=(const ICU4XGregorianDateTimeFormatter&) = delete;
  ICU4XGregorianDateTimeFormatter operator=(ICU4XGregorianDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGregorianDateTimeFormatter_D_HPP

#ifndef ICU4XGregorianDateFormatter_D_HPP
#define ICU4XGregorianDateFormatter_D_HPP

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
class ICU4XIsoDate;
class ICU4XIsoDateTime;
class ICU4XLocale;
class ICU4XDateLength;
class ICU4XError;


namespace capi {
    typedef struct ICU4XGregorianDateFormatter ICU4XGregorianDateFormatter;
}

class ICU4XGregorianDateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XGregorianDateFormatter>, ICU4XError> create_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength length);

  inline std::string format_iso_date(const ICU4XIsoDate& value) const;

  inline std::string format_iso_datetime(const ICU4XIsoDateTime& value) const;

  inline const capi::ICU4XGregorianDateFormatter* AsFFI() const;
  inline capi::ICU4XGregorianDateFormatter* AsFFI();
  inline static const ICU4XGregorianDateFormatter* FromFFI(const capi::ICU4XGregorianDateFormatter* ptr);
  inline static ICU4XGregorianDateFormatter* FromFFI(capi::ICU4XGregorianDateFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGregorianDateFormatter() = delete;
  ICU4XGregorianDateFormatter(const ICU4XGregorianDateFormatter&) = delete;
  ICU4XGregorianDateFormatter(ICU4XGregorianDateFormatter&&) noexcept = delete;
  ICU4XGregorianDateFormatter operator=(const ICU4XGregorianDateFormatter&) = delete;
  ICU4XGregorianDateFormatter operator=(ICU4XGregorianDateFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGregorianDateFormatter_D_HPP

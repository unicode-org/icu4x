#ifndef DateFormatter_D_HPP
#define DateFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DateLength.d.hpp"
#include "Error.d.hpp"

class DataProvider;
class Date;
class DateTime;
class IsoDate;
class IsoDateTime;
class Locale;
class DateLength;
class Error;


namespace capi {
    typedef struct DateFormatter DateFormatter;
}

class DateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<DateFormatter>, Error> create_with_length(const DataProvider& provider, const Locale& locale, DateLength date_length);

  inline diplomat::result<std::string, Error> format_date(const Date& value) const;

  inline diplomat::result<std::string, Error> format_iso_date(const IsoDate& value) const;

  inline diplomat::result<std::string, Error> format_datetime(const DateTime& value) const;

  inline diplomat::result<std::string, Error> format_iso_datetime(const IsoDateTime& value) const;

  inline const capi::DateFormatter* AsFFI() const;
  inline capi::DateFormatter* AsFFI();
  inline static const DateFormatter* FromFFI(const capi::DateFormatter* ptr);
  inline static DateFormatter* FromFFI(capi::DateFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  DateFormatter() = delete;
  DateFormatter(const DateFormatter&) = delete;
  DateFormatter(DateFormatter&&) noexcept = delete;
  DateFormatter operator=(const DateFormatter&) = delete;
  DateFormatter operator=(DateFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DateFormatter_D_HPP

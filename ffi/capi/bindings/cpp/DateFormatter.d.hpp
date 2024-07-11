#ifndef DateFormatter_D_HPP
#define DateFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct Date Date; }
class Date;
namespace capi {typedef struct DateTime DateTime; }
class DateTime;
namespace capi {typedef struct IsoDate IsoDate; }
class IsoDate;
namespace capi {typedef struct IsoDateTime IsoDateTime; }
class IsoDateTime;
namespace capi {typedef struct Locale Locale; }
class Locale;
class DateLength;
class Error;


namespace diplomat {
namespace capi {
    typedef struct DateFormatter DateFormatter;
} // namespace capi
} // namespace

class DateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<DateFormatter>, Error> create_with_length(const DataProvider& provider, const Locale& locale, DateLength date_length);

  inline diplomat::result<std::string, Error> format_date(const Date& value) const;

  inline diplomat::result<std::string, Error> format_iso_date(const IsoDate& value) const;

  inline diplomat::result<std::string, Error> format_datetime(const DateTime& value) const;

  inline diplomat::result<std::string, Error> format_iso_datetime(const IsoDateTime& value) const;

  inline const diplomat::capi::DateFormatter* AsFFI() const;
  inline diplomat::capi::DateFormatter* AsFFI();
  inline static const DateFormatter* FromFFI(const diplomat::capi::DateFormatter* ptr);
  inline static DateFormatter* FromFFI(diplomat::capi::DateFormatter* ptr);
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

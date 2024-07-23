#ifndef DateTimeFormatter_D_HPP
#define DateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct DateTime; }
class DateTime;
namespace diplomat::capi { struct IsoDateTime; }
class IsoDateTime;
namespace diplomat::capi { struct Locale; }
class Locale;
class DateLength;
class Error;
class TimeLength;


namespace diplomat {
namespace capi {
    struct DateTimeFormatter;
} // namespace capi
} // namespace

class DateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<DateTimeFormatter>, Error> create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length);

  inline diplomat::result<std::string, Error> format_datetime(const DateTime& value) const;

  inline diplomat::result<std::string, Error> format_iso_datetime(const IsoDateTime& value) const;

  inline const diplomat::capi::DateTimeFormatter* AsFFI() const;
  inline diplomat::capi::DateTimeFormatter* AsFFI();
  inline static const DateTimeFormatter* FromFFI(const diplomat::capi::DateTimeFormatter* ptr);
  inline static DateTimeFormatter* FromFFI(diplomat::capi::DateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  DateTimeFormatter() = delete;
  DateTimeFormatter(const DateTimeFormatter&) = delete;
  DateTimeFormatter(DateTimeFormatter&&) noexcept = delete;
  DateTimeFormatter operator=(const DateTimeFormatter&) = delete;
  DateTimeFormatter operator=(DateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DateTimeFormatter_D_HPP

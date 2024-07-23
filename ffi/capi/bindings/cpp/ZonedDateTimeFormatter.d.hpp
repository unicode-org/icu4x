#ifndef ZonedDateTimeFormatter_D_HPP
#define ZonedDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CustomTimeZone; }
class CustomTimeZone;
namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct DateTime; }
class DateTime;
namespace diplomat::capi { struct IsoDateTime; }
class IsoDateTime;
namespace diplomat::capi { struct Locale; }
class Locale;
struct IsoTimeZoneOptions;
class DateLength;
class Error;
class TimeLength;


namespace diplomat {
namespace capi {
    struct ZonedDateTimeFormatter;
} // namespace capi
} // namespace

class ZonedDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error> create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length);

  inline static diplomat::result<std::unique_ptr<ZonedDateTimeFormatter>, Error> create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options);

  inline diplomat::result<std::string, Error> format_datetime_with_custom_time_zone(const DateTime& datetime, const CustomTimeZone& time_zone) const;

  inline diplomat::result<std::string, Error> format_iso_datetime_with_custom_time_zone(const IsoDateTime& datetime, const CustomTimeZone& time_zone) const;

  inline const diplomat::capi::ZonedDateTimeFormatter* AsFFI() const;
  inline diplomat::capi::ZonedDateTimeFormatter* AsFFI();
  inline static const ZonedDateTimeFormatter* FromFFI(const diplomat::capi::ZonedDateTimeFormatter* ptr);
  inline static ZonedDateTimeFormatter* FromFFI(diplomat::capi::ZonedDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ZonedDateTimeFormatter() = delete;
  ZonedDateTimeFormatter(const ZonedDateTimeFormatter&) = delete;
  ZonedDateTimeFormatter(ZonedDateTimeFormatter&&) noexcept = delete;
  ZonedDateTimeFormatter operator=(const ZonedDateTimeFormatter&) = delete;
  ZonedDateTimeFormatter operator=(ZonedDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ZonedDateTimeFormatter_D_HPP

#ifndef icu4x_ZonedDateTimeFormatter_D_HPP
#define icu4x_ZonedDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct CustomTimeZone; }
class CustomTimeZone;
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct DateTime; }
class DateTime;
namespace capi { struct IsoDateTime; }
class IsoDateTime;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct ZonedDateTimeFormatter; }
class ZonedDateTimeFormatter;
struct IsoTimeZoneOptions;
class DateLength;
class Error;
class TimeLength;
}


namespace icu4x {
namespace capi {
    struct ZonedDateTimeFormatter;
} // namespace capi
} // namespace

namespace icu4x {
class ZonedDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatter>, icu4x::Error> create_with_lengths(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DateLength date_length, icu4x::TimeLength time_length);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatter>, icu4x::Error> create_with_lengths_and_iso_8601_time_zone_fallback(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DateLength date_length, icu4x::TimeLength time_length, icu4x::IsoTimeZoneOptions zone_options);

  inline diplomat::result<std::string, icu4x::Error> format_datetime_with_custom_time_zone(const icu4x::DateTime& datetime, const icu4x::CustomTimeZone& time_zone) const;

  inline diplomat::result<std::string, icu4x::Error> format_iso_datetime_with_custom_time_zone(const icu4x::IsoDateTime& datetime, const icu4x::CustomTimeZone& time_zone) const;

  inline const icu4x::capi::ZonedDateTimeFormatter* AsFFI() const;
  inline icu4x::capi::ZonedDateTimeFormatter* AsFFI();
  inline static const icu4x::ZonedDateTimeFormatter* FromFFI(const icu4x::capi::ZonedDateTimeFormatter* ptr);
  inline static icu4x::ZonedDateTimeFormatter* FromFFI(icu4x::capi::ZonedDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ZonedDateTimeFormatter() = delete;
  ZonedDateTimeFormatter(const icu4x::ZonedDateTimeFormatter&) = delete;
  ZonedDateTimeFormatter(icu4x::ZonedDateTimeFormatter&&) noexcept = delete;
  ZonedDateTimeFormatter operator=(const icu4x::ZonedDateTimeFormatter&) = delete;
  ZonedDateTimeFormatter operator=(icu4x::ZonedDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ZonedDateTimeFormatter_D_HPP

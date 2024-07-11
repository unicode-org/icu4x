#ifndef GregorianZonedDateTimeFormatter_D_HPP
#define GregorianZonedDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct CustomTimeZone CustomTimeZone; }
class CustomTimeZone;
namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct IsoDateTime IsoDateTime; }
class IsoDateTime;
namespace capi {typedef struct Locale Locale; }
class Locale;
struct IsoTimeZoneOptions;
class DateLength;
class Error;
class TimeLength;


namespace diplomat {
namespace capi {
    typedef struct GregorianZonedDateTimeFormatter GregorianZonedDateTimeFormatter;
} // namespace capi
} // namespace

class GregorianZonedDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error> create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length);

  inline static diplomat::result<std::unique_ptr<GregorianZonedDateTimeFormatter>, Error> create_with_lengths_and_iso_8601_time_zone_fallback(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length, IsoTimeZoneOptions zone_options);

  inline std::string format_iso_datetime_with_custom_time_zone(const IsoDateTime& datetime, const CustomTimeZone& time_zone) const;

  inline const diplomat::capi::GregorianZonedDateTimeFormatter* AsFFI() const;
  inline diplomat::capi::GregorianZonedDateTimeFormatter* AsFFI();
  inline static const GregorianZonedDateTimeFormatter* FromFFI(const diplomat::capi::GregorianZonedDateTimeFormatter* ptr);
  inline static GregorianZonedDateTimeFormatter* FromFFI(diplomat::capi::GregorianZonedDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  GregorianZonedDateTimeFormatter() = delete;
  GregorianZonedDateTimeFormatter(const GregorianZonedDateTimeFormatter&) = delete;
  GregorianZonedDateTimeFormatter(GregorianZonedDateTimeFormatter&&) noexcept = delete;
  GregorianZonedDateTimeFormatter operator=(const GregorianZonedDateTimeFormatter&) = delete;
  GregorianZonedDateTimeFormatter operator=(GregorianZonedDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GregorianZonedDateTimeFormatter_D_HPP

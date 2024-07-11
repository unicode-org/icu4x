#ifndef TimeZoneFormatter_D_HPP
#define TimeZoneFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Error.d.hpp"
#include "IsoTimeZoneOptions.d.hpp"

class CustomTimeZone;
class DataProvider;
class Locale;
struct IsoTimeZoneOptions;
class Error;


namespace capi {
    typedef struct TimeZoneFormatter TimeZoneFormatter;
}

class TimeZoneFormatter {
public:

  inline static diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error> create_with_localized_gmt_fallback(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<TimeZoneFormatter>, Error> create_with_iso_8601_fallback(const DataProvider& provider, const Locale& locale, IsoTimeZoneOptions options);

  inline diplomat::result<std::monostate, Error> load_generic_non_location_long(const DataProvider& provider);

  inline diplomat::result<std::monostate, Error> load_generic_non_location_short(const DataProvider& provider);

  inline diplomat::result<std::monostate, Error> load_specific_non_location_long(const DataProvider& provider);

  inline diplomat::result<std::monostate, Error> load_specific_non_location_short(const DataProvider& provider);

  inline diplomat::result<std::monostate, Error> load_generic_location_format(const DataProvider& provider);

  inline diplomat::result<std::monostate, Error> include_localized_gmt_format();

  inline diplomat::result<std::monostate, Error> load_iso_8601_format(IsoTimeZoneOptions options);

  inline std::string format_custom_time_zone(const CustomTimeZone& value) const;

  inline diplomat::result<std::string, Error> format_custom_time_zone_no_fallback(const CustomTimeZone& value) const;

  inline const capi::TimeZoneFormatter* AsFFI() const;
  inline capi::TimeZoneFormatter* AsFFI();
  inline static const TimeZoneFormatter* FromFFI(const capi::TimeZoneFormatter* ptr);
  inline static TimeZoneFormatter* FromFFI(capi::TimeZoneFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  TimeZoneFormatter() = delete;
  TimeZoneFormatter(const TimeZoneFormatter&) = delete;
  TimeZoneFormatter(TimeZoneFormatter&&) noexcept = delete;
  TimeZoneFormatter operator=(const TimeZoneFormatter&) = delete;
  TimeZoneFormatter operator=(TimeZoneFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TimeZoneFormatter_D_HPP

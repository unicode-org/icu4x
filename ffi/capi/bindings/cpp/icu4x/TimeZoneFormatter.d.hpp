#ifndef icu4x_TimeZoneFormatter_D_HPP
#define icu4x_TimeZoneFormatter_D_HPP

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
namespace capi { struct Locale; }
class Locale;
namespace capi { struct TimeZoneFormatter; }
class TimeZoneFormatter;
struct IsoTimeZoneOptions;
class Error;
}


namespace icu4x {
namespace capi {
    struct TimeZoneFormatter;
} // namespace capi
} // namespace

namespace icu4x {
class TimeZoneFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error> create_with_localized_offset_fallback(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::Error> create_with_iso_8601_fallback(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::IsoTimeZoneOptions options);

  inline diplomat::result<std::monostate, icu4x::Error> load_generic_non_location_long(const icu4x::DataProvider& provider);

  inline diplomat::result<std::monostate, icu4x::Error> load_generic_non_location_short(const icu4x::DataProvider& provider);

  inline diplomat::result<std::monostate, icu4x::Error> load_specific_non_location_long(const icu4x::DataProvider& provider);

  inline diplomat::result<std::monostate, icu4x::Error> load_specific_non_location_short(const icu4x::DataProvider& provider);

  inline diplomat::result<std::monostate, icu4x::Error> load_generic_location_format(const icu4x::DataProvider& provider);

  inline diplomat::result<std::monostate, icu4x::Error> include_localized_offset_format();

  inline diplomat::result<std::monostate, icu4x::Error> load_iso_8601_format(icu4x::IsoTimeZoneOptions options);

  inline std::string format_custom_time_zone(const icu4x::CustomTimeZone& value) const;

  inline diplomat::result<std::string, icu4x::Error> format_custom_time_zone_no_fallback(const icu4x::CustomTimeZone& value) const;

  inline const icu4x::capi::TimeZoneFormatter* AsFFI() const;
  inline icu4x::capi::TimeZoneFormatter* AsFFI();
  inline static const icu4x::TimeZoneFormatter* FromFFI(const icu4x::capi::TimeZoneFormatter* ptr);
  inline static icu4x::TimeZoneFormatter* FromFFI(icu4x::capi::TimeZoneFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  TimeZoneFormatter() = delete;
  TimeZoneFormatter(const icu4x::TimeZoneFormatter&) = delete;
  TimeZoneFormatter(icu4x::TimeZoneFormatter&&) noexcept = delete;
  TimeZoneFormatter operator=(const icu4x::TimeZoneFormatter&) = delete;
  TimeZoneFormatter operator=(icu4x::TimeZoneFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_TimeZoneFormatter_D_HPP

#ifndef icu4x_ZonedTimeFormatter_D_HPP
#define icu4x_ZonedTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct Time; }
class Time;
namespace capi { struct TimeFormatter; }
class TimeFormatter;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
namespace capi { struct ZonedTimeFormatter; }
class ZonedTimeFormatter;
class DateTimeFormatterLoadError;
class DateTimeWriteError;
}


namespace icu4x {
namespace capi {
    struct ZonedTimeFormatter;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `FixedCalendarDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/type.FixedCalendarDateTimeFormatter.html) for more information.
 */
class ZonedTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_long(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_short(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_long(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_short(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_location(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_location_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_exemplar_city(const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_exemplar_city_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::TimeFormatter& formatter);

  /**
   * See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.FixedCalendarDateTimeFormatter.html#method.format) for more information.
   */
  inline diplomat::result<std::string, icu4x::DateTimeWriteError> format(const icu4x::Time& time, const icu4x::TimeZoneInfo& zone) const;

  inline const icu4x::capi::ZonedTimeFormatter* AsFFI() const;
  inline icu4x::capi::ZonedTimeFormatter* AsFFI();
  inline static const icu4x::ZonedTimeFormatter* FromFFI(const icu4x::capi::ZonedTimeFormatter* ptr);
  inline static icu4x::ZonedTimeFormatter* FromFFI(icu4x::capi::ZonedTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ZonedTimeFormatter() = delete;
  ZonedTimeFormatter(const icu4x::ZonedTimeFormatter&) = delete;
  ZonedTimeFormatter(icu4x::ZonedTimeFormatter&&) noexcept = delete;
  ZonedTimeFormatter operator=(const icu4x::ZonedTimeFormatter&) = delete;
  ZonedTimeFormatter operator=(icu4x::ZonedTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ZonedTimeFormatter_D_HPP

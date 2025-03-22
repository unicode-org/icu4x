#ifndef icu4x_TimeZoneFormatter_D_HPP
#define icu4x_TimeZoneFormatter_D_HPP

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
namespace capi { struct TimeZoneFormatter; }
class TimeZoneFormatter;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
class DateTimeFormatterLoadError;
class DateTimeWriteError;
}


namespace icu4x {
namespace capi {
    struct TimeZoneFormatter;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `FixedCalendarDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/type.FixedCalendarDateTimeFormatter.html) for more information.
 */
class TimeZoneFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_long(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_short(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_specific_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_long(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_short(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_generic_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_location(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_location_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_exemplar_city(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneFormatter>, icu4x::DateTimeFormatterLoadError> create_exemplar_city_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  /**
   * See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.FixedCalendarDateTimeFormatter.html#method.format) for more information.
   */
  inline diplomat::result<std::string, icu4x::DateTimeWriteError> format(const icu4x::TimeZoneInfo& zone) const;

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

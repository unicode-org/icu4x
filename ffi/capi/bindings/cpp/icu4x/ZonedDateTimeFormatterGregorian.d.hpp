#ifndef icu4x_ZonedDateTimeFormatterGregorian_D_HPP
#define icu4x_ZonedDateTimeFormatterGregorian_D_HPP

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
namespace capi { struct DateTimeFormatterGregorian; }
class DateTimeFormatterGregorian;
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct Time; }
class Time;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
namespace capi { struct ZonedDateTimeFormatterGregorian; }
class ZonedDateTimeFormatterGregorian;
class DateTimeFormatterLoadError;
class DateTimeWriteError;
}


namespace icu4x {
namespace capi {
    struct ZonedDateTimeFormatterGregorian;
} // namespace capi
} // namespace

namespace icu4x {
class ZonedDateTimeFormatterGregorian {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline diplomat::result<std::string, icu4x::DateTimeWriteError> format_iso(const icu4x::IsoDate& date, const icu4x::Time& time, const icu4x::TimeZoneInfo& zone) const;

  inline const icu4x::capi::ZonedDateTimeFormatterGregorian* AsFFI() const;
  inline icu4x::capi::ZonedDateTimeFormatterGregorian* AsFFI();
  inline static const icu4x::ZonedDateTimeFormatterGregorian* FromFFI(const icu4x::capi::ZonedDateTimeFormatterGregorian* ptr);
  inline static icu4x::ZonedDateTimeFormatterGregorian* FromFFI(icu4x::capi::ZonedDateTimeFormatterGregorian* ptr);
  inline static void operator delete(void* ptr);
private:
  ZonedDateTimeFormatterGregorian() = delete;
  ZonedDateTimeFormatterGregorian(const icu4x::ZonedDateTimeFormatterGregorian&) = delete;
  ZonedDateTimeFormatterGregorian(icu4x::ZonedDateTimeFormatterGregorian&&) noexcept = delete;
  ZonedDateTimeFormatterGregorian operator=(const icu4x::ZonedDateTimeFormatterGregorian&) = delete;
  ZonedDateTimeFormatterGregorian operator=(icu4x::ZonedDateTimeFormatterGregorian&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ZonedDateTimeFormatterGregorian_D_HPP

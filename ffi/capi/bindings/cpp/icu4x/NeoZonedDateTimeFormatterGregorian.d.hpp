#ifndef icu4x_NeoZonedDateTimeFormatterGregorian_D_HPP
#define icu4x_NeoZonedDateTimeFormatterGregorian_D_HPP

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
namespace capi { struct NeoZonedDateTimeFormatterGregorian; }
class NeoZonedDateTimeFormatterGregorian;
namespace capi { struct Time; }
class Time;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
class DateTimeFormatterLoadError;
class DateTimeWriteError;
}


namespace icu4x {
namespace capi {
    struct NeoZonedDateTimeFormatterGregorian;
} // namespace capi
} // namespace

namespace icu4x {
class NeoZonedDateTimeFormatterGregorian {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_generic_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_specific_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long(const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoZonedDateTimeFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_localized_offset_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::DateTimeFormatterGregorian& formatter);

  inline diplomat::result<std::string, icu4x::DateTimeWriteError> format_iso(const icu4x::IsoDate& date, const icu4x::Time& time, const icu4x::TimeZoneInfo& zone) const;

  inline const icu4x::capi::NeoZonedDateTimeFormatterGregorian* AsFFI() const;
  inline icu4x::capi::NeoZonedDateTimeFormatterGregorian* AsFFI();
  inline static const icu4x::NeoZonedDateTimeFormatterGregorian* FromFFI(const icu4x::capi::NeoZonedDateTimeFormatterGregorian* ptr);
  inline static icu4x::NeoZonedDateTimeFormatterGregorian* FromFFI(icu4x::capi::NeoZonedDateTimeFormatterGregorian* ptr);
  inline static void operator delete(void* ptr);
private:
  NeoZonedDateTimeFormatterGregorian() = delete;
  NeoZonedDateTimeFormatterGregorian(const icu4x::NeoZonedDateTimeFormatterGregorian&) = delete;
  NeoZonedDateTimeFormatterGregorian(icu4x::NeoZonedDateTimeFormatterGregorian&&) noexcept = delete;
  NeoZonedDateTimeFormatterGregorian operator=(const icu4x::NeoZonedDateTimeFormatterGregorian&) = delete;
  NeoZonedDateTimeFormatterGregorian operator=(icu4x::NeoZonedDateTimeFormatterGregorian&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_NeoZonedDateTimeFormatterGregorian_D_HPP

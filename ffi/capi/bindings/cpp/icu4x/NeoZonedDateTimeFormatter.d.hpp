#ifndef icu4x_NeoZonedDateTimeFormatter_D_HPP
#define icu4x_NeoZonedDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Date; }
class Date;
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct Time; }
class Time;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
struct DateTimeMismatchedCalendarError;
class DateTimeWriteError;
}


namespace icu4x {
namespace capi {
    struct NeoZonedDateTimeFormatter;
} // namespace capi
} // namespace

namespace icu4x {
class NeoZonedDateTimeFormatter {
public:

  inline diplomat::result<std::string, icu4x::DateTimeWriteError> format_iso(const icu4x::IsoDate& date, const icu4x::Time& time, const icu4x::TimeZoneInfo& zone) const;

  inline diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError> format_same_calendar(const icu4x::Date& _date, const icu4x::Time& _time) const;

  inline const icu4x::capi::NeoZonedDateTimeFormatter* AsFFI() const;
  inline icu4x::capi::NeoZonedDateTimeFormatter* AsFFI();
  inline static const icu4x::NeoZonedDateTimeFormatter* FromFFI(const icu4x::capi::NeoZonedDateTimeFormatter* ptr);
  inline static icu4x::NeoZonedDateTimeFormatter* FromFFI(icu4x::capi::NeoZonedDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  NeoZonedDateTimeFormatter() = delete;
  NeoZonedDateTimeFormatter(const icu4x::NeoZonedDateTimeFormatter&) = delete;
  NeoZonedDateTimeFormatter(icu4x::NeoZonedDateTimeFormatter&&) noexcept = delete;
  NeoZonedDateTimeFormatter operator=(const icu4x::NeoZonedDateTimeFormatter&) = delete;
  NeoZonedDateTimeFormatter operator=(icu4x::NeoZonedDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_NeoZonedDateTimeFormatter_D_HPP

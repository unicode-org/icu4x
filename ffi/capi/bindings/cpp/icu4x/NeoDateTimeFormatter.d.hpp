#ifndef icu4x_NeoDateTimeFormatter_D_HPP
#define icu4x_NeoDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct NeoDateTimeFormatter; }
class NeoDateTimeFormatter;
namespace capi { struct Time; }
class Time;
struct DateTimeFieldSetBuilder;
class DateTimeAlignment;
class DateTimeFormatterBuildOrLoadError;
class DateTimeFormatterLoadError;
class NeoDateTimeLength;
class TimePrecision;
class YearStyle;
}


namespace icu4x {
namespace capi {
    struct NeoDateTimeFormatter;
} // namespace capi
} // namespace

namespace icu4x {
class NeoDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterBuildOrLoadError> create_from_builder(const icu4x::Locale& locale, icu4x::DateTimeFieldSetBuilder builder);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_dt(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_mdt(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_ymdt(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment, icu4x::YearStyle year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_det(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_mdet(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_ymdet(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment, icu4x::YearStyle year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> create_et(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment);

  inline std::string format_iso(const icu4x::IsoDate& date, const icu4x::Time& time) const;

  inline const icu4x::capi::NeoDateTimeFormatter* AsFFI() const;
  inline icu4x::capi::NeoDateTimeFormatter* AsFFI();
  inline static const icu4x::NeoDateTimeFormatter* FromFFI(const icu4x::capi::NeoDateTimeFormatter* ptr);
  inline static icu4x::NeoDateTimeFormatter* FromFFI(icu4x::capi::NeoDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  NeoDateTimeFormatter() = delete;
  NeoDateTimeFormatter(const icu4x::NeoDateTimeFormatter&) = delete;
  NeoDateTimeFormatter(icu4x::NeoDateTimeFormatter&&) noexcept = delete;
  NeoDateTimeFormatter operator=(const icu4x::NeoDateTimeFormatter&) = delete;
  NeoDateTimeFormatter operator=(icu4x::NeoDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_NeoDateTimeFormatter_D_HPP

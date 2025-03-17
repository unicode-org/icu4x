#ifndef icu4x_NeoDateFormatter_D_HPP
#define icu4x_NeoDateFormatter_D_HPP

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
namespace capi { struct Date; }
class Date;
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct NeoDateFormatter; }
class NeoDateFormatter;
struct DateTimeMismatchedCalendarError;
class DateTimeAlignment;
class DateTimeFormatterLoadError;
class DateTimeLength;
class YearStyle;
}


namespace icu4x {
namespace capi {
    struct NeoDateFormatter;
} // namespace capi
} // namespace

namespace icu4x {
class NeoDateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_d(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_d_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_md(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_md_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_ymd(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_ymd_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_de(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_de_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_mde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_mde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_ymde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_ymde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_e(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_e_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_m(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_m_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_ym(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_ym_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_y(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> create_y_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline std::string format_iso(const icu4x::IsoDate& date) const;

  inline diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError> format_same_calendar(const icu4x::Date& date) const;

  inline const icu4x::capi::NeoDateFormatter* AsFFI() const;
  inline icu4x::capi::NeoDateFormatter* AsFFI();
  inline static const icu4x::NeoDateFormatter* FromFFI(const icu4x::capi::NeoDateFormatter* ptr);
  inline static icu4x::NeoDateFormatter* FromFFI(icu4x::capi::NeoDateFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  NeoDateFormatter() = delete;
  NeoDateFormatter(const icu4x::NeoDateFormatter&) = delete;
  NeoDateFormatter(icu4x::NeoDateFormatter&&) noexcept = delete;
  NeoDateFormatter operator=(const icu4x::NeoDateFormatter&) = delete;
  NeoDateFormatter operator=(icu4x::NeoDateFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_NeoDateFormatter_D_HPP

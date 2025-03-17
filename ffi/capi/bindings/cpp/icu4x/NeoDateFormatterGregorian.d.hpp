#ifndef icu4x_NeoDateFormatterGregorian_D_HPP
#define icu4x_NeoDateFormatterGregorian_D_HPP

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
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct NeoDateFormatterGregorian; }
class NeoDateFormatterGregorian;
class DateTimeAlignment;
class DateTimeFormatterLoadError;
class DateTimeLength;
class YearStyle;
}


namespace icu4x {
namespace capi {
    struct NeoDateFormatterGregorian;
} // namespace capi
} // namespace

namespace icu4x {
class NeoDateFormatterGregorian {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_d(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_d_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_md(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_md_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_ymd(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_ymd_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_de(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_de_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_mde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_mde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_ymde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_ymde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_e(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_e_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_m(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_m_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_ym(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_ym_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_y(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline static diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> create_y_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style);

  inline std::string format_iso(const icu4x::IsoDate& date) const;

  inline const icu4x::capi::NeoDateFormatterGregorian* AsFFI() const;
  inline icu4x::capi::NeoDateFormatterGregorian* AsFFI();
  inline static const icu4x::NeoDateFormatterGregorian* FromFFI(const icu4x::capi::NeoDateFormatterGregorian* ptr);
  inline static icu4x::NeoDateFormatterGregorian* FromFFI(icu4x::capi::NeoDateFormatterGregorian* ptr);
  inline static void operator delete(void* ptr);
private:
  NeoDateFormatterGregorian() = delete;
  NeoDateFormatterGregorian(const icu4x::NeoDateFormatterGregorian&) = delete;
  NeoDateFormatterGregorian(icu4x::NeoDateFormatterGregorian&&) noexcept = delete;
  NeoDateFormatterGregorian operator=(const icu4x::NeoDateFormatterGregorian&) = delete;
  NeoDateFormatterGregorian operator=(icu4x::NeoDateFormatterGregorian&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_NeoDateFormatterGregorian_D_HPP

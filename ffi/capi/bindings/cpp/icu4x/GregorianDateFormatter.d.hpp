#ifndef icu4x_GregorianDateFormatter_D_HPP
#define icu4x_GregorianDateFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct GregorianDateFormatter; }
class GregorianDateFormatter;
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct IsoDateTime; }
class IsoDateTime;
namespace capi { struct Locale; }
class Locale;
class DateLength;
class Error;
}


namespace icu4x {
namespace capi {
    struct GregorianDateFormatter;
} // namespace capi
} // namespace

namespace icu4x {
class GregorianDateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::GregorianDateFormatter>, icu4x::Error> create_with_length(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DateLength length);

  inline std::string format_iso_date(const icu4x::IsoDate& value) const;

  inline std::string format_iso_datetime(const icu4x::IsoDateTime& value) const;

  inline const icu4x::capi::GregorianDateFormatter* AsFFI() const;
  inline icu4x::capi::GregorianDateFormatter* AsFFI();
  inline static const icu4x::GregorianDateFormatter* FromFFI(const icu4x::capi::GregorianDateFormatter* ptr);
  inline static icu4x::GregorianDateFormatter* FromFFI(icu4x::capi::GregorianDateFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  GregorianDateFormatter() = delete;
  GregorianDateFormatter(const icu4x::GregorianDateFormatter&) = delete;
  GregorianDateFormatter(icu4x::GregorianDateFormatter&&) noexcept = delete;
  GregorianDateFormatter operator=(const icu4x::GregorianDateFormatter&) = delete;
  GregorianDateFormatter operator=(icu4x::GregorianDateFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_GregorianDateFormatter_D_HPP

#ifndef GregorianDateFormatter_D_HPP
#define GregorianDateFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct IsoDate; }
class IsoDate;
namespace diplomat::capi { struct IsoDateTime; }
class IsoDateTime;
namespace diplomat::capi { struct Locale; }
class Locale;
class DateLength;
class Error;


namespace diplomat {
namespace capi {
    struct GregorianDateFormatter;
} // namespace capi
} // namespace

class GregorianDateFormatter {
public:

  inline static diplomat::result<std::unique_ptr<GregorianDateFormatter>, Error> create_with_length(const DataProvider& provider, const Locale& locale, DateLength length);

  inline std::string format_iso_date(const IsoDate& value) const;

  inline std::string format_iso_datetime(const IsoDateTime& value) const;

  inline const diplomat::capi::GregorianDateFormatter* AsFFI() const;
  inline diplomat::capi::GregorianDateFormatter* AsFFI();
  inline static const GregorianDateFormatter* FromFFI(const diplomat::capi::GregorianDateFormatter* ptr);
  inline static GregorianDateFormatter* FromFFI(diplomat::capi::GregorianDateFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  GregorianDateFormatter() = delete;
  GregorianDateFormatter(const GregorianDateFormatter&) = delete;
  GregorianDateFormatter(GregorianDateFormatter&&) noexcept = delete;
  GregorianDateFormatter operator=(const GregorianDateFormatter&) = delete;
  GregorianDateFormatter operator=(GregorianDateFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GregorianDateFormatter_D_HPP

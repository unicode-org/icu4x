#ifndef GregorianDateTimeFormatter_D_HPP
#define GregorianDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct IsoDateTime IsoDateTime; }
class IsoDateTime;
namespace capi {typedef struct Locale Locale; }
class Locale;
class DateLength;
class Error;
class TimeLength;


namespace diplomat {
namespace capi {
    typedef struct GregorianDateTimeFormatter GregorianDateTimeFormatter;
} // namespace capi
} // namespace

class GregorianDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error> create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length);

  inline std::string format_iso_datetime(const IsoDateTime& value) const;

  inline const diplomat::capi::GregorianDateTimeFormatter* AsFFI() const;
  inline diplomat::capi::GregorianDateTimeFormatter* AsFFI();
  inline static const GregorianDateTimeFormatter* FromFFI(const diplomat::capi::GregorianDateTimeFormatter* ptr);
  inline static GregorianDateTimeFormatter* FromFFI(diplomat::capi::GregorianDateTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  GregorianDateTimeFormatter() = delete;
  GregorianDateTimeFormatter(const GregorianDateTimeFormatter&) = delete;
  GregorianDateTimeFormatter(GregorianDateTimeFormatter&&) noexcept = delete;
  GregorianDateTimeFormatter operator=(const GregorianDateTimeFormatter&) = delete;
  GregorianDateTimeFormatter operator=(GregorianDateTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GregorianDateTimeFormatter_D_HPP

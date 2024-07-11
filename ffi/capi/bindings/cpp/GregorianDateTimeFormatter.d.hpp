#ifndef GregorianDateTimeFormatter_D_HPP
#define GregorianDateTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DateLength.d.hpp"
#include "Error.d.hpp"
#include "TimeLength.d.hpp"

class DataProvider;
class IsoDateTime;
class Locale;
class DateLength;
class Error;
class TimeLength;


namespace capi {
    typedef struct GregorianDateTimeFormatter GregorianDateTimeFormatter;
}

class GregorianDateTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error> create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length);

  inline std::string format_iso_datetime(const IsoDateTime& value) const;

  inline const capi::GregorianDateTimeFormatter* AsFFI() const;
  inline capi::GregorianDateTimeFormatter* AsFFI();
  inline static const GregorianDateTimeFormatter* FromFFI(const capi::GregorianDateTimeFormatter* ptr);
  inline static GregorianDateTimeFormatter* FromFFI(capi::GregorianDateTimeFormatter* ptr);
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

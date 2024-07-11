#ifndef TimeFormatter_D_HPP
#define TimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Error.d.hpp"
#include "TimeLength.d.hpp"

class DataProvider;
class DateTime;
class IsoDateTime;
class Locale;
class Time;
class Error;
class TimeLength;


namespace capi {
    typedef struct TimeFormatter TimeFormatter;
}

class TimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<TimeFormatter>, Error> create_with_length(const DataProvider& provider, const Locale& locale, TimeLength length);

  inline std::string format_time(const Time& value) const;

  inline std::string format_datetime(const DateTime& value) const;

  inline std::string format_iso_datetime(const IsoDateTime& value) const;

  inline const capi::TimeFormatter* AsFFI() const;
  inline capi::TimeFormatter* AsFFI();
  inline static const TimeFormatter* FromFFI(const capi::TimeFormatter* ptr);
  inline static TimeFormatter* FromFFI(capi::TimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  TimeFormatter() = delete;
  TimeFormatter(const TimeFormatter&) = delete;
  TimeFormatter(TimeFormatter&&) noexcept = delete;
  TimeFormatter operator=(const TimeFormatter&) = delete;
  TimeFormatter operator=(TimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TimeFormatter_D_HPP

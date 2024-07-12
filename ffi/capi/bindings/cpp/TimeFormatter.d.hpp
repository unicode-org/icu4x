#ifndef TimeFormatter_D_HPP
#define TimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct DateTime; }
class DateTime;
namespace diplomat::capi { struct IsoDateTime; }
class IsoDateTime;
namespace diplomat::capi { struct Locale; }
class Locale;
namespace diplomat::capi { struct Time; }
class Time;
class Error;
class TimeLength;


namespace diplomat {
namespace capi {
    struct TimeFormatter;
} // namespace capi
} // namespace

class TimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<TimeFormatter>, Error> create_with_length(const DataProvider& provider, const Locale& locale, TimeLength length);

  inline std::string format_time(const Time& value) const;

  inline std::string format_datetime(const DateTime& value) const;

  inline std::string format_iso_datetime(const IsoDateTime& value) const;

  inline const diplomat::capi::TimeFormatter* AsFFI() const;
  inline diplomat::capi::TimeFormatter* AsFFI();
  inline static const TimeFormatter* FromFFI(const diplomat::capi::TimeFormatter* ptr);
  inline static TimeFormatter* FromFFI(diplomat::capi::TimeFormatter* ptr);
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

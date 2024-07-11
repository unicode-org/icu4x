#ifndef TimeFormatter_D_HPP
#define TimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct DateTime DateTime; }
class DateTime;
namespace capi {typedef struct IsoDateTime IsoDateTime; }
class IsoDateTime;
namespace capi {typedef struct Locale Locale; }
class Locale;
namespace capi {typedef struct Time Time; }
class Time;
class Error;
class TimeLength;


namespace diplomat {
namespace capi {
    typedef struct TimeFormatter TimeFormatter;
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

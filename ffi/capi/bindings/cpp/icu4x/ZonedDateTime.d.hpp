#ifndef icu4x_ZonedDateTime_D_HPP
#define icu4x_ZonedDateTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Date; }
class Date;
namespace capi { struct Time; }
class Time;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
}


namespace icu4x {
namespace capi {
    struct ZonedDateTime {
      icu4x::capi::Date* date;
      icu4x::capi::Time* time;
      icu4x::capi::TimeZoneInfo* zone;
    };
    
    typedef struct ZonedDateTime_option {union { ZonedDateTime ok; }; bool is_ok; } ZonedDateTime_option;
} // namespace capi
} // namespace


namespace icu4x {
struct ZonedDateTime {
  std::unique_ptr<icu4x::Date> date;
  std::unique_ptr<icu4x::Time> time;
  std::unique_ptr<icu4x::TimeZoneInfo> zone;

  inline icu4x::capi::ZonedDateTime AsFFI() const;
  inline static icu4x::ZonedDateTime FromFFI(icu4x::capi::ZonedDateTime c_struct);
};

} // namespace
#endif // icu4x_ZonedDateTime_D_HPP

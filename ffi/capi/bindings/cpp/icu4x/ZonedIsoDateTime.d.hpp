#ifndef icu4x_ZonedIsoDateTime_D_HPP
#define icu4x_ZonedIsoDateTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct IsoDate; }
class IsoDate;
namespace capi { struct Time; }
class Time;
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
}


namespace icu4x {
namespace capi {
    struct ZonedIsoDateTime {
      icu4x::capi::IsoDate* date;
      icu4x::capi::Time* time;
      icu4x::capi::TimeZoneInfo* zone;
    };
    
    typedef struct ZonedIsoDateTime_option {union { ZonedIsoDateTime ok; }; bool is_ok; } ZonedIsoDateTime_option;
} // namespace capi
} // namespace


namespace icu4x {
struct ZonedIsoDateTime {
  std::unique_ptr<icu4x::IsoDate> date;
  std::unique_ptr<icu4x::Time> time;
  std::unique_ptr<icu4x::TimeZoneInfo> zone;

  inline icu4x::capi::ZonedIsoDateTime AsFFI() const;
  inline static icu4x::ZonedIsoDateTime FromFFI(icu4x::capi::ZonedIsoDateTime c_struct);
};

} // namespace
#endif // icu4x_ZonedIsoDateTime_D_HPP

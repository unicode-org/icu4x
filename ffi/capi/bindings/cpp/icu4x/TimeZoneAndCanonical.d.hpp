#ifndef icu4x_TimeZoneAndCanonical_D_HPP
#define icu4x_TimeZoneAndCanonical_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct TimeZoneInfo; }
class TimeZoneInfo;
}


namespace icu4x {
namespace capi {
    struct TimeZoneAndCanonical {
      icu4x::capi::TimeZoneInfo* time_zone;
      diplomat::capi::DiplomatStringView canonical;
    };
    
    typedef struct TimeZoneAndCanonical_option {union { TimeZoneAndCanonical ok; }; bool is_ok; } TimeZoneAndCanonical_option;
} // namespace capi
} // namespace


namespace icu4x {
struct TimeZoneAndCanonical {
  std::unique_ptr<icu4x::TimeZoneInfo> time_zone;
  std::string_view canonical;

  inline icu4x::capi::TimeZoneAndCanonical AsFFI() const;
  inline static icu4x::TimeZoneAndCanonical FromFFI(icu4x::capi::TimeZoneAndCanonical c_struct);
};

} // namespace
#endif // icu4x_TimeZoneAndCanonical_D_HPP

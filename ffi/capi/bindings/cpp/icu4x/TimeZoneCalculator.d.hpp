#ifndef icu4x_TimeZoneCalculator_D_HPP
#define icu4x_TimeZoneCalculator_D_HPP

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
namespace capi { struct TimeZoneCalculator; }
class TimeZoneCalculator;
class DataError;
}


namespace icu4x {
namespace capi {
    struct TimeZoneCalculator;
} // namespace capi
} // namespace

namespace icu4x {
class TimeZoneCalculator {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZoneCalculator>, icu4x::DataError> create(const icu4x::DataProvider& provider);

  inline const icu4x::capi::TimeZoneCalculator* AsFFI() const;
  inline icu4x::capi::TimeZoneCalculator* AsFFI();
  inline static const icu4x::TimeZoneCalculator* FromFFI(const icu4x::capi::TimeZoneCalculator* ptr);
  inline static icu4x::TimeZoneCalculator* FromFFI(icu4x::capi::TimeZoneCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  TimeZoneCalculator() = delete;
  TimeZoneCalculator(const icu4x::TimeZoneCalculator&) = delete;
  TimeZoneCalculator(icu4x::TimeZoneCalculator&&) noexcept = delete;
  TimeZoneCalculator operator=(const icu4x::TimeZoneCalculator&) = delete;
  TimeZoneCalculator operator=(icu4x::TimeZoneCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_TimeZoneCalculator_D_HPP

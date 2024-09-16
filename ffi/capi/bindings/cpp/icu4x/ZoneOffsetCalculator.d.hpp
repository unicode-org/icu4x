#ifndef icu4x_ZoneOffsetCalculator_D_HPP
#define icu4x_ZoneOffsetCalculator_D_HPP

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
namespace capi { struct ZoneOffsetCalculator; }
class ZoneOffsetCalculator;
class DataError;
}


namespace icu4x {
namespace capi {
    struct ZoneOffsetCalculator;
} // namespace capi
} // namespace

namespace icu4x {
class ZoneOffsetCalculator {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::ZoneOffsetCalculator>, icu4x::DataError> create(const icu4x::DataProvider& provider);

  inline const icu4x::capi::ZoneOffsetCalculator* AsFFI() const;
  inline icu4x::capi::ZoneOffsetCalculator* AsFFI();
  inline static const icu4x::ZoneOffsetCalculator* FromFFI(const icu4x::capi::ZoneOffsetCalculator* ptr);
  inline static icu4x::ZoneOffsetCalculator* FromFFI(icu4x::capi::ZoneOffsetCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  ZoneOffsetCalculator() = delete;
  ZoneOffsetCalculator(const icu4x::ZoneOffsetCalculator&) = delete;
  ZoneOffsetCalculator(icu4x::ZoneOffsetCalculator&&) noexcept = delete;
  ZoneOffsetCalculator operator=(const icu4x::ZoneOffsetCalculator&) = delete;
  ZoneOffsetCalculator operator=(icu4x::ZoneOffsetCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ZoneOffsetCalculator_D_HPP

#ifndef icu4x_WeekdaySetIterator_D_HPP
#define icu4x_WeekdaySetIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
class Weekday;
}


namespace icu4x {
namespace capi {
    struct WeekdaySetIterator;
} // namespace capi
} // namespace

namespace icu4x {
class WeekdaySetIterator {
public:

  inline std::optional<icu4x::Weekday> next();

  inline const icu4x::capi::WeekdaySetIterator* AsFFI() const;
  inline icu4x::capi::WeekdaySetIterator* AsFFI();
  inline static const icu4x::WeekdaySetIterator* FromFFI(const icu4x::capi::WeekdaySetIterator* ptr);
  inline static icu4x::WeekdaySetIterator* FromFFI(icu4x::capi::WeekdaySetIterator* ptr);
  inline static void operator delete(void* ptr);
private:
  WeekdaySetIterator() = delete;
  WeekdaySetIterator(const icu4x::WeekdaySetIterator&) = delete;
  WeekdaySetIterator(icu4x::WeekdaySetIterator&&) noexcept = delete;
  WeekdaySetIterator operator=(const icu4x::WeekdaySetIterator&) = delete;
  WeekdaySetIterator operator=(icu4x::WeekdaySetIterator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_WeekdaySetIterator_D_HPP

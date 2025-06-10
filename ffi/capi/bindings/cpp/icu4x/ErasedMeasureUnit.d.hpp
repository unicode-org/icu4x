#ifndef icu4x_ErasedMeasureUnit_D_HPP
#define icu4x_ErasedMeasureUnit_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    struct ErasedMeasureUnit;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * An ICU4X Erased Measurement Unit object which represents a single unit of measurement
 * such as `meter`, `second`, `kilometer-per-hour`, `square-meter`, etc.
 *
 * You can create an instance of this object using {@link ErasedMeasureUnitParser} by calling the `parse` method.
 *
 * See the [Rust documentation for `ErasedMeasureUnit`](https://docs.rs/icu/2.0.0/icu/experimental/measure/measureunit/struct.ErasedMeasureUnit.html) for more information.
 */
class ErasedMeasureUnit {
public:

  inline const icu4x::capi::ErasedMeasureUnit* AsFFI() const;
  inline icu4x::capi::ErasedMeasureUnit* AsFFI();
  inline static const icu4x::ErasedMeasureUnit* FromFFI(const icu4x::capi::ErasedMeasureUnit* ptr);
  inline static icu4x::ErasedMeasureUnit* FromFFI(icu4x::capi::ErasedMeasureUnit* ptr);
  inline static void operator delete(void* ptr);
private:
  ErasedMeasureUnit() = delete;
  ErasedMeasureUnit(const icu4x::ErasedMeasureUnit&) = delete;
  ErasedMeasureUnit(icu4x::ErasedMeasureUnit&&) noexcept = delete;
  ErasedMeasureUnit operator=(const icu4x::ErasedMeasureUnit&) = delete;
  ErasedMeasureUnit operator=(icu4x::ErasedMeasureUnit&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ErasedMeasureUnit_D_HPP

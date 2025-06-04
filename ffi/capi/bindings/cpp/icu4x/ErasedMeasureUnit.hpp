#ifndef icu4x_ErasedMeasureUnit_HPP
#define icu4x_ErasedMeasureUnit_HPP

#include "ErasedMeasureUnit.d.hpp"

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
    extern "C" {

    void icu4x_ErasedMeasureUnit_destroy_mv1(ErasedMeasureUnit* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const icu4x::capi::ErasedMeasureUnit* icu4x::ErasedMeasureUnit::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ErasedMeasureUnit*>(this);
}

inline icu4x::capi::ErasedMeasureUnit* icu4x::ErasedMeasureUnit::AsFFI() {
  return reinterpret_cast<icu4x::capi::ErasedMeasureUnit*>(this);
}

inline const icu4x::ErasedMeasureUnit* icu4x::ErasedMeasureUnit::FromFFI(const icu4x::capi::ErasedMeasureUnit* ptr) {
  return reinterpret_cast<const icu4x::ErasedMeasureUnit*>(ptr);
}

inline icu4x::ErasedMeasureUnit* icu4x::ErasedMeasureUnit::FromFFI(icu4x::capi::ErasedMeasureUnit* ptr) {
  return reinterpret_cast<icu4x::ErasedMeasureUnit*>(ptr);
}

inline void icu4x::ErasedMeasureUnit::operator delete(void* ptr) {
  icu4x::capi::icu4x_ErasedMeasureUnit_destroy_mv1(reinterpret_cast<icu4x::capi::ErasedMeasureUnit*>(ptr));
}


#endif // icu4x_ErasedMeasureUnit_HPP

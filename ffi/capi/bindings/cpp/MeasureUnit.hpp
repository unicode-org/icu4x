#ifndef MeasureUnit_HPP
#define MeasureUnit_HPP

#include "MeasureUnit.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    void ICU4XMeasureUnit_destroy(MeasureUnit* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline const diplomat::capi::MeasureUnit* MeasureUnit::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MeasureUnit*>(this);
}

inline diplomat::capi::MeasureUnit* MeasureUnit::AsFFI() {
  return reinterpret_cast<diplomat::capi::MeasureUnit*>(this);
}

inline const MeasureUnit* MeasureUnit::FromFFI(const diplomat::capi::MeasureUnit* ptr) {
  return reinterpret_cast<const MeasureUnit*>(ptr);
}

inline MeasureUnit* MeasureUnit::FromFFI(diplomat::capi::MeasureUnit* ptr) {
  return reinterpret_cast<MeasureUnit*>(ptr);
}

inline void MeasureUnit::operator delete(void* ptr) {
  diplomat::capi::ICU4XMeasureUnit_destroy(reinterpret_cast<diplomat::capi::MeasureUnit*>(ptr));
}


#endif // MeasureUnit_HPP

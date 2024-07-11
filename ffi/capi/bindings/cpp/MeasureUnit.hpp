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


namespace capi {
    extern "C" {
    
    
    void ICU4XMeasureUnit_destroy(MeasureUnit* self);
    
    } // extern "C"
}

inline const capi::MeasureUnit* MeasureUnit::AsFFI() const {
  return reinterpret_cast<const capi::MeasureUnit*>(this);
}

inline capi::MeasureUnit* MeasureUnit::AsFFI() {
  return reinterpret_cast<capi::MeasureUnit*>(this);
}

inline const MeasureUnit* MeasureUnit::FromFFI(const capi::MeasureUnit* ptr) {
  return reinterpret_cast<const MeasureUnit*>(ptr);
}

inline MeasureUnit* MeasureUnit::FromFFI(capi::MeasureUnit* ptr) {
  return reinterpret_cast<MeasureUnit*>(ptr);
}

inline void MeasureUnit::operator delete(void* ptr) {
  capi::ICU4XMeasureUnit_destroy(reinterpret_cast<capi::MeasureUnit*>(ptr));
}


#endif // MeasureUnit_HPP

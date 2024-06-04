#ifndef ICU4XMeasureUnit_HPP
#define ICU4XMeasureUnit_HPP

#include "ICU4XMeasureUnit.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XMeasureUnit.h"


inline const capi::ICU4XMeasureUnit* ICU4XMeasureUnit::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XMeasureUnit*>(this);
}

inline capi::ICU4XMeasureUnit* ICU4XMeasureUnit::AsFFI() {
  return reinterpret_cast<capi::ICU4XMeasureUnit*>(this);
}

inline const ICU4XMeasureUnit* ICU4XMeasureUnit::FromFFI(const capi::ICU4XMeasureUnit* ptr) {
  return reinterpret_cast<const ICU4XMeasureUnit*>(ptr);
}

inline ICU4XMeasureUnit* ICU4XMeasureUnit::FromFFI(capi::ICU4XMeasureUnit* ptr) {
  return reinterpret_cast<ICU4XMeasureUnit*>(ptr);
}

inline void ICU4XMeasureUnit::operator delete(void* ptr) {
  capi::ICU4XMeasureUnit_destroy(reinterpret_cast<capi::ICU4XMeasureUnit*>(ptr));
}


#endif // ICU4XMeasureUnit_HPP

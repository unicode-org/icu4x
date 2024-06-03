#ifndef ICU4XUnitsConverter_HPP
#define ICU4XUnitsConverter_HPP

#include "ICU4XUnitsConverter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XUnitsConverter.h"


inline double ICU4XUnitsConverter::convert_f64(double value) const {
  auto result = capi::ICU4XUnitsConverter_convert_f64(this->AsFFI(),
    value);
  return result;
}

inline std::unique_ptr<ICU4XUnitsConverter> ICU4XUnitsConverter::clone() const {
  auto result = capi::ICU4XUnitsConverter_clone(this->AsFFI());
  return std::unique_ptr<ICU4XUnitsConverter>(ICU4XUnitsConverter::FromFFI(result));
}

inline const capi::ICU4XUnitsConverter* ICU4XUnitsConverter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XUnitsConverter*>(this);
}

inline capi::ICU4XUnitsConverter* ICU4XUnitsConverter::AsFFI() {
  return reinterpret_cast<capi::ICU4XUnitsConverter*>(this);
}

inline const ICU4XUnitsConverter* ICU4XUnitsConverter::FromFFI(const capi::ICU4XUnitsConverter* ptr) {
  return reinterpret_cast<const ICU4XUnitsConverter*>(ptr);
}

inline ICU4XUnitsConverter* ICU4XUnitsConverter::FromFFI(capi::ICU4XUnitsConverter* ptr) {
  return reinterpret_cast<ICU4XUnitsConverter*>(ptr);
}

inline void ICU4XUnitsConverter::operator delete(void* ptr) {
  capi::ICU4XUnitsConverter_destroy(reinterpret_cast<capi::ICU4XUnitsConverter*>(ptr));
}


#endif // ICU4XUnitsConverter_HPP

#ifndef UnitsConverter_HPP
#define UnitsConverter_HPP

#include "UnitsConverter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    double ICU4XUnitsConverter_convert_f64(const UnitsConverter* self, double value);
    
    UnitsConverter* ICU4XUnitsConverter_clone(const UnitsConverter* self);
    
    
    void ICU4XUnitsConverter_destroy(UnitsConverter* self);
    
    } // extern "C"
}

inline double UnitsConverter::convert_f64(double value) const {
  auto result = capi::ICU4XUnitsConverter_convert_f64(this->AsFFI(),
    value);
  return result;
}

inline std::unique_ptr<UnitsConverter> UnitsConverter::clone() const {
  auto result = capi::ICU4XUnitsConverter_clone(this->AsFFI());
  return std::unique_ptr<UnitsConverter>(UnitsConverter::FromFFI(result));
}

inline const capi::UnitsConverter* UnitsConverter::AsFFI() const {
  return reinterpret_cast<const capi::UnitsConverter*>(this);
}

inline capi::UnitsConverter* UnitsConverter::AsFFI() {
  return reinterpret_cast<capi::UnitsConverter*>(this);
}

inline const UnitsConverter* UnitsConverter::FromFFI(const capi::UnitsConverter* ptr) {
  return reinterpret_cast<const UnitsConverter*>(ptr);
}

inline UnitsConverter* UnitsConverter::FromFFI(capi::UnitsConverter* ptr) {
  return reinterpret_cast<UnitsConverter*>(ptr);
}

inline void UnitsConverter::operator delete(void* ptr) {
  capi::ICU4XUnitsConverter_destroy(reinterpret_cast<capi::UnitsConverter*>(ptr));
}


#endif // UnitsConverter_HPP

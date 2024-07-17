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


namespace diplomat {
namespace capi {
    extern "C" {
    
    double ICU4XUnitsConverter_convert_f64(const diplomat::capi::UnitsConverter* self, double value);
    
    diplomat::capi::UnitsConverter* ICU4XUnitsConverter_clone(const diplomat::capi::UnitsConverter* self);
    
    
    void ICU4XUnitsConverter_destroy(UnitsConverter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline double UnitsConverter::convert_double(double value) const {
  auto result = diplomat::capi::ICU4XUnitsConverter_convert_f64(this->AsFFI(),
    value);
  return result;
}

inline std::unique_ptr<UnitsConverter> UnitsConverter::clone() const {
  auto result = diplomat::capi::ICU4XUnitsConverter_clone(this->AsFFI());
  return std::unique_ptr<UnitsConverter>(UnitsConverter::FromFFI(result));
}

inline const diplomat::capi::UnitsConverter* UnitsConverter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::UnitsConverter*>(this);
}

inline diplomat::capi::UnitsConverter* UnitsConverter::AsFFI() {
  return reinterpret_cast<diplomat::capi::UnitsConverter*>(this);
}

inline const UnitsConverter* UnitsConverter::FromFFI(const diplomat::capi::UnitsConverter* ptr) {
  return reinterpret_cast<const UnitsConverter*>(ptr);
}

inline UnitsConverter* UnitsConverter::FromFFI(diplomat::capi::UnitsConverter* ptr) {
  return reinterpret_cast<UnitsConverter*>(ptr);
}

inline void UnitsConverter::operator delete(void* ptr) {
  diplomat::capi::ICU4XUnitsConverter_destroy(reinterpret_cast<diplomat::capi::UnitsConverter*>(ptr));
}


#endif // UnitsConverter_HPP

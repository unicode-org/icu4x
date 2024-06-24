#ifndef ICU4XLocaleFallbackerWithConfig_HPP
#define ICU4XLocaleFallbackerWithConfig_HPP

#include "ICU4XLocaleFallbackerWithConfig.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XLocaleFallbackIterator.hpp"


namespace capi {
    extern "C" {
    
    ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackerWithConfig_fallback_for_locale(const ICU4XLocaleFallbackerWithConfig* self, const ICU4XLocale* locale);
    
    
    void ICU4XLocaleFallbackerWithConfig_destroy(ICU4XLocaleFallbackerWithConfig* self);
    
    } // extern "C"
}

inline std::unique_ptr<ICU4XLocaleFallbackIterator> ICU4XLocaleFallbackerWithConfig::fallback_for_locale(const ICU4XLocale& locale) const {
  auto result = capi::ICU4XLocaleFallbackerWithConfig_fallback_for_locale(this->AsFFI(),
    locale.AsFFI());
  return std::unique_ptr<ICU4XLocaleFallbackIterator>(ICU4XLocaleFallbackIterator::FromFFI(result));
}

inline const capi::ICU4XLocaleFallbackerWithConfig* ICU4XLocaleFallbackerWithConfig::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleFallbackerWithConfig*>(this);
}

inline capi::ICU4XLocaleFallbackerWithConfig* ICU4XLocaleFallbackerWithConfig::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleFallbackerWithConfig*>(this);
}

inline const ICU4XLocaleFallbackerWithConfig* ICU4XLocaleFallbackerWithConfig::FromFFI(const capi::ICU4XLocaleFallbackerWithConfig* ptr) {
  return reinterpret_cast<const ICU4XLocaleFallbackerWithConfig*>(ptr);
}

inline ICU4XLocaleFallbackerWithConfig* ICU4XLocaleFallbackerWithConfig::FromFFI(capi::ICU4XLocaleFallbackerWithConfig* ptr) {
  return reinterpret_cast<ICU4XLocaleFallbackerWithConfig*>(ptr);
}

inline void ICU4XLocaleFallbackerWithConfig::operator delete(void* ptr) {
  capi::ICU4XLocaleFallbackerWithConfig_destroy(reinterpret_cast<capi::ICU4XLocaleFallbackerWithConfig*>(ptr));
}


#endif // ICU4XLocaleFallbackerWithConfig_HPP

#ifndef LocaleFallbackerWithConfig_HPP
#define LocaleFallbackerWithConfig_HPP

#include "LocaleFallbackerWithConfig.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Locale.hpp"
#include "LocaleFallbackIterator.hpp"


namespace capi {
    extern "C" {
    
    LocaleFallbackIterator* ICU4XLocaleFallbackerWithConfig_fallback_for_locale(const LocaleFallbackerWithConfig* self, const Locale* locale);
    
    
    void ICU4XLocaleFallbackerWithConfig_destroy(LocaleFallbackerWithConfig* self);
    
    } // extern "C"
}

inline std::unique_ptr<LocaleFallbackIterator> LocaleFallbackerWithConfig::fallback_for_locale(const Locale& locale) const {
  auto result = capi::ICU4XLocaleFallbackerWithConfig_fallback_for_locale(this->AsFFI(),
    locale.AsFFI());
  return std::unique_ptr<LocaleFallbackIterator>(LocaleFallbackIterator::FromFFI(result));
}

inline const capi::LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::AsFFI() const {
  return reinterpret_cast<const capi::LocaleFallbackerWithConfig*>(this);
}

inline capi::LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::AsFFI() {
  return reinterpret_cast<capi::LocaleFallbackerWithConfig*>(this);
}

inline const LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::FromFFI(const capi::LocaleFallbackerWithConfig* ptr) {
  return reinterpret_cast<const LocaleFallbackerWithConfig*>(ptr);
}

inline LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::FromFFI(capi::LocaleFallbackerWithConfig* ptr) {
  return reinterpret_cast<LocaleFallbackerWithConfig*>(ptr);
}

inline void LocaleFallbackerWithConfig::operator delete(void* ptr) {
  capi::ICU4XLocaleFallbackerWithConfig_destroy(reinterpret_cast<capi::LocaleFallbackerWithConfig*>(ptr));
}


#endif // LocaleFallbackerWithConfig_HPP

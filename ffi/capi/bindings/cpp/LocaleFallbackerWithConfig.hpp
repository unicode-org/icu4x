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


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::LocaleFallbackIterator* icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1(const diplomat::capi::LocaleFallbackerWithConfig* self, const diplomat::capi::Locale* locale);
    
    
    void icu4x_LocaleFallbackerWithConfig_destroy_mv1(LocaleFallbackerWithConfig* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<LocaleFallbackIterator> LocaleFallbackerWithConfig::fallback_for_locale(const Locale& locale) const {
  auto result = diplomat::capi::icu4x_LocaleFallbackerWithConfig_fallback_for_locale_mv1(this->AsFFI(),
    locale.AsFFI());
  return std::unique_ptr<LocaleFallbackIterator>(LocaleFallbackIterator::FromFFI(result));
}

inline const diplomat::capi::LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleFallbackerWithConfig*>(this);
}

inline diplomat::capi::LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleFallbackerWithConfig*>(this);
}

inline const LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::FromFFI(const diplomat::capi::LocaleFallbackerWithConfig* ptr) {
  return reinterpret_cast<const LocaleFallbackerWithConfig*>(ptr);
}

inline LocaleFallbackerWithConfig* LocaleFallbackerWithConfig::FromFFI(diplomat::capi::LocaleFallbackerWithConfig* ptr) {
  return reinterpret_cast<LocaleFallbackerWithConfig*>(ptr);
}

inline void LocaleFallbackerWithConfig::operator delete(void* ptr) {
  diplomat::capi::icu4x_LocaleFallbackerWithConfig_destroy_mv1(reinterpret_cast<diplomat::capi::LocaleFallbackerWithConfig*>(ptr));
}


#endif // LocaleFallbackerWithConfig_HPP

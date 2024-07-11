#ifndef LocaleFallbackIterator_HPP
#define LocaleFallbackIterator_HPP

#include "LocaleFallbackIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Locale.hpp"


namespace capi {
    extern "C" {
    
    Locale* ICU4XLocaleFallbackIterator_next(LocaleFallbackIterator* self);
    
    
    void ICU4XLocaleFallbackIterator_destroy(LocaleFallbackIterator* self);
    
    } // extern "C"
}

inline std::unique_ptr<Locale> LocaleFallbackIterator::next() {
  auto result = capi::ICU4XLocaleFallbackIterator_next(this->AsFFI());
  return std::unique_ptr<Locale>(Locale::FromFFI(result));
}

inline const capi::LocaleFallbackIterator* LocaleFallbackIterator::AsFFI() const {
  return reinterpret_cast<const capi::LocaleFallbackIterator*>(this);
}

inline capi::LocaleFallbackIterator* LocaleFallbackIterator::AsFFI() {
  return reinterpret_cast<capi::LocaleFallbackIterator*>(this);
}

inline const LocaleFallbackIterator* LocaleFallbackIterator::FromFFI(const capi::LocaleFallbackIterator* ptr) {
  return reinterpret_cast<const LocaleFallbackIterator*>(ptr);
}

inline LocaleFallbackIterator* LocaleFallbackIterator::FromFFI(capi::LocaleFallbackIterator* ptr) {
  return reinterpret_cast<LocaleFallbackIterator*>(ptr);
}

inline void LocaleFallbackIterator::operator delete(void* ptr) {
  capi::ICU4XLocaleFallbackIterator_destroy(reinterpret_cast<capi::LocaleFallbackIterator*>(ptr));
}


#endif // LocaleFallbackIterator_HPP

#ifndef LocaleFallbackIterator_HPP
#define LocaleFallbackIterator_HPP

#include "LocaleFallbackIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::Locale* icu4x_LocaleFallbackIterator_next_mv1(diplomat::capi::LocaleFallbackIterator* self);
    
    
    void icu4x_LocaleFallbackIterator_destroy_mv1(LocaleFallbackIterator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Locale> LocaleFallbackIterator::next() {
  auto result = diplomat::capi::icu4x_LocaleFallbackIterator_next_mv1(this->AsFFI());
  return std::unique_ptr<Locale>(Locale::FromFFI(result));
}

inline const diplomat::capi::LocaleFallbackIterator* LocaleFallbackIterator::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleFallbackIterator*>(this);
}

inline diplomat::capi::LocaleFallbackIterator* LocaleFallbackIterator::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleFallbackIterator*>(this);
}

inline const LocaleFallbackIterator* LocaleFallbackIterator::FromFFI(const diplomat::capi::LocaleFallbackIterator* ptr) {
  return reinterpret_cast<const LocaleFallbackIterator*>(ptr);
}

inline LocaleFallbackIterator* LocaleFallbackIterator::FromFFI(diplomat::capi::LocaleFallbackIterator* ptr) {
  return reinterpret_cast<LocaleFallbackIterator*>(ptr);
}

inline void LocaleFallbackIterator::operator delete(void* ptr) {
  diplomat::capi::icu4x_LocaleFallbackIterator_destroy_mv1(reinterpret_cast<diplomat::capi::LocaleFallbackIterator*>(ptr));
}


#endif // LocaleFallbackIterator_HPP

#ifndef ICU4XLocaleFallbackIterator_HPP
#define ICU4XLocaleFallbackIterator_HPP

#include "ICU4XLocaleFallbackIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XLocaleFallbackIterator.h"


inline std::unique_ptr<ICU4XLocale> ICU4XLocaleFallbackIterator::next() {
  auto result = capi::ICU4XLocaleFallbackIterator_next(this->AsFFI());
  return std::unique_ptr<ICU4XLocale>(ICU4XLocale::FromFFI(result));
}

inline const capi::ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackIterator::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleFallbackIterator*>(this);
}

inline capi::ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackIterator::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleFallbackIterator*>(this);
}

inline const ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackIterator::FromFFI(const capi::ICU4XLocaleFallbackIterator* ptr) {
  return reinterpret_cast<const ICU4XLocaleFallbackIterator*>(ptr);
}

inline ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackIterator::FromFFI(capi::ICU4XLocaleFallbackIterator* ptr) {
  return reinterpret_cast<ICU4XLocaleFallbackIterator*>(ptr);
}

inline void ICU4XLocaleFallbackIterator::operator delete(void* ptr) {
  capi::ICU4XLocaleFallbackIterator_destroy(reinterpret_cast<capi::ICU4XLocaleFallbackIterator*>(ptr));
}


#endif // ICU4XLocaleFallbackIterator_HPP

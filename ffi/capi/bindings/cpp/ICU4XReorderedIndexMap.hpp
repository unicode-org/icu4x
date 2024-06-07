#ifndef ICU4XReorderedIndexMap_HPP
#define ICU4XReorderedIndexMap_HPP

#include "ICU4XReorderedIndexMap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XReorderedIndexMap.h"


inline diplomat::span<const size_t> ICU4XReorderedIndexMap::as_slice() const {
  auto result = capi::ICU4XReorderedIndexMap_as_slice(this->AsFFI());
  return diplomat::span<const size_t>(result.data, result.len);
}

inline size_t ICU4XReorderedIndexMap::len() const {
  auto result = capi::ICU4XReorderedIndexMap_len(this->AsFFI());
  return result;
}

inline bool ICU4XReorderedIndexMap::is_empty() const {
  auto result = capi::ICU4XReorderedIndexMap_is_empty(this->AsFFI());
  return result;
}

inline size_t ICU4XReorderedIndexMap::get(size_t index) const {
  auto result = capi::ICU4XReorderedIndexMap_get(this->AsFFI(),
    index);
  return result;
}

inline const capi::ICU4XReorderedIndexMap* ICU4XReorderedIndexMap::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XReorderedIndexMap*>(this);
}

inline capi::ICU4XReorderedIndexMap* ICU4XReorderedIndexMap::AsFFI() {
  return reinterpret_cast<capi::ICU4XReorderedIndexMap*>(this);
}

inline const ICU4XReorderedIndexMap* ICU4XReorderedIndexMap::FromFFI(const capi::ICU4XReorderedIndexMap* ptr) {
  return reinterpret_cast<const ICU4XReorderedIndexMap*>(ptr);
}

inline ICU4XReorderedIndexMap* ICU4XReorderedIndexMap::FromFFI(capi::ICU4XReorderedIndexMap* ptr) {
  return reinterpret_cast<ICU4XReorderedIndexMap*>(ptr);
}

inline void ICU4XReorderedIndexMap::operator delete(void* ptr) {
  capi::ICU4XReorderedIndexMap_destroy(reinterpret_cast<capi::ICU4XReorderedIndexMap*>(ptr));
}


#endif // ICU4XReorderedIndexMap_HPP

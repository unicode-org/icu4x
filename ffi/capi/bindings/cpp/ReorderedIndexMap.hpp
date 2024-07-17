#ifndef ReorderedIndexMap_HPP
#define ReorderedIndexMap_HPP

#include "ReorderedIndexMap.d.hpp"

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
    
    diplomat::capi::DiplomatUsizeView ICU4XReorderedIndexMap_as_slice(const diplomat::capi::ReorderedIndexMap* self);
    
    size_t ICU4XReorderedIndexMap_len(const diplomat::capi::ReorderedIndexMap* self);
    
    bool ICU4XReorderedIndexMap_is_empty(const diplomat::capi::ReorderedIndexMap* self);
    
    size_t ICU4XReorderedIndexMap_get(const diplomat::capi::ReorderedIndexMap* self, size_t index);
    
    
    void ICU4XReorderedIndexMap_destroy(ReorderedIndexMap* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::span<const size_t> ReorderedIndexMap::as_slice() const {
  auto result = diplomat::capi::ICU4XReorderedIndexMap_as_slice(this->AsFFI());
  return diplomat::span<const size_t>(result.data, result.len);
}

inline size_t ReorderedIndexMap::len() const {
  auto result = diplomat::capi::ICU4XReorderedIndexMap_len(this->AsFFI());
  return result;
}

inline bool ReorderedIndexMap::is_empty() const {
  auto result = diplomat::capi::ICU4XReorderedIndexMap_is_empty(this->AsFFI());
  return result;
}

inline size_t ReorderedIndexMap::get(size_t index) const {
  auto result = diplomat::capi::ICU4XReorderedIndexMap_get(this->AsFFI(),
    index);
  return result;
}

inline const diplomat::capi::ReorderedIndexMap* ReorderedIndexMap::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ReorderedIndexMap*>(this);
}

inline diplomat::capi::ReorderedIndexMap* ReorderedIndexMap::AsFFI() {
  return reinterpret_cast<diplomat::capi::ReorderedIndexMap*>(this);
}

inline const ReorderedIndexMap* ReorderedIndexMap::FromFFI(const diplomat::capi::ReorderedIndexMap* ptr) {
  return reinterpret_cast<const ReorderedIndexMap*>(ptr);
}

inline ReorderedIndexMap* ReorderedIndexMap::FromFFI(diplomat::capi::ReorderedIndexMap* ptr) {
  return reinterpret_cast<ReorderedIndexMap*>(ptr);
}

inline void ReorderedIndexMap::operator delete(void* ptr) {
  diplomat::capi::ICU4XReorderedIndexMap_destroy(reinterpret_cast<diplomat::capi::ReorderedIndexMap*>(ptr));
}


#endif // ReorderedIndexMap_HPP

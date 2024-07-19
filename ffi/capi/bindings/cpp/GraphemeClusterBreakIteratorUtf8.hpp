#ifndef GraphemeClusterBreakIteratorUtf8_HPP
#define GraphemeClusterBreakIteratorUtf8_HPP

#include "GraphemeClusterBreakIteratorUtf8.d.hpp"

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
    
    int32_t icu4x_GraphemeClusterBreakIteratorUtf8_next_mv1(diplomat::capi::GraphemeClusterBreakIteratorUtf8* self);
    
    
    void icu4x_GraphemeClusterBreakIteratorUtf8_destroy_mv1(GraphemeClusterBreakIteratorUtf8* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t GraphemeClusterBreakIteratorUtf8::next() {
  auto result = diplomat::capi::icu4x_GraphemeClusterBreakIteratorUtf8_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GraphemeClusterBreakIteratorUtf8*>(this);
}

inline diplomat::capi::GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<diplomat::capi::GraphemeClusterBreakIteratorUtf8*>(this);
}

inline const GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::FromFFI(const diplomat::capi::GraphemeClusterBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const GraphemeClusterBreakIteratorUtf8*>(ptr);
}

inline GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::FromFFI(diplomat::capi::GraphemeClusterBreakIteratorUtf8* ptr) {
  return reinterpret_cast<GraphemeClusterBreakIteratorUtf8*>(ptr);
}

inline void GraphemeClusterBreakIteratorUtf8::operator delete(void* ptr) {
  diplomat::capi::icu4x_GraphemeClusterBreakIteratorUtf8_destroy_mv1(reinterpret_cast<diplomat::capi::GraphemeClusterBreakIteratorUtf8*>(ptr));
}


#endif // GraphemeClusterBreakIteratorUtf8_HPP

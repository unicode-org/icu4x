#ifndef GraphemeClusterBreakIteratorLatin1_HPP
#define GraphemeClusterBreakIteratorLatin1_HPP

#include "GraphemeClusterBreakIteratorLatin1.d.hpp"

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
    
    int32_t icu4x_GraphemeClusterBreakIteratorLatin1_next_mv1(diplomat::capi::GraphemeClusterBreakIteratorLatin1* self);
    
    
    void icu4x_GraphemeClusterBreakIteratorLatin1_destroy_mv1(GraphemeClusterBreakIteratorLatin1* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t GraphemeClusterBreakIteratorLatin1::next() {
  auto result = diplomat::capi::icu4x_GraphemeClusterBreakIteratorLatin1_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GraphemeClusterBreakIteratorLatin1*>(this);
}

inline diplomat::capi::GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<diplomat::capi::GraphemeClusterBreakIteratorLatin1*>(this);
}

inline const GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::FromFFI(const diplomat::capi::GraphemeClusterBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const GraphemeClusterBreakIteratorLatin1*>(ptr);
}

inline GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::FromFFI(diplomat::capi::GraphemeClusterBreakIteratorLatin1* ptr) {
  return reinterpret_cast<GraphemeClusterBreakIteratorLatin1*>(ptr);
}

inline void GraphemeClusterBreakIteratorLatin1::operator delete(void* ptr) {
  diplomat::capi::icu4x_GraphemeClusterBreakIteratorLatin1_destroy_mv1(reinterpret_cast<diplomat::capi::GraphemeClusterBreakIteratorLatin1*>(ptr));
}


#endif // GraphemeClusterBreakIteratorLatin1_HPP

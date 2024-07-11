#ifndef GraphemeClusterBreakIteratorUtf16_HPP
#define GraphemeClusterBreakIteratorUtf16_HPP

#include "GraphemeClusterBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    int32_t ICU4XGraphemeClusterBreakIteratorUtf16_next(GraphemeClusterBreakIteratorUtf16* self);
    
    
    void ICU4XGraphemeClusterBreakIteratorUtf16_destroy(GraphemeClusterBreakIteratorUtf16* self);
    
    } // extern "C"
}

inline int32_t GraphemeClusterBreakIteratorUtf16::next() {
  auto result = capi::ICU4XGraphemeClusterBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline const capi::GraphemeClusterBreakIteratorUtf16* GraphemeClusterBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::GraphemeClusterBreakIteratorUtf16*>(this);
}

inline capi::GraphemeClusterBreakIteratorUtf16* GraphemeClusterBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::GraphemeClusterBreakIteratorUtf16*>(this);
}

inline const GraphemeClusterBreakIteratorUtf16* GraphemeClusterBreakIteratorUtf16::FromFFI(const capi::GraphemeClusterBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const GraphemeClusterBreakIteratorUtf16*>(ptr);
}

inline GraphemeClusterBreakIteratorUtf16* GraphemeClusterBreakIteratorUtf16::FromFFI(capi::GraphemeClusterBreakIteratorUtf16* ptr) {
  return reinterpret_cast<GraphemeClusterBreakIteratorUtf16*>(ptr);
}

inline void GraphemeClusterBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterBreakIteratorUtf16_destroy(reinterpret_cast<capi::GraphemeClusterBreakIteratorUtf16*>(ptr));
}


#endif // GraphemeClusterBreakIteratorUtf16_HPP

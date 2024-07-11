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


namespace capi {
    extern "C" {
    
    int32_t ICU4XGraphemeClusterBreakIteratorUtf8_next(GraphemeClusterBreakIteratorUtf8* self);
    
    
    void ICU4XGraphemeClusterBreakIteratorUtf8_destroy(GraphemeClusterBreakIteratorUtf8* self);
    
    } // extern "C"
}

inline int32_t GraphemeClusterBreakIteratorUtf8::next() {
  auto result = capi::ICU4XGraphemeClusterBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline const capi::GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::GraphemeClusterBreakIteratorUtf8*>(this);
}

inline capi::GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::GraphemeClusterBreakIteratorUtf8*>(this);
}

inline const GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::FromFFI(const capi::GraphemeClusterBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const GraphemeClusterBreakIteratorUtf8*>(ptr);
}

inline GraphemeClusterBreakIteratorUtf8* GraphemeClusterBreakIteratorUtf8::FromFFI(capi::GraphemeClusterBreakIteratorUtf8* ptr) {
  return reinterpret_cast<GraphemeClusterBreakIteratorUtf8*>(ptr);
}

inline void GraphemeClusterBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterBreakIteratorUtf8_destroy(reinterpret_cast<capi::GraphemeClusterBreakIteratorUtf8*>(ptr));
}


#endif // GraphemeClusterBreakIteratorUtf8_HPP

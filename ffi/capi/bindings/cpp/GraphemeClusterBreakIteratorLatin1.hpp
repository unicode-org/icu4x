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


namespace capi {
    extern "C" {
    
    int32_t ICU4XGraphemeClusterBreakIteratorLatin1_next(GraphemeClusterBreakIteratorLatin1* self);
    
    
    void ICU4XGraphemeClusterBreakIteratorLatin1_destroy(GraphemeClusterBreakIteratorLatin1* self);
    
    } // extern "C"
}

inline int32_t GraphemeClusterBreakIteratorLatin1::next() {
  auto result = capi::ICU4XGraphemeClusterBreakIteratorLatin1_next(this->AsFFI());
  return result;
}

inline const capi::GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const capi::GraphemeClusterBreakIteratorLatin1*>(this);
}

inline capi::GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<capi::GraphemeClusterBreakIteratorLatin1*>(this);
}

inline const GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::FromFFI(const capi::GraphemeClusterBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const GraphemeClusterBreakIteratorLatin1*>(ptr);
}

inline GraphemeClusterBreakIteratorLatin1* GraphemeClusterBreakIteratorLatin1::FromFFI(capi::GraphemeClusterBreakIteratorLatin1* ptr) {
  return reinterpret_cast<GraphemeClusterBreakIteratorLatin1*>(ptr);
}

inline void GraphemeClusterBreakIteratorLatin1::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterBreakIteratorLatin1_destroy(reinterpret_cast<capi::GraphemeClusterBreakIteratorLatin1*>(ptr));
}


#endif // GraphemeClusterBreakIteratorLatin1_HPP

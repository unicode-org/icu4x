#ifndef ICU4XBidiInfo_HPP
#define ICU4XBidiInfo_HPP

#include "ICU4XBidiInfo.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiParagraph.hpp"


namespace capi {
    extern "C" {
    
    size_t ICU4XBidiInfo_paragraph_count(const ICU4XBidiInfo* self);
    
    ICU4XBidiParagraph* ICU4XBidiInfo_paragraph_at(const ICU4XBidiInfo* self, size_t n);
    
    size_t ICU4XBidiInfo_size(const ICU4XBidiInfo* self);
    
    uint8_t ICU4XBidiInfo_level_at(const ICU4XBidiInfo* self, size_t pos);
    
    
    void ICU4XBidiInfo_destroy(ICU4XBidiInfo* self);
    
    } // extern "C"
}

inline size_t ICU4XBidiInfo::paragraph_count() const {
  auto result = capi::ICU4XBidiInfo_paragraph_count(this->AsFFI());
  return result;
}

inline std::unique_ptr<ICU4XBidiParagraph> ICU4XBidiInfo::paragraph_at(size_t n) const {
  auto result = capi::ICU4XBidiInfo_paragraph_at(this->AsFFI(),
    n);
  return std::unique_ptr<ICU4XBidiParagraph>(ICU4XBidiParagraph::FromFFI(result));
}

inline size_t ICU4XBidiInfo::size() const {
  auto result = capi::ICU4XBidiInfo_size(this->AsFFI());
  return result;
}

inline uint8_t ICU4XBidiInfo::level_at(size_t pos) const {
  auto result = capi::ICU4XBidiInfo_level_at(this->AsFFI(),
    pos);
  return result;
}

inline const capi::ICU4XBidiInfo* ICU4XBidiInfo::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XBidiInfo*>(this);
}

inline capi::ICU4XBidiInfo* ICU4XBidiInfo::AsFFI() {
  return reinterpret_cast<capi::ICU4XBidiInfo*>(this);
}

inline const ICU4XBidiInfo* ICU4XBidiInfo::FromFFI(const capi::ICU4XBidiInfo* ptr) {
  return reinterpret_cast<const ICU4XBidiInfo*>(ptr);
}

inline ICU4XBidiInfo* ICU4XBidiInfo::FromFFI(capi::ICU4XBidiInfo* ptr) {
  return reinterpret_cast<ICU4XBidiInfo*>(ptr);
}

inline void ICU4XBidiInfo::operator delete(void* ptr) {
  capi::ICU4XBidiInfo_destroy(reinterpret_cast<capi::ICU4XBidiInfo*>(ptr));
}


#endif // ICU4XBidiInfo_HPP

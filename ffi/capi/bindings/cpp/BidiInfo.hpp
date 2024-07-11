#ifndef BidiInfo_HPP
#define BidiInfo_HPP

#include "BidiInfo.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiParagraph.hpp"


namespace capi {
    extern "C" {
    
    size_t ICU4XBidiInfo_paragraph_count(const BidiInfo* self);
    
    BidiParagraph* ICU4XBidiInfo_paragraph_at(const BidiInfo* self, size_t n);
    
    size_t ICU4XBidiInfo_size(const BidiInfo* self);
    
    uint8_t ICU4XBidiInfo_level_at(const BidiInfo* self, size_t pos);
    
    
    void ICU4XBidiInfo_destroy(BidiInfo* self);
    
    } // extern "C"
}

inline size_t BidiInfo::paragraph_count() const {
  auto result = capi::ICU4XBidiInfo_paragraph_count(this->AsFFI());
  return result;
}

inline std::unique_ptr<BidiParagraph> BidiInfo::paragraph_at(size_t n) const {
  auto result = capi::ICU4XBidiInfo_paragraph_at(this->AsFFI(),
    n);
  return std::unique_ptr<BidiParagraph>(BidiParagraph::FromFFI(result));
}

inline size_t BidiInfo::size() const {
  auto result = capi::ICU4XBidiInfo_size(this->AsFFI());
  return result;
}

inline uint8_t BidiInfo::level_at(size_t pos) const {
  auto result = capi::ICU4XBidiInfo_level_at(this->AsFFI(),
    pos);
  return result;
}

inline const capi::BidiInfo* BidiInfo::AsFFI() const {
  return reinterpret_cast<const capi::BidiInfo*>(this);
}

inline capi::BidiInfo* BidiInfo::AsFFI() {
  return reinterpret_cast<capi::BidiInfo*>(this);
}

inline const BidiInfo* BidiInfo::FromFFI(const capi::BidiInfo* ptr) {
  return reinterpret_cast<const BidiInfo*>(ptr);
}

inline BidiInfo* BidiInfo::FromFFI(capi::BidiInfo* ptr) {
  return reinterpret_cast<BidiInfo*>(ptr);
}

inline void BidiInfo::operator delete(void* ptr) {
  capi::ICU4XBidiInfo_destroy(reinterpret_cast<capi::BidiInfo*>(ptr));
}


#endif // BidiInfo_HPP

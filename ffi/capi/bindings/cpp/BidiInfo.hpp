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


namespace diplomat {
namespace capi {
    extern "C" {
    
    size_t ICU4XBidiInfo_paragraph_count(const diplomat::capi::BidiInfo* self);
    
    diplomat::capi::BidiParagraph* ICU4XBidiInfo_paragraph_at(const diplomat::capi::BidiInfo* self, size_t n);
    
    size_t ICU4XBidiInfo_size(const diplomat::capi::BidiInfo* self);
    
    uint8_t ICU4XBidiInfo_level_at(const diplomat::capi::BidiInfo* self, size_t pos);
    
    
    void ICU4XBidiInfo_destroy(BidiInfo* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline size_t BidiInfo::paragraph_count() const {
  auto result = diplomat::capi::ICU4XBidiInfo_paragraph_count(this->AsFFI());
  return result;
}

inline std::unique_ptr<BidiParagraph> BidiInfo::paragraph_at(size_t n) const {
  auto result = diplomat::capi::ICU4XBidiInfo_paragraph_at(this->AsFFI(),
    n);
  return std::unique_ptr<BidiParagraph>(BidiParagraph::FromFFI(result));
}

inline size_t BidiInfo::size() const {
  auto result = diplomat::capi::ICU4XBidiInfo_size(this->AsFFI());
  return result;
}

inline uint8_t BidiInfo::level_at(size_t pos) const {
  auto result = diplomat::capi::ICU4XBidiInfo_level_at(this->AsFFI(),
    pos);
  return result;
}

inline const diplomat::capi::BidiInfo* BidiInfo::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::BidiInfo*>(this);
}

inline diplomat::capi::BidiInfo* BidiInfo::AsFFI() {
  return reinterpret_cast<diplomat::capi::BidiInfo*>(this);
}

inline const BidiInfo* BidiInfo::FromFFI(const diplomat::capi::BidiInfo* ptr) {
  return reinterpret_cast<const BidiInfo*>(ptr);
}

inline BidiInfo* BidiInfo::FromFFI(diplomat::capi::BidiInfo* ptr) {
  return reinterpret_cast<BidiInfo*>(ptr);
}

inline void BidiInfo::operator delete(void* ptr) {
  diplomat::capi::ICU4XBidiInfo_destroy(reinterpret_cast<diplomat::capi::BidiInfo*>(ptr));
}


#endif // BidiInfo_HPP

#ifndef BidiParagraph_HPP
#define BidiParagraph_HPP

#include "BidiParagraph.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiDirection.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    bool icu4x_BidiParagraph_set_paragraph_in_text_mv1(diplomat::capi::BidiParagraph* self, size_t n);
    
    diplomat::capi::BidiDirection icu4x_BidiParagraph_direction_mv1(const diplomat::capi::BidiParagraph* self);
    
    size_t icu4x_BidiParagraph_size_mv1(const diplomat::capi::BidiParagraph* self);
    
    size_t icu4x_BidiParagraph_range_start_mv1(const diplomat::capi::BidiParagraph* self);
    
    size_t icu4x_BidiParagraph_range_end_mv1(const diplomat::capi::BidiParagraph* self);
    
    typedef struct icu4x_BidiParagraph_reorder_line_mv1_result { bool is_ok;} icu4x_BidiParagraph_reorder_line_mv1_result;
    icu4x_BidiParagraph_reorder_line_mv1_result icu4x_BidiParagraph_reorder_line_mv1(const diplomat::capi::BidiParagraph* self, size_t range_start, size_t range_end, diplomat::capi::DiplomatWrite* write);
    
    uint8_t icu4x_BidiParagraph_level_at_mv1(const diplomat::capi::BidiParagraph* self, size_t pos);
    
    
    void icu4x_BidiParagraph_destroy_mv1(BidiParagraph* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline bool BidiParagraph::set_paragraph_in_text(size_t n) {
  auto result = diplomat::capi::icu4x_BidiParagraph_set_paragraph_in_text_mv1(this->AsFFI(),
    n);
  return result;
}

inline BidiDirection BidiParagraph::direction() const {
  auto result = diplomat::capi::icu4x_BidiParagraph_direction_mv1(this->AsFFI());
  return BidiDirection::FromFFI(result);
}

inline size_t BidiParagraph::size() const {
  auto result = diplomat::capi::icu4x_BidiParagraph_size_mv1(this->AsFFI());
  return result;
}

inline size_t BidiParagraph::range_start() const {
  auto result = diplomat::capi::icu4x_BidiParagraph_range_start_mv1(this->AsFFI());
  return result;
}

inline size_t BidiParagraph::range_end() const {
  auto result = diplomat::capi::icu4x_BidiParagraph_range_end_mv1(this->AsFFI());
  return result;
}

inline std::optional<std::string> BidiParagraph::reorder_line(size_t range_start, size_t range_end) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_BidiParagraph_reorder_line_mv1(this->AsFFI(),
    range_start,
    range_end,
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline uint8_t BidiParagraph::level_at(size_t pos) const {
  auto result = diplomat::capi::icu4x_BidiParagraph_level_at_mv1(this->AsFFI(),
    pos);
  return result;
}

inline const diplomat::capi::BidiParagraph* BidiParagraph::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::BidiParagraph*>(this);
}

inline diplomat::capi::BidiParagraph* BidiParagraph::AsFFI() {
  return reinterpret_cast<diplomat::capi::BidiParagraph*>(this);
}

inline const BidiParagraph* BidiParagraph::FromFFI(const diplomat::capi::BidiParagraph* ptr) {
  return reinterpret_cast<const BidiParagraph*>(ptr);
}

inline BidiParagraph* BidiParagraph::FromFFI(diplomat::capi::BidiParagraph* ptr) {
  return reinterpret_cast<BidiParagraph*>(ptr);
}

inline void BidiParagraph::operator delete(void* ptr) {
  diplomat::capi::icu4x_BidiParagraph_destroy_mv1(reinterpret_cast<diplomat::capi::BidiParagraph*>(ptr));
}


#endif // BidiParagraph_HPP

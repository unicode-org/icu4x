#ifndef Bidi_HPP
#define Bidi_HPP

#include "Bidi.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BidiInfo.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ReorderedIndexMap.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_Bidi_create_mv1_result {union {diplomat::capi::Bidi* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_Bidi_create_mv1_result;
    icu4x_Bidi_create_mv1_result icu4x_Bidi_create_mv1(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::BidiInfo* icu4x_Bidi_for_text_utf8_mv1(const diplomat::capi::Bidi* self, const char* text_data, size_t text_len, uint8_t default_level);
    
    diplomat::capi::ReorderedIndexMap* icu4x_Bidi_reorder_visual_mv1(const diplomat::capi::Bidi* self, const uint8_t* levels_data, size_t levels_len);
    
    bool icu4x_Bidi_level_is_rtl_mv1(uint8_t level);
    
    bool icu4x_Bidi_level_is_ltr_mv1(uint8_t level);
    
    uint8_t icu4x_Bidi_level_rtl_mv1();
    
    uint8_t icu4x_Bidi_level_ltr_mv1();
    
    
    void icu4x_Bidi_destroy_mv1(Bidi* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Bidi>, DataError> Bidi::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_Bidi_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<Bidi>, DataError>(diplomat::Ok<std::unique_ptr<Bidi>>(std::unique_ptr<Bidi>(Bidi::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Bidi>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<BidiInfo> Bidi::for_text(std::string_view text, uint8_t default_level) const {
  auto result = diplomat::capi::icu4x_Bidi_for_text_utf8_mv1(this->AsFFI(),
    text.data(),
    text.size(),
    default_level);
  return std::unique_ptr<BidiInfo>(BidiInfo::FromFFI(result));
}

inline std::unique_ptr<ReorderedIndexMap> Bidi::reorder_visual(diplomat::span<const uint8_t> levels) const {
  auto result = diplomat::capi::icu4x_Bidi_reorder_visual_mv1(this->AsFFI(),
    levels.data(),
    levels.size());
  return std::unique_ptr<ReorderedIndexMap>(ReorderedIndexMap::FromFFI(result));
}

inline bool Bidi::level_is_rtl(uint8_t level) {
  auto result = diplomat::capi::icu4x_Bidi_level_is_rtl_mv1(level);
  return result;
}

inline bool Bidi::level_is_ltr(uint8_t level) {
  auto result = diplomat::capi::icu4x_Bidi_level_is_ltr_mv1(level);
  return result;
}

inline uint8_t Bidi::level_rtl() {
  auto result = diplomat::capi::icu4x_Bidi_level_rtl_mv1();
  return result;
}

inline uint8_t Bidi::level_ltr() {
  auto result = diplomat::capi::icu4x_Bidi_level_ltr_mv1();
  return result;
}

inline const diplomat::capi::Bidi* Bidi::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Bidi*>(this);
}

inline diplomat::capi::Bidi* Bidi::AsFFI() {
  return reinterpret_cast<diplomat::capi::Bidi*>(this);
}

inline const Bidi* Bidi::FromFFI(const diplomat::capi::Bidi* ptr) {
  return reinterpret_cast<const Bidi*>(ptr);
}

inline Bidi* Bidi::FromFFI(diplomat::capi::Bidi* ptr) {
  return reinterpret_cast<Bidi*>(ptr);
}

inline void Bidi::operator delete(void* ptr) {
  diplomat::capi::icu4x_Bidi_destroy_mv1(reinterpret_cast<diplomat::capi::Bidi*>(ptr));
}


#endif // Bidi_HPP

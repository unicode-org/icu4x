#ifndef icu4x_SentenceBreakOptionsV1_HPP
#define icu4x_SentenceBreakOptionsV1_HPP

#include "SentenceBreakOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "Locale.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::SentenceBreakOptionsV1* icu4x_SentenceBreakOptionsV1_create_mv1(const icu4x::capi::Locale* locale);
    
    
    void icu4x_SentenceBreakOptionsV1_destroy_mv1(SentenceBreakOptionsV1* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::SentenceBreakOptionsV1> icu4x::SentenceBreakOptionsV1::create(const icu4x::Locale& locale) {
  auto result = icu4x::capi::icu4x_SentenceBreakOptionsV1_create_mv1(locale.AsFFI());
  return std::unique_ptr<icu4x::SentenceBreakOptionsV1>(icu4x::SentenceBreakOptionsV1::FromFFI(result));
}

inline const icu4x::capi::SentenceBreakOptionsV1* icu4x::SentenceBreakOptionsV1::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::SentenceBreakOptionsV1*>(this);
}

inline icu4x::capi::SentenceBreakOptionsV1* icu4x::SentenceBreakOptionsV1::AsFFI() {
  return reinterpret_cast<icu4x::capi::SentenceBreakOptionsV1*>(this);
}

inline const icu4x::SentenceBreakOptionsV1* icu4x::SentenceBreakOptionsV1::FromFFI(const icu4x::capi::SentenceBreakOptionsV1* ptr) {
  return reinterpret_cast<const icu4x::SentenceBreakOptionsV1*>(ptr);
}

inline icu4x::SentenceBreakOptionsV1* icu4x::SentenceBreakOptionsV1::FromFFI(icu4x::capi::SentenceBreakOptionsV1* ptr) {
  return reinterpret_cast<icu4x::SentenceBreakOptionsV1*>(ptr);
}

inline void icu4x::SentenceBreakOptionsV1::operator delete(void* ptr) {
  icu4x::capi::icu4x_SentenceBreakOptionsV1_destroy_mv1(reinterpret_cast<icu4x::capi::SentenceBreakOptionsV1*>(ptr));
}


#endif // icu4x_SentenceBreakOptionsV1_HPP

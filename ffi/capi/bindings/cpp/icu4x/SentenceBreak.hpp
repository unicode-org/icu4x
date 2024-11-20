#ifndef icu4x_SentenceBreak_HPP
#define icu4x_SentenceBreak_HPP

#include "SentenceBreak.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    uint8_t icu4x_SentenceBreak_to_integer_mv1(icu4x::capi::SentenceBreak self);
    
    typedef struct icu4x_SentenceBreak_from_integer_mv1_result {union {icu4x::capi::SentenceBreak ok; }; bool is_ok;} icu4x_SentenceBreak_from_integer_mv1_result;
    icu4x_SentenceBreak_from_integer_mv1_result icu4x_SentenceBreak_from_integer_mv1(uint8_t other);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::SentenceBreak icu4x::SentenceBreak::AsFFI() const {
  return static_cast<icu4x::capi::SentenceBreak>(value);
}

inline icu4x::SentenceBreak icu4x::SentenceBreak::FromFFI(icu4x::capi::SentenceBreak c_enum) {
  switch (c_enum) {
    case icu4x::capi::SentenceBreak_Other:
    case icu4x::capi::SentenceBreak_ATerm:
    case icu4x::capi::SentenceBreak_Close:
    case icu4x::capi::SentenceBreak_Format:
    case icu4x::capi::SentenceBreak_Lower:
    case icu4x::capi::SentenceBreak_Numeric:
    case icu4x::capi::SentenceBreak_OLetter:
    case icu4x::capi::SentenceBreak_Sep:
    case icu4x::capi::SentenceBreak_Sp:
    case icu4x::capi::SentenceBreak_STerm:
    case icu4x::capi::SentenceBreak_Upper:
    case icu4x::capi::SentenceBreak_CR:
    case icu4x::capi::SentenceBreak_Extend:
    case icu4x::capi::SentenceBreak_LF:
    case icu4x::capi::SentenceBreak_SContinue:
      return static_cast<icu4x::SentenceBreak::Value>(c_enum);
    default:
      abort();
  }
}

inline uint8_t icu4x::SentenceBreak::to_integer() {
  auto result = icu4x::capi::icu4x_SentenceBreak_to_integer_mv1(this->AsFFI());
  return result;
}

inline std::optional<icu4x::SentenceBreak> icu4x::SentenceBreak::from_integer(uint8_t other) {
  auto result = icu4x::capi::icu4x_SentenceBreak_from_integer_mv1(other);
  return result.is_ok ? std::optional<icu4x::SentenceBreak>(icu4x::SentenceBreak::FromFFI(result.ok)) : std::nullopt;
}
#endif // icu4x_SentenceBreak_HPP
